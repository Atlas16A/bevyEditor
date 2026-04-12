//! A simple launcher for Bevy applications. 
//! This is intended to be a standalone binary that can be distributed for the purpose of creating/managing Bevy projects.
use bevy::{
    feathers::{FeathersCorePlugins}, prelude::*
};

/// The internal Bevy Launcher plugin.
pub struct LauncherInternalPlugin;

impl Plugin for LauncherInternalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Launcher".to_string(),
                resolution: bevy::window::WindowResolution::new(1000, 600),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::oklch(0.2046, 0.0, 0.0)))
        .add_plugins(FeathersCorePlugins);
    }
}

fn main() {
    App::new().add_plugins(LauncherInternalPlugin).run();
}