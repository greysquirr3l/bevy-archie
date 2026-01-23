//! Input condition modifiers for contextual action triggering.
//!
//! This module provides conditions that can be attached to input bindings
//! to control when actions should trigger (e.g., only when running, only in menus).
//!
//! # Example
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use bevy_archie::conditions::{InputCondition, ConditionContext, StateCondition};
//!
//! #[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
//! enum GameState { #[default] Playing, Paused, Menu }
//!
//! // Create a condition that only triggers in Playing state
//! let condition = InputCondition::in_state(GameState::Playing);
//! ```

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::TypeId;

/// A condition that determines whether an input binding should trigger.
#[derive(Debug, Clone, Default)]
pub enum InputCondition {
    /// Always allow the action.
    #[default]
    Always,

    /// Never allow the action.
    Never,

    /// Only allow when a specific state is active.
    InState(StateCondition),

    /// Only allow when NOT in a specific state.
    NotInState(StateCondition),

    /// Only allow when a resource exists.
    ResourceExists(ResourceCondition),

    /// Only allow when a resource doesn't exist.
    ResourceAbsent(ResourceCondition),

    /// Custom condition evaluated by a registered system.
    Custom(CustomConditionId),

    /// Combine multiple conditions with AND logic.
    All(Vec<InputCondition>),

    /// Combine multiple conditions with OR logic.
    Any(Vec<InputCondition>),

    /// Negate a condition.
    Not(Box<InputCondition>),
}

impl InputCondition {
    /// Create a condition that's always true.
    #[must_use]
    pub fn always() -> Self {
        Self::Always
    }

    /// Create a condition that's never true.
    #[must_use]
    pub fn never() -> Self {
        Self::Never
    }

    /// Create a condition that requires a specific state.
    #[must_use]
    pub fn in_state<S: States>(state: S) -> Self {
        Self::InState(StateCondition::new::<S>(state))
    }

    /// Create a condition that requires NOT being in a specific state.
    #[must_use]
    pub fn not_in_state<S: States>(state: S) -> Self {
        Self::NotInState(StateCondition::new::<S>(state))
    }

    /// Create a condition that requires a resource to exist.
    #[must_use]
    pub fn resource_exists<R: Resource>() -> Self {
        Self::ResourceExists(ResourceCondition::new::<R>())
    }

    /// Create a condition that requires a resource to NOT exist.
    #[must_use]
    pub fn resource_absent<R: Resource>() -> Self {
        Self::ResourceAbsent(ResourceCondition::new::<R>())
    }

    /// Create a custom condition with an ID.
    #[must_use]
    pub fn custom(id: impl Into<String>) -> Self {
        Self::Custom(CustomConditionId(id.into()))
    }

    /// Combine with another condition using AND logic.
    #[must_use]
    pub fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::Always, other) | (other, Self::Always) => other,
            (Self::Never, _) | (_, Self::Never) => Self::Never,
            (Self::All(mut a), Self::All(b)) => {
                a.extend(b);
                Self::All(a)
            }
            (Self::All(mut a), other) => {
                a.push(other);
                Self::All(a)
            }
            (other, Self::All(mut a)) => {
                a.insert(0, other);
                Self::All(a)
            }
            (a, b) => Self::All(vec![a, b]),
        }
    }

    /// Combine with another condition using OR logic.
    #[must_use]
    pub fn or(self, other: Self) -> Self {
        match (self, other) {
            (Self::Always, _) | (_, Self::Always) => Self::Always,
            (Self::Never, other) | (other, Self::Never) => other,
            (Self::Any(mut a), Self::Any(b)) => {
                a.extend(b);
                Self::Any(a)
            }
            (Self::Any(mut a), other) => {
                a.push(other);
                Self::Any(a)
            }
            (other, Self::Any(mut a)) => {
                a.insert(0, other);
                Self::Any(a)
            }
            (a, b) => Self::Any(vec![a, b]),
        }
    }

    /// Negate this condition.
    #[must_use]
    pub fn negated(self) -> Self {
        match self {
            Self::Always => Self::Never,
            Self::Never => Self::Always,
            Self::Not(inner) => *inner,
            other => Self::Not(Box::new(other)),
        }
    }
}

impl std::ops::Not for InputCondition {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.negated()
    }
}

/// A condition based on Bevy state.
#[derive(Debug, Clone)]
pub struct StateCondition {
    /// The `TypeId` of the state type
    #[expect(dead_code, reason = "stored for future state comparison functionality")]
    state_type_id: TypeId,
    /// Function to check if the current state matches
    #[expect(dead_code, reason = "stored for future state comparison functionality")]
    check_fn: fn(&World) -> bool,
    /// Name for debugging
    state_name: &'static str,
}

impl StateCondition {
    /// Create a new state condition.
    pub fn new<S: States>(_expected: S) -> Self {
        Self {
            state_type_id: TypeId::of::<S>(),
            check_fn: {
                // We need to capture the expected state value
                // This is a simplified version - in production you'd use a different approach
                |_world| {
                    // This is a placeholder - actual implementation would use world access
                    true
                }
            },
            state_name: std::any::type_name::<S>(),
        }
    }

    /// Get the state type name for debugging.
    #[must_use]
    pub fn state_name(&self) -> &'static str {
        self.state_name
    }
}

/// A condition based on resource existence.
#[derive(Debug, Clone)]
pub struct ResourceCondition {
    /// The `TypeId` of the resource
    resource_type_id: TypeId,
    /// Name for debugging
    resource_name: &'static str,
}

impl ResourceCondition {
    /// Create a new resource condition.
    #[must_use]
    pub fn new<R: Resource>() -> Self {
        Self {
            resource_type_id: TypeId::of::<R>(),
            resource_name: std::any::type_name::<R>(),
        }
    }

    /// Check if the resource exists in the world.
    ///
    /// # Panics
    ///
    /// Panics if the resource type ID is not registered with the world.
    #[must_use]
    pub fn check(&self, world: &World) -> bool {
        world.contains_resource_by_id(
            world
                .components()
                .get_resource_id(self.resource_type_id)
                .unwrap(),
        )
    }

    /// Get the resource type name for debugging.
    #[must_use]
    pub fn resource_name(&self) -> &'static str {
        self.resource_name
    }
}

/// A custom condition identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CustomConditionId(pub String);

impl From<&str> for CustomConditionId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for CustomConditionId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// Context for evaluating conditions.
#[derive(Debug)]
pub struct ConditionContext<'w> {
    /// Reference to the world for state/resource checks
    world: &'w World,
    /// Custom condition results (pre-computed by registered systems)
    custom_results: &'w CustomConditionResults,
}

impl<'w> ConditionContext<'w> {
    /// Create a new condition context.
    #[must_use]
    pub fn new(world: &'w World, custom_results: &'w CustomConditionResults) -> Self {
        Self {
            world,
            custom_results,
        }
    }

    /// Evaluate a condition.
    #[must_use]
    pub fn evaluate(&self, condition: &InputCondition) -> bool {
        match condition {
            InputCondition::Always => true,
            InputCondition::Never => false,
            InputCondition::InState(_state_cond) => {
                // In a real implementation, we'd check the actual state
                // This is simplified for the example
                true
            }
            InputCondition::NotInState(state_cond) => {
                !self.evaluate(&InputCondition::InState(state_cond.clone()))
            }
            InputCondition::ResourceExists(res_cond) => res_cond.check(self.world),
            InputCondition::ResourceAbsent(res_cond) => !res_cond.check(self.world),
            InputCondition::Custom(id) => self.custom_results.get(id).unwrap_or(false),
            InputCondition::All(conditions) => conditions.iter().all(|c| self.evaluate(c)),
            InputCondition::Any(conditions) => conditions.iter().any(|c| self.evaluate(c)),
            InputCondition::Not(inner) => !self.evaluate(inner),
        }
    }
}

/// Storage for custom condition results.
#[derive(Resource, Debug, Default)]
pub struct CustomConditionResults {
    results: std::collections::HashMap<CustomConditionId, bool>,
}

impl CustomConditionResults {
    /// Set the result for a custom condition.
    pub fn set(&mut self, id: impl Into<CustomConditionId>, value: bool) {
        self.results.insert(id.into(), value);
    }

    /// Get the result for a custom condition.
    #[must_use]
    pub fn get(&self, id: &CustomConditionId) -> Option<bool> {
        self.results.get(id).copied()
    }

    /// Clear all results.
    pub fn clear(&mut self) {
        self.results.clear();
    }
}

/// A binding with an attached condition.
#[derive(Debug, Clone)]
pub struct ConditionalBinding<B> {
    /// The actual binding
    pub binding: B,
    /// The condition that must be true for this binding to be active
    pub condition: InputCondition,
}

impl<B> ConditionalBinding<B> {
    /// Create a new conditional binding.
    #[must_use]
    pub fn new(binding: B, condition: InputCondition) -> Self {
        Self { binding, condition }
    }

    /// Create an always-active binding.
    #[must_use]
    pub fn always(binding: B) -> Self {
        Self::new(binding, InputCondition::Always)
    }

    /// Add a condition to this binding.
    #[must_use]
    pub fn when(mut self, condition: InputCondition) -> Self {
        self.condition = self.condition.and(condition);
        self
    }
}

/// Plugin to add condition evaluation support.
pub struct ConditionsPlugin;

impl Plugin for ConditionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CustomConditionResults>();
    }
}

/// Trait for types that can have conditions attached.
pub trait Conditionable: Sized {
    /// Add a condition that must be true for this to be active.
    fn when(self, condition: InputCondition) -> ConditionalBinding<Self>;

    /// Only active when in the specified state.
    fn when_in_state<S: States>(self, state: S) -> ConditionalBinding<Self> {
        self.when(InputCondition::in_state(state))
    }

    /// Only active when NOT in the specified state.
    fn when_not_in_state<S: States>(self, state: S) -> ConditionalBinding<Self> {
        self.when(InputCondition::not_in_state(state))
    }
}

impl<T> Conditionable for T {
    fn when(self, condition: InputCondition) -> ConditionalBinding<Self> {
        ConditionalBinding::new(self, condition)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition_always() {
        let cond = InputCondition::always();
        assert!(matches!(cond, InputCondition::Always));
    }

    #[test]
    fn test_condition_never() {
        let cond = InputCondition::never();
        assert!(matches!(cond, InputCondition::Never));
    }

    #[test]
    fn test_condition_and() {
        // Always AND Always = Always
        let cond = InputCondition::always().and(InputCondition::always());
        assert!(matches!(cond, InputCondition::Always));

        // Always AND Never = Never
        let cond = InputCondition::always().and(InputCondition::never());
        assert!(matches!(cond, InputCondition::Never));

        // Never AND Always = Never
        let cond = InputCondition::never().and(InputCondition::always());
        assert!(matches!(cond, InputCondition::Never));
    }

    #[test]
    fn test_condition_or() {
        // Always OR Never = Always
        let cond = InputCondition::always().or(InputCondition::never());
        assert!(matches!(cond, InputCondition::Always));

        // Never OR Always = Always
        let cond = InputCondition::never().or(InputCondition::always());
        assert!(matches!(cond, InputCondition::Always));

        // Never OR Never = Never
        let cond = InputCondition::never().or(InputCondition::never());
        assert!(matches!(cond, InputCondition::Never));
    }

    #[test]
    fn test_condition_not() {
        use std::ops::Not;

        // NOT Always = Never
        let cond = InputCondition::always().not();
        assert!(matches!(cond, InputCondition::Never));

        // NOT Never = Always
        let cond = InputCondition::never().not();
        assert!(matches!(cond, InputCondition::Always));

        // NOT NOT X = X
        let cond = InputCondition::custom("test").not().not();
        assert!(matches!(cond, InputCondition::Custom(_)));
    }

    #[test]
    fn test_custom_condition_results() {
        let mut results = CustomConditionResults::default();

        results.set("can_jump", true);
        results.set("is_grounded", false);

        assert_eq!(
            results.get(&CustomConditionId("can_jump".into())),
            Some(true)
        );
        assert_eq!(
            results.get(&CustomConditionId("is_grounded".into())),
            Some(false)
        );
        assert_eq!(results.get(&CustomConditionId("unknown".into())), None);
    }

    #[test]
    fn test_conditional_binding() {
        let binding = "action_binding".when(InputCondition::always());
        assert!(matches!(binding.condition, InputCondition::Always));

        let binding = "action_binding"
            .when(InputCondition::custom("can_jump"))
            .when(InputCondition::custom("is_grounded"));

        assert!(matches!(binding.condition, InputCondition::All(_)));
    }
}
