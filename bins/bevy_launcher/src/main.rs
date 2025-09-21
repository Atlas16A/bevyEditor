//! Internal Bevy Launcher plugin, where the bevy launcher is built.
use bevy::prelude::*;

/// The internal Bevy Launcher plugin.
pub struct LauncherInternalPlugin;

impl Plugin for LauncherInternalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
    }
}

fn main() {
    App::new().add_plugins(LauncherInternalPlugin).run();
}
