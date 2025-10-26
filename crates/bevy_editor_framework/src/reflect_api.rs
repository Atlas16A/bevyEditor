use core::any::TypeId;

use bevy_asset::{AssetServer, Handle};
use bevy_ecs::{
    bundle::Bundle,
    component::Component,
    system::{Commands, Res},
};
use bevy_reflect::reflect_trait;
use bevy_scene2::{Scene, ScenePatch};

#[reflect_trait]
/// A trait that can be implemented by types that wish to provide custom editor UI.
pub trait Editor {
    /// Should return true if the type is compatible with the Editor.
    /// For example, if the type wishes to modify the UI, it should return true so that the editor can call the `add_ui` method.
    fn is_plugin_compatible(&self) -> bool;
    /// Returns a list of `TypeIds` that this editor can target.
    fn ui_target(&self) -> Option<Vec<TypeId>>;
    /// Applies a bsn patch to the target.
    fn load_scene(&self, assets: &AssetServer) -> Handle<ScenePatch>;
}

#[derive(Component)]
/// A marker component to test UI targeting.
pub struct TestUiTarget;
