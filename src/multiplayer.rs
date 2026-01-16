//! Multiplayer input management.
//!
//! This module provides per-player input isolation, controller ownership,
//! and player assignment for local multiplayer games.

use bevy::prelude::*;
use std::collections::HashMap;

/// Player identifier (0-indexed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Reflect)]
pub struct PlayerId(pub u8);

impl PlayerId {
    /// Create a new player ID.
    #[must_use]
    pub const fn new(id: u8) -> Self {
        Self(id)
    }

    /// Get the ID value.
    #[must_use]
    pub const fn id(self) -> u8 {
        self.0
    }
}

/// Component marking an entity as belonging to a specific player.
#[derive(Debug, Clone, Copy, Component, Reflect)]
pub struct Player {
    /// Player identifier.
    pub id: PlayerId,
    /// Whether this player is active.
    pub active: bool,
}

impl Player {
    /// Create a new player.
    #[must_use]
    pub const fn new(id: u8) -> Self {
        Self {
            id: PlayerId(id),
            active: true,
        }
    }

    /// Create player 1.
    #[must_use]
    pub fn one() -> Self {
        Self::new(0)
    }

    /// Create player 2.
    #[must_use]
    pub fn two() -> Self {
        Self::new(1)
    }
}

/// Controller ownership - which player owns which gamepad.
#[derive(Debug, Clone, Resource)]
pub struct ControllerOwnership {
    /// Map of gamepad entities to player IDs.
    pub owners: HashMap<Entity, PlayerId>,
    /// Map of player IDs to gamepad entities.
    pub assignments: HashMap<PlayerId, Entity>,
    /// Whether to auto-assign new controllers.
    pub auto_assign: bool,
}

impl Default for ControllerOwnership {
    fn default() -> Self {
        Self {
            owners: HashMap::new(),
            assignments: HashMap::new(),
            auto_assign: true,
        }
    }
}

impl ControllerOwnership {
    /// Assign a gamepad to a player.
    pub fn assign(&mut self, gamepad: Entity, player: PlayerId) {
        // Remove previous assignment if exists
        if let Some(old_gamepad) = self.assignments.insert(player, gamepad) {
            self.owners.remove(&old_gamepad);
        }
        self.owners.insert(gamepad, player);
    }

    /// Unassign a gamepad.
    pub fn unassign_gamepad(&mut self, gamepad: Entity) {
        if let Some(player) = self.owners.remove(&gamepad) {
            self.assignments.remove(&player);
        }
    }

    /// Unassign a player.
    pub fn unassign_player(&mut self, player: PlayerId) {
        if let Some(gamepad) = self.assignments.remove(&player) {
            self.owners.remove(&gamepad);
        }
    }

    /// Get the player owning a gamepad.
    #[must_use]
    pub fn get_owner(&self, gamepad: Entity) -> Option<PlayerId> {
        self.owners.get(&gamepad).copied()
    }

    /// Get the gamepad assigned to a player.
    #[must_use]
    pub fn get_gamepad(&self, player: PlayerId) -> Option<Entity> {
        self.assignments.get(&player).copied()
    }

    /// Check if a gamepad is assigned.
    #[must_use]
    pub fn is_assigned(&self, gamepad: Entity) -> bool {
        self.owners.contains_key(&gamepad)
    }

    /// Get all unassigned gamepads.
    #[must_use]
    pub fn get_unassigned(&self, all_gamepads: &[Entity]) -> Vec<Entity> {
        all_gamepads
            .iter()
            .filter(|g| !self.is_assigned(**g))
            .copied()
            .collect()
    }
}

/// Event fired when a controller is assigned to a player.
#[derive(Debug, Clone, Message)]
pub struct ControllerAssigned {
    /// The gamepad entity.
    pub gamepad: Entity,
    /// The player it was assigned to.
    pub player: PlayerId,
}

/// Event fired when a controller is unassigned.
#[derive(Debug, Clone, Message)]
pub struct ControllerUnassigned {
    /// The gamepad entity.
    pub gamepad: Entity,
    /// The player it was assigned to.
    pub player: PlayerId,
}

/// Request to assign a controller to a player.
#[derive(Debug, Clone, Message)]
pub struct AssignControllerRequest {
    /// The gamepad to assign.
    pub gamepad: Entity,
    /// The player to assign to.
    pub player: PlayerId,
}

/// System to handle controller assignment requests.
pub fn handle_assignment_requests(
    mut requests: MessageReader<AssignControllerRequest>,
    mut ownership: ResMut<ControllerOwnership>,
    mut assigned_events: MessageWriter<ControllerAssigned>,
) {
    for request in requests.read() {
        ownership.assign(request.gamepad, request.player);
        assigned_events.write(ControllerAssigned {
            gamepad: request.gamepad,
            player: request.player,
        });
    }
}

/// System to auto-assign new gamepads to players.
pub fn auto_assign_controllers(
    mut ownership: ResMut<ControllerOwnership>,
    gamepads: Query<Entity, Added<Gamepad>>,
    mut assigned_events: MessageWriter<ControllerAssigned>,
) {
    if !ownership.auto_assign {
        return;
    }

    for gamepad in gamepads.iter() {
        if ownership.is_assigned(gamepad) {
            continue;
        }

        // Find next available player slot (0-3 for 4 players)
        for player_id in 0..4 {
            let player = PlayerId(player_id);
            if ownership.get_gamepad(player).is_none() {
                ownership.assign(gamepad, player);
                assigned_events.write(ControllerAssigned { gamepad, player });
                break;
            }
        }
    }
}

/// System to handle gamepad disconnections.
pub fn handle_controller_disconnections(
    mut ownership: ResMut<ControllerOwnership>,
    mut removed_gamepads: RemovedComponents<Gamepad>,
    mut unassigned_events: MessageWriter<ControllerUnassigned>,
) {
    for gamepad in removed_gamepads.read() {
        if let Some(player) = ownership.get_owner(gamepad) {
            ownership.unassign_gamepad(gamepad);
            unassigned_events.write(ControllerUnassigned { gamepad, player });
        }
    }
}

/// Plugin for registering multiplayer types.
pub(crate) fn register_multiplayer_types(app: &mut App) {
    app.register_type::<PlayerId>()
        .register_type::<Player>()
        .init_resource::<ControllerOwnership>()
        .add_message::<ControllerAssigned>()
        .add_message::<ControllerUnassigned>()
        .add_message::<AssignControllerRequest>();
}

/// Add multiplayer systems to the app.
pub(crate) fn add_multiplayer_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            handle_assignment_requests,
            auto_assign_controllers,
            handle_controller_disconnections,
        )
            .chain(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_id_new() {
        let id = PlayerId::new(3);
        assert_eq!(id.id(), 3);
    }

    #[test]
    fn test_player_new() {
        let player = Player::new(2);
        assert_eq!(player.id.id(), 2);
        assert!(player.active);
    }

    #[test]
    fn test_player_one_two() {
        let p1 = Player::one();
        let p2 = Player::two();

        assert_eq!(p1.id.id(), 0);
        assert_eq!(p2.id.id(), 1);
        assert!(p1.active);
        assert!(p2.active);
    }

    #[test]
    fn test_controller_ownership_default() {
        let ownership = ControllerOwnership::default();
        assert_eq!(ownership.owners.len(), 0);
        assert_eq!(ownership.assignments.len(), 0);
        assert!(ownership.auto_assign);
    }

    #[test]
    fn test_controller_ownership_assign() {
        let mut ownership = ControllerOwnership::default();
        let gamepad = Entity::from_bits(100);
        let player = PlayerId::new(0);

        ownership.assign(gamepad, player);

        assert_eq!(ownership.get_owner(gamepad), Some(player));
        assert_eq!(ownership.get_gamepad(player), Some(gamepad));
    }

    #[test]
    fn test_controller_ownership_reassign() {
        let mut ownership = ControllerOwnership::default();
        let gamepad1 = Entity::from_bits(100);
        let gamepad2 = Entity::from_bits(200);
        let player = PlayerId::new(0);

        ownership.assign(gamepad1, player);
        ownership.assign(gamepad2, player);

        // Player should now have gamepad2
        assert_eq!(ownership.get_gamepad(player), Some(gamepad2));
        // gamepad1 should no longer be assigned
        assert_eq!(ownership.get_owner(gamepad1), None);
    }

    #[test]
    fn test_controller_ownership_unassign_gamepad() {
        let mut ownership = ControllerOwnership::default();
        let gamepad = Entity::from_bits(100);
        let player = PlayerId::new(0);

        ownership.assign(gamepad, player);
        ownership.unassign_gamepad(gamepad);

        assert_eq!(ownership.get_owner(gamepad), None);
        assert_eq!(ownership.get_gamepad(player), None);
    }

    #[test]
    fn test_controller_ownership_unassign_player() {
        let mut ownership = ControllerOwnership::default();
        let gamepad = Entity::from_bits(100);
        let player = PlayerId::new(0);

        ownership.assign(gamepad, player);
        ownership.unassign_player(player);

        assert_eq!(ownership.get_owner(gamepad), None);
        assert_eq!(ownership.get_gamepad(player), None);
    }

    #[test]
    fn test_controller_ownership_get_methods() {
        let mut ownership = ControllerOwnership::default();
        let gamepad = Entity::from_bits(123);
        let player = PlayerId::new(1);

        ownership.assign(gamepad, player);

        assert_eq!(ownership.get_owner(gamepad), Some(player));
        assert_eq!(ownership.get_gamepad(player), Some(gamepad));

        // Test non-existent queries
        let other_gamepad = Entity::from_bits(999);
        let other_player = PlayerId::new(99);
        assert_eq!(ownership.get_owner(other_gamepad), None);
        assert_eq!(ownership.get_gamepad(other_player), None);
    }

    #[test]
    fn test_controller_assigned_event() {
        let gamepad = Entity::from_bits(50);
        let player = PlayerId::new(2);
        let event = ControllerAssigned { gamepad, player };

        assert_eq!(event.gamepad, gamepad);
        assert_eq!(event.player, player);
    }

    #[test]
    fn test_controller_unassigned_event() {
        let gamepad = Entity::from_bits(50);
        let player = PlayerId::new(2);
        let event = ControllerUnassigned { gamepad, player };

        assert_eq!(event.gamepad, gamepad);
        assert_eq!(event.player, player);
    }

    #[test]
    fn test_assign_controller_request() {
        let gamepad = Entity::from_bits(75);
        let player = PlayerId::new(3);
        let request = AssignControllerRequest { gamepad, player };

        assert_eq!(request.gamepad, gamepad);
        assert_eq!(request.player, player);
    }
}
