use std::any::TypeId;

use bevy_app::{App, Plugin, Startup};
use bevy_ecs::{reflect::AppTypeRegistry, system::Res};
use bevy_reflect::ReflectKind;
use bevy_log::info;

use crate::reflect_api::EditorCompatible;

pub struct EditorReflectionPlugin;

impl Plugin for EditorReflectionPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup);
    }
}

fn setup(registry: Res<AppTypeRegistry>) {
    let registry = registry.read();
    registry.iter().for_each(|ty| {
        // Iterate over all registered types and their fields

        let typeinfo = ty.type_info().ty().path();
        if typeinfo.contains("Plugin") {
            

            if ty.type_info().kind() != ReflectKind::Struct {
                return;
            }
            info!("Type: {typeinfo:?}");
            let type_info = ty.type_info().as_struct().unwrap();
            let custom_attributes = type_info.custom_attributes();
            custom_attributes.iter().for_each(|attr| {
                info!(" - Custom Attribute: {attr:?}");
                if type_info.has_attribute_by_id(TypeId::of::<EditorCompatible>()) {
                    info!("   - GamePlugin is compatible with Editor, consider adding it to the editor!");
                }
            });
        }
    });
}