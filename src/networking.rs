//! Network synchronization for action states.
//!
//! This module provides types for efficiently synchronizing action state
//! changes across a network, useful for multiplayer games.
//!
//! # Example
//!
//! ```rust,no_run
//! use bevy_archie::networking::{ActionDiff, ActionDiffBuffer};
//!
//! // Create a diff buffer for tracking changes
//! let mut buffer = ActionDiffBuffer::<u32>::new();
//!
//! // Record a button press
//! buffer.record_press(1); // Action ID 1 was pressed
//!
//! // Get diffs to send over network
//! let diffs = buffer.drain_diffs();
//! ```

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// A single action state change for network transmission.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ActionDiff<A> {
    /// An action was pressed/started.
    Pressed {
        /// The action that was pressed
        action: A,
        /// Timestamp (frame number or time)
        timestamp: u64,
    },
    /// An action was released/ended.
    Released {
        /// The action that was released
        action: A,
        /// Timestamp (frame number or time)
        timestamp: u64,
    },
    /// An action's axis value changed.
    AxisChanged {
        /// The action with changed axis
        action: A,
        /// The new axis value
        value: f32,
        /// Timestamp
        timestamp: u64,
    },
    /// An action's dual axis value changed.
    DualAxisChanged {
        /// The action with changed axis
        action: A,
        /// The new X axis value
        x: f32,
        /// The new Y axis value
        y: f32,
        /// Timestamp
        timestamp: u64,
    },
}

impl<A: Clone> ActionDiff<A> {
    /// Get the action this diff applies to.
    #[must_use]
    pub fn action(&self) -> A {
        match self {
            Self::Pressed { action, .. }
            | Self::Released { action, .. }
            | Self::AxisChanged { action, .. }
            | Self::DualAxisChanged { action, .. } => action.clone(),
        }
    }

    /// Get the timestamp of this diff.
    #[must_use]
    pub fn timestamp(&self) -> u64 {
        match self {
            Self::Pressed { timestamp, .. }
            | Self::Released { timestamp, .. }
            | Self::AxisChanged { timestamp, .. }
            | Self::DualAxisChanged { timestamp, .. } => *timestamp,
        }
    }
}

/// A buffer for collecting action diffs to send over the network.
#[derive(Debug, Clone, Default)]
pub struct ActionDiffBuffer<A> {
    /// Queued diffs waiting to be sent
    diffs: VecDeque<ActionDiff<A>>,
    /// Current timestamp (usually frame number)
    current_timestamp: u64,
    /// Maximum diffs to buffer before dropping old ones
    max_buffer_size: usize,
}

impl<A: Clone + PartialEq> ActionDiffBuffer<A> {
    /// Create a new diff buffer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            diffs: VecDeque::new(),
            current_timestamp: 0,
            max_buffer_size: 256,
        }
    }

    /// Create a buffer with a custom max size.
    #[must_use]
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            max_buffer_size: max_size,
            ..Self::new()
        }
    }

    /// Set the current timestamp (call each frame).
    pub fn set_timestamp(&mut self, timestamp: u64) {
        self.current_timestamp = timestamp;
    }

    /// Increment the timestamp.
    pub fn tick(&mut self) {
        self.current_timestamp += 1;
    }

    /// Record a button press.
    pub fn record_press(&mut self, action: A) {
        self.push_diff(ActionDiff::Pressed {
            action,
            timestamp: self.current_timestamp,
        });
    }

    /// Record a button release.
    pub fn record_release(&mut self, action: A) {
        self.push_diff(ActionDiff::Released {
            action,
            timestamp: self.current_timestamp,
        });
    }

    /// Record an axis value change.
    pub fn record_axis(&mut self, action: A, value: f32) {
        self.push_diff(ActionDiff::AxisChanged {
            action,
            value,
            timestamp: self.current_timestamp,
        });
    }

    /// Record a dual axis value change.
    pub fn record_dual_axis(&mut self, action: A, x: f32, y: f32) {
        self.push_diff(ActionDiff::DualAxisChanged {
            action,
            x,
            y,
            timestamp: self.current_timestamp,
        });
    }

    /// Push a diff to the buffer.
    fn push_diff(&mut self, diff: ActionDiff<A>) {
        // Enforce max buffer size
        if self.diffs.len() >= self.max_buffer_size {
            self.diffs.pop_front();
        }
        self.diffs.push_back(diff);
    }

    /// Get all pending diffs without removing them.
    #[must_use]
    pub fn peek_diffs(&self) -> &VecDeque<ActionDiff<A>> {
        &self.diffs
    }

    /// Drain all pending diffs.
    pub fn drain_diffs(&mut self) -> Vec<ActionDiff<A>> {
        self.diffs.drain(..).collect()
    }

    /// Get the number of pending diffs.
    #[must_use]
    pub fn len(&self) -> usize {
        self.diffs.len()
    }

    /// Check if there are no pending diffs.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.diffs.is_empty()
    }

    /// Clear all pending diffs.
    pub fn clear(&mut self) {
        self.diffs.clear();
    }
}

/// Component for entities that need networked input.
#[derive(Component, Debug, Default)]
pub struct NetworkedInput<A> {
    /// Buffer for outgoing diffs
    pub outgoing: ActionDiffBuffer<A>,
    /// Buffer for incoming diffs (to be applied)
    pub incoming: VecDeque<ActionDiff<A>>,
}

impl<A: Clone + PartialEq> NetworkedInput<A> {
    /// Create a new networked input component.
    #[must_use]
    pub fn new() -> Self {
        Self {
            outgoing: ActionDiffBuffer::new(),
            incoming: VecDeque::new(),
        }
    }

    /// Queue incoming diffs to be applied.
    pub fn receive_diffs(&mut self, diffs: impl IntoIterator<Item = ActionDiff<A>>) {
        self.incoming.extend(diffs);
    }

    /// Get the next incoming diff to apply.
    pub fn next_incoming(&mut self) -> Option<ActionDiff<A>> {
        self.incoming.pop_front()
    }

    /// Check if there are incoming diffs.
    #[must_use]
    pub fn has_incoming(&self) -> bool {
        !self.incoming.is_empty()
    }
}

/// Serialize diffs for network transmission.
///
/// Returns bytes that can be sent over the network.
///
/// # Errors
///
/// Returns an error if serialization fails.
pub fn serialize_diffs<A: Serialize>(
    diffs: &[ActionDiff<A>],
) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(diffs)
}

/// Deserialize diffs received from network.
///
/// # Errors
///
/// Returns an error if deserialization fails.
pub fn deserialize_diffs<A: for<'de> Deserialize<'de>>(
    bytes: &[u8],
) -> Result<Vec<ActionDiff<A>>, serde_json::Error> {
    serde_json::from_slice(bytes)
}

/// A snapshot of action states for full state sync.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionStateSnapshot<A> {
    /// Currently pressed actions
    pub pressed: Vec<A>,
    /// Current axis values
    pub axes: Vec<(A, f32)>,
    /// Current dual axis values
    pub dual_axes: Vec<(A, f32, f32)>,
    /// Snapshot timestamp
    pub timestamp: u64,
}

impl<A: Clone + PartialEq> ActionStateSnapshot<A> {
    /// Create an empty snapshot.
    #[must_use]
    pub fn new(timestamp: u64) -> Self {
        Self {
            pressed: Vec::new(),
            axes: Vec::new(),
            dual_axes: Vec::new(),
            timestamp,
        }
    }

    /// Add a pressed action to the snapshot.
    pub fn add_pressed(&mut self, action: A) {
        if !self.pressed.contains(&action) {
            self.pressed.push(action);
        }
    }

    /// Add an axis value to the snapshot.
    pub fn add_axis(&mut self, action: A, value: f32) {
        self.axes.push((action, value));
    }

    /// Add a dual axis value to the snapshot.
    pub fn add_dual_axis(&mut self, action: A, x: f32, y: f32) {
        self.dual_axes.push((action, x, y));
    }
}

/// Configuration for network input synchronization.
#[derive(Resource, Debug, Clone)]
pub struct NetworkInputConfig {
    /// How often to send full snapshots (in frames)
    pub snapshot_interval: u32,
    /// Whether to use delta compression
    pub delta_compression: bool,
    /// Maximum age of diffs to accept (for lag compensation)
    pub max_diff_age: u64,
    /// Whether to interpolate between received states
    pub interpolation: bool,
}

impl Default for NetworkInputConfig {
    fn default() -> Self {
        Self {
            snapshot_interval: 60, // Once per second at 60fps
            delta_compression: true,
            max_diff_age: 30, // Half second of lag tolerance
            interpolation: true,
        }
    }
}

/// Plugin for network input synchronization.
pub struct NetworkInputPlugin<A: Clone + PartialEq + Send + Sync + 'static> {
    _marker: std::marker::PhantomData<A>,
}

impl<A: Clone + PartialEq + Send + Sync + 'static> Default for NetworkInputPlugin<A> {
    fn default() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<A: Clone + PartialEq + Send + Sync + 'static> Plugin for NetworkInputPlugin<A> {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetworkInputConfig>();
    }
}

/// Helper trait for creating diffs from state changes.
pub trait ActionDiffExt<A: Clone + PartialEq> {
    /// Compare two states and generate diffs.
    fn diff_states(old_pressed: &[A], new_pressed: &[A], timestamp: u64) -> Vec<ActionDiff<A>>;
}

impl<A: Clone + PartialEq> ActionDiffExt<A> for ActionDiff<A> {
    fn diff_states(old_pressed: &[A], new_pressed: &[A], timestamp: u64) -> Vec<ActionDiff<A>> {
        let mut diffs = Vec::new();

        // Find newly pressed actions
        for action in new_pressed {
            if !old_pressed.contains(action) {
                diffs.push(ActionDiff::Pressed {
                    action: action.clone(),
                    timestamp,
                });
            }
        }

        // Find newly released actions
        for action in old_pressed {
            if !new_pressed.contains(action) {
                diffs.push(ActionDiff::Released {
                    action: action.clone(),
                    timestamp,
                });
            }
        }

        diffs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_diff_buffer() {
        let mut buffer = ActionDiffBuffer::<u32>::new();

        buffer.tick();
        buffer.record_press(1);
        buffer.tick();
        buffer.record_release(1);

        let diffs = buffer.drain_diffs();
        assert_eq!(diffs.len(), 2);

        assert!(matches!(
            diffs[0],
            ActionDiff::Pressed {
                action: 1,
                timestamp: 1
            }
        ));
        assert!(matches!(
            diffs[1],
            ActionDiff::Released {
                action: 1,
                timestamp: 2
            }
        ));
    }

    #[test]
    fn test_diff_states() {
        let old = vec![1, 2, 3];
        let new = vec![2, 3, 4];

        let diffs = ActionDiff::diff_states(&old, &new, 100);

        // Should have: Released(1), Pressed(4)
        assert_eq!(diffs.len(), 2);

        let has_release = diffs
            .iter()
            .any(|d| matches!(d, ActionDiff::Released { action: 1, .. }));
        let has_press = diffs
            .iter()
            .any(|d| matches!(d, ActionDiff::Pressed { action: 4, .. }));

        assert!(has_release);
        assert!(has_press);
    }

    #[test]
    fn test_serialization() {
        let diffs = vec![
            ActionDiff::Pressed::<u32> {
                action: 1,
                timestamp: 100,
            },
            ActionDiff::Released::<u32> {
                action: 2,
                timestamp: 101,
            },
        ];

        let bytes = serialize_diffs(&diffs).unwrap();
        let restored: Vec<ActionDiff<u32>> = deserialize_diffs(&bytes).unwrap();

        assert_eq!(diffs, restored);
    }

    #[test]
    fn test_buffer_max_size() {
        let mut buffer = ActionDiffBuffer::<u32>::with_max_size(3);

        buffer.record_press(1);
        buffer.record_press(2);
        buffer.record_press(3);
        buffer.record_press(4); // Should drop oldest

        assert_eq!(buffer.len(), 3);
        let diffs = buffer.drain_diffs();
        assert!(diffs.iter().any(|d| d.action() == 4));
        assert!(!diffs.iter().any(|d| d.action() == 1)); // 1 was dropped
    }

    #[test]
    fn test_action_snapshot() {
        let mut snapshot = ActionStateSnapshot::<u32>::new(100);

        snapshot.add_pressed(1);
        snapshot.add_pressed(2);
        snapshot.add_axis(3, 0.5);
        snapshot.add_dual_axis(4, 0.3, -0.7);

        assert_eq!(snapshot.pressed.len(), 2);
        assert_eq!(snapshot.axes.len(), 1);
        assert_eq!(snapshot.dual_axes.len(), 1);
    }
}
