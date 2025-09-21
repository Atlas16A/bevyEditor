//! Internal Bevy Launcher plugin, where the bevy launcher is built.
use bevy::prelude::*;
use bevy_cli::commands::{
    build::{build, BuildArgs, BuildSubcommands, BuildWebArgs},
    completions::completions,
    lint::{lint, InstallArgs, LintArgs, LintSubcommands},
    new::{new, NewArgs},
    run::{run, RunArgs, RunSubcommands, RunWebArgs},
};

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
