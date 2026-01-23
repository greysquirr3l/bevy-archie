//! State machine integration for input-driven state transitions.
//!
//! This module provides utilities for connecting input actions to
//! Bevy state transitions, useful for character controllers and menus.
//!
//! # Example
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use bevy_archie::state_machine::InputStateMachine;
//!
//! #[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
//! enum PlayerState { #[default] Idle, Running, Jumping }
//!
//! fn setup_state_machine(mut commands: Commands) {
//!     let mut machine = InputStateMachine::<PlayerState, u32>::new();
//!     
//!     // From Idle, pressing Jump (action 1) goes to Jumping
//!     machine.add_transition(PlayerState::Idle, 1, PlayerState::Jumping);
//!     
//!     // From Idle, holding Move (action 2) goes to Running  
//!     machine.add_transition(PlayerState::Idle, 2, PlayerState::Running);
//!     
//!     commands.insert_resource(machine);
//! }
//! ```

use bevy::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;

/// A state machine that responds to input actions.
#[derive(Resource, Debug)]
pub struct InputStateMachine<S, A> {
    /// Transitions: (`from_state`, action) -> (`to_state`, condition)
    transitions: HashMap<(S, A), TransitionConfig<S>>,
    /// Default transitions that apply from any state
    global_transitions: HashMap<A, TransitionConfig<S>>,
}

impl<S: Clone + Eq + Hash, A: Clone + Eq + Hash> Default for InputStateMachine<S, A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: Clone + Eq + Hash, A: Clone + Eq + Hash> InputStateMachine<S, A> {
    /// Create a new state machine.
    #[must_use]
    pub fn new() -> Self {
        Self {
            transitions: HashMap::new(),
            global_transitions: HashMap::new(),
        }
    }

    /// Add a transition from one state to another when an action is pressed.
    pub fn add_transition(&mut self, from: S, action: A, to: S) -> &mut Self {
        self.transitions.insert(
            (from, action),
            TransitionConfig {
                target: to,
                trigger: TriggerType::JustPressed,
                guard: TransitionGuard::Always,
            },
        );
        self
    }

    /// Add a transition with a specific trigger type.
    pub fn add_transition_with_trigger(
        &mut self,
        from: S,
        action: A,
        to: S,
        trigger: TriggerType,
    ) -> &mut Self {
        self.transitions.insert(
            (from, action),
            TransitionConfig {
                target: to,
                trigger,
                guard: TransitionGuard::Always,
            },
        );
        self
    }

    /// Add a global transition (from any state).
    pub fn add_global_transition(&mut self, action: A, to: S) -> &mut Self {
        self.global_transitions.insert(
            action,
            TransitionConfig {
                target: to,
                trigger: TriggerType::JustPressed,
                guard: TransitionGuard::Always,
            },
        );
        self
    }

    /// Get the target state for a given input in the current state.
    #[must_use]
    pub fn get_transition(&self, current: &S, action: &A, trigger: TriggerType) -> Option<&S> {
        // Check specific transitions first
        if let Some(config) = self.transitions.get(&(current.clone(), action.clone()))
            && config.trigger == trigger
        {
            return Some(&config.target);
        }

        // Fall back to global transitions
        if let Some(config) = self.global_transitions.get(action)
            && config.trigger == trigger
        {
            return Some(&config.target);
        }

        None
    }
}

/// Configuration for a state transition.
#[derive(Debug, Clone)]
pub struct TransitionConfig<S> {
    /// The target state to transition to
    pub target: S,
    /// What type of input triggers this transition
    pub trigger: TriggerType,
    /// Optional guard condition
    pub guard: TransitionGuard,
}

/// What type of input event triggers a transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriggerType {
    /// Trigger when action is just pressed
    JustPressed,
    /// Trigger when action is released
    JustReleased,
    /// Trigger while action is held
    Pressed,
    /// Trigger while action is not pressed
    Released,
}

/// Guard condition for a transition.
#[derive(Debug, Clone)]
pub enum TransitionGuard {
    /// Always allow the transition
    Always,
    /// Transition requires a custom condition to be true
    Custom(String),
    /// Transition requires multiple conditions
    All(Vec<TransitionGuard>),
    /// Transition requires any of multiple conditions
    Any(Vec<TransitionGuard>),
}

impl TransitionGuard {
    /// Create a guard that always passes.
    #[must_use]
    pub fn always() -> Self {
        Self::Always
    }

    /// Create a custom guard condition.
    #[must_use]
    pub fn custom(name: impl Into<String>) -> Self {
        Self::Custom(name.into())
    }
}

/// A component for entities with input-driven state.
#[derive(Component, Debug)]
pub struct InputDrivenState<S, A> {
    /// Reference to the state machine
    _machine: std::marker::PhantomData<(S, A)>,
    /// Last processed input frame
    #[allow(dead_code)]
    last_frame: u64,
}

impl<S, A> Default for InputDrivenState<S, A> {
    fn default() -> Self {
        Self {
            _machine: std::marker::PhantomData,
            last_frame: 0,
        }
    }
}

/// Event emitted when a state transition occurs.
#[derive(Event, Debug, Clone)]
pub struct StateTransitionEvent<S> {
    /// The entity that transitioned (if any)
    pub entity: Option<Entity>,
    /// The state transitioned from
    pub from: S,
    /// The state transitioned to
    pub to: S,
    /// What triggered the transition
    pub trigger: TriggerType,
}

/// Builder for creating state machines fluently.
#[derive(Debug)]
pub struct StateMachineBuilder<S, A> {
    machine: InputStateMachine<S, A>,
}

impl<S: Clone + Eq + Hash, A: Clone + Eq + Hash> StateMachineBuilder<S, A> {
    /// Create a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            machine: InputStateMachine::new(),
        }
    }

    /// Add a transition.
    #[must_use]
    pub fn on(mut self, from: S, action: A, to: S) -> Self {
        self.machine.add_transition(from, action, to);
        self
    }

    /// Add a transition with a specific trigger.
    #[must_use]
    pub fn on_trigger(mut self, from: S, action: A, to: S, trigger: TriggerType) -> Self {
        self.machine
            .add_transition_with_trigger(from, action, to, trigger);
        self
    }

    /// Add a global transition.
    #[must_use]
    pub fn on_any(mut self, action: A, to: S) -> Self {
        self.machine.add_global_transition(action, to);
        self
    }

    /// Build the state machine.
    #[must_use]
    pub fn build(self) -> InputStateMachine<S, A> {
        self.machine
    }
}

impl<S: Clone + Eq + Hash, A: Clone + Eq + Hash> Default for StateMachineBuilder<S, A> {
    fn default() -> Self {
        Self::new()
    }
}

/// A simple state graph for visualization/debugging.
#[derive(Debug, Default)]
pub struct StateGraph<S> {
    /// All states in the graph
    pub states: Vec<S>,
    /// Edges: (`from_index`, `to_index`, label)
    pub edges: Vec<(usize, usize, String)>,
}

impl<S: Clone + Eq + Hash> StateGraph<S> {
    /// Build a graph from a state machine.
    #[must_use]
    pub fn from_machine<A: Clone + Eq + Hash + std::fmt::Debug>(
        machine: &InputStateMachine<S, A>,
    ) -> Self {
        let mut graph = Self {
            states: Vec::new(),
            edges: Vec::new(),
        };

        // Collect all states
        let mut state_indices: HashMap<S, usize> = HashMap::new();

        for ((from, _action), config) in &machine.transitions {
            if !state_indices.contains_key(from) {
                state_indices.insert(from.clone(), graph.states.len());
                graph.states.push(from.clone());
            }
            if !state_indices.contains_key(&config.target) {
                state_indices.insert(config.target.clone(), graph.states.len());
                graph.states.push(config.target.clone());
            }
        }

        // Build edges
        for ((from, action), config) in &machine.transitions {
            let from_idx = state_indices[from];
            let to_idx = state_indices[&config.target];
            graph.edges.push((from_idx, to_idx, format!("{action:?}")));
        }

        graph
    }
}

/// Timer-based state that auto-transitions after a duration.
#[derive(Component, Debug)]
pub struct TimedState<S> {
    /// Current state
    pub current: S,
    /// Time remaining before auto-transition
    pub timer: f32,
    /// State to transition to when timer expires
    pub next: Option<S>,
}

impl<S: Clone> TimedState<S> {
    /// Create a new timed state.
    #[must_use]
    pub fn new(initial: S) -> Self {
        Self {
            current: initial,
            timer: 0.0,
            next: None,
        }
    }

    /// Set up an auto-transition after a duration.
    pub fn transition_after(&mut self, duration: f32, to: S) {
        self.timer = duration;
        self.next = Some(to);
    }

    /// Update the timer. Returns the new state if a transition occurred.
    pub fn update(&mut self, delta: f32) -> Option<S> {
        if self.next.is_some() {
            self.timer -= delta;
            if self.timer <= 0.0
                && let Some(next) = self.next.take()
            {
                self.current = next.clone();
                return Some(next);
            }
        }
        None
    }
}

/// System set for state machine processing.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum StateMachineSet {
    /// Process input and determine transitions
    ProcessInput,
    /// Execute state transitions
    Transition,
    /// React to state changes
    PostTransition,
}

/// Plugin for state machine integration.
pub struct StateMachinePlugin;

impl Plugin for StateMachinePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                StateMachineSet::ProcessInput,
                StateMachineSet::Transition,
                StateMachineSet::PostTransition,
            )
                .chain(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
    enum TestState {
        #[default]
        Idle,
        Running,
        Jumping,
    }

    #[derive(Clone, Eq, PartialEq, Debug, Hash)]
    enum TestAction {
        Move,
        Jump,
    }

    #[test]
    fn test_state_machine_basic() {
        let mut machine = InputStateMachine::<TestState, TestAction>::new();

        machine.add_transition(TestState::Idle, TestAction::Jump, TestState::Jumping);
        machine.add_transition(TestState::Idle, TestAction::Move, TestState::Running);

        let target = machine.get_transition(
            &TestState::Idle,
            &TestAction::Jump,
            TriggerType::JustPressed,
        );
        assert_eq!(target, Some(&TestState::Jumping));

        let target = machine.get_transition(
            &TestState::Running,
            &TestAction::Jump,
            TriggerType::JustPressed,
        );
        assert_eq!(target, None); // No transition defined from Running
    }

    #[test]
    fn test_global_transition() {
        let mut machine = InputStateMachine::<TestState, TestAction>::new();

        // Global transition: pressing Jump from any state goes to Jumping
        machine.add_global_transition(TestAction::Jump, TestState::Jumping);

        let target = machine.get_transition(
            &TestState::Running, // Even from Running
            &TestAction::Jump,
            TriggerType::JustPressed,
        );
        assert_eq!(target, Some(&TestState::Jumping));
    }

    #[test]
    fn test_builder() {
        let machine = StateMachineBuilder::<TestState, TestAction>::new()
            .on(TestState::Idle, TestAction::Jump, TestState::Jumping)
            .on(TestState::Idle, TestAction::Move, TestState::Running)
            .on_any(TestAction::Jump, TestState::Jumping)
            .build();

        assert!(machine.transitions.len() >= 2);
    }

    #[test]
    fn test_timed_state() {
        let mut timed = TimedState::new(TestState::Jumping);
        timed.transition_after(0.5, TestState::Idle);

        // Not enough time
        assert!(timed.update(0.3).is_none());

        // Timer expired
        let result = timed.update(0.3);
        assert_eq!(result, Some(TestState::Idle));
        assert_eq!(timed.current, TestState::Idle);
    }

    #[test]
    fn test_state_graph() {
        let machine = StateMachineBuilder::new()
            .on(TestState::Idle, TestAction::Jump, TestState::Jumping)
            .on(TestState::Jumping, TestAction::Move, TestState::Running)
            .build();

        let graph = StateGraph::from_machine(&machine);

        // Should have 3 states (Idle, Jumping, Running)
        assert_eq!(graph.states.len(), 3);
        // Should have 2 edges
        assert_eq!(graph.edges.len(), 2);
    }
}
