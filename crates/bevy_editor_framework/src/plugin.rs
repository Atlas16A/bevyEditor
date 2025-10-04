use std::any::TypeId;

use bevy_app::{App, Plugin, Startup};
use bevy_ecs::{reflect::AppTypeRegistry, system::Res};
use bevy_log::info;
use bevy_reflect::{
    prelude::ReflectDefault, serde::TypedReflectDeserializer, Reflect, ReflectKind,
};

use crate::{reflect_api::EditorCompatibility, ReflectEditorCompatibility};

pub struct EditorReflectionPlugin;

impl Plugin for EditorReflectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(registry: Res<AppTypeRegistry>) {
    let registry = registry.read();
    registry.iter().for_each(|ty| {
        // Iterate over all registered types and their fields

        let typeinfo = ty.type_info().ty().path();
        if typeinfo.contains("GamePlugin") {
            if ty.type_info().kind() != ReflectKind::Struct {
                return;
            }
            info!("Type: {typeinfo:?}");
            let reflect_editor_compatibility = ty
                .data::<ReflectEditorCompatibility>()
                .expect("`ReflectEditorCompatibility` should be registered");

            let reflect_default = ty
                .data::<ReflectDefault>()
                .expect("`ReflectDefault` should be registered");

            let mut value: Box<dyn Reflect> = reflect_default.default();

            let identifiable: &dyn EditorCompatibility = reflect_editor_compatibility
                .get(value.as_reflect())
                .unwrap();

            if identifiable.is_plugin_compatible() {
                info!(
                    " - This Plugin is compatible with Editor, consider adding it to the editor!"
                );
            }

            /* let custom_attributes = type_info.custom_attributes();
            custom_attributes.iter().for_each(|attr| {
                info!(" - Custom Attribute: {attr:?}");
                if type_info.has_attribute_by_id(TypeId::of::<EditorCompatible>()) {
                    info!("   - GamePlugin is compatible with Editor, consider adding it to the editor!");
                }
            }); */
        }
    });
}
