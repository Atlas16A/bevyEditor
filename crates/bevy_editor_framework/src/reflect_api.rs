use core::any::TypeId;

use bevy_ecs::{component::Component, system::Commands};
use bevy_reflect::reflect_trait;

#[reflect_trait]
/// A trait that can be implemented by types that wish to provide custom editor UI.
pub trait Editor {
    /// Should return true if the type is compatible with the Editor.
    /// For example, if the type wishes to modify the UI, it should return true so that the editor can call the `add_ui` method.
    fn is_plugin_compatible(&self) -> bool;
    /// Add the editor UI to the commands.
    fn add_ui(&self, commands: &mut Commands);
    /// Returns a list of `TypeIds` that this editor can target.
    fn ui_target(&self) -> Option<Vec<TypeId>>;
}

#[derive(Component)]
/// A marker component to test UI targeting.
pub struct TestUiTarget;
