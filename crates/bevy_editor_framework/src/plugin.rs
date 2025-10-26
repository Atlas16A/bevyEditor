use bevy_app::{App, Plugin, Startup};
use bevy_asset::AssetServer;
use bevy_camera::Camera2d;
use bevy_ecs::{
    reflect::AppTypeRegistry,
    resource::Resource,
    schedule::IntoScheduleConfigs,
    system::{Commands, In, IntoSystem, Res},
};
use bevy_log::info;
use bevy_reflect::{prelude::ReflectDefault, Reflect, TypeRegistration};
use bevy_scene2::ScenePatchInstance;

use crate::{reflect_api::Editor, ReflectEditor};

pub struct EditorReflectionPlugin;

impl Plugin for EditorReflectionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EditorTypesForEvaluation { types: Vec::new() });

        app.add_systems(
            Startup,
            (gather_editor_implementor_types.pipe(editor_plugin_types.pipe(store_registrations)),),
        );
        app.add_systems(Startup, plugin_editor_evaluation.after(store_registrations));
    }
}

#[derive(Resource)]
struct EditorTypesForEvaluation {
    types: Vec<TypeRegistration>,
}

fn reflect_default(registration: &TypeRegistration) -> Box<dyn Reflect> {
    registration
        .data::<ReflectDefault>()
        .expect("`ReflectDefault` should be registered")
        .default()
}

/// Iterate over all registered types and their fields.
/// Checking if the type has reflected the Editor trait and the Default trait.
/// This is required to be able to instantiate the type and call the Editor methods.
fn gather_editor_implementor_types(registry: Res<AppTypeRegistry>) -> Vec<TypeRegistration> {
    let mut types = Vec::new();
    let registry = registry.read();

    registry.iter().for_each(|ty| {
        if ty.data::<ReflectEditor>().is_some() && ty.data::<ReflectDefault>().is_some() {
            info!(
                "Type: {:?} - Has ReflectEditor and ReflectDefault registered",
                ty.type_info().ty().path()
            );
            types.push(ty.clone());
        }
    });
    types
}

/// This is a very naive way to check if the type implements the Plugin trait
/// It relies on the fact that ending with "Plugin" is a convention for naming plugins
///
/// It should be replaced with a more robust way to check if the type implements the Plugin trait
///
/// ```
/// let typeinfo = ty.type_info().ty().path();
/// if ty.data::<ReflectPlugin>().is_none() {
///    info!("Type: {typeinfo:?} - No ReflectPlugin");
///    return;
/// }
/// ```
fn editor_plugin_types(In(types): In<Vec<TypeRegistration>>) -> Vec<TypeRegistration> {
    types
        .into_iter()
        .filter(|ty| ty.type_info().ty().path().contains("Plugin"))
        .collect()
}

fn plugin_editor_evaluation(
    resource: Res<EditorTypesForEvaluation>,
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    for ty in resource.types.iter() {
        let typeinfo = ty.type_info().ty().path();
        info!("Editor Plugin Type: {typeinfo:?}");

        let value = reflect_default(ty);

        let reflect_editor = ty
            .data::<ReflectEditor>()
            .expect("`ReflectEditor` should be registered");

        let editor_implementor: &dyn Editor = reflect_editor.get(value.as_reflect()).unwrap();

        if editor_implementor.is_plugin_compatible() {
            info!(" - This Plugin is compatible with Editor, consider adding it to the editor!");
        }

        commands.spawn(Camera2d);

        let _ = editor_implementor
            .ui_target()
            .unwrap()
            .into_iter()
            .map(|targets| {
                info!(" - UI Target: {targets:?}");
            });

        commands.spawn(ScenePatchInstance(editor_implementor.load_scene(&assets)));
    }
}

fn store_registrations(In(types): In<Vec<TypeRegistration>>, mut commands: Commands) {
    commands.insert_resource(EditorTypesForEvaluation { types });
}
