//! Internal Bevy Launcher plugin, where the bevy launcher is built.

use bevy::feathers::controls::{button, ButtonProps};
use bevy::feathers::dark_theme::create_dark_theme;
use bevy::feathers::theme::{ThemedText, UiTheme};
use bevy::feathers::FeathersPlugins;
use bevy::ui_widgets::Callback;
use bevy::{prelude::*, window::WindowResolution};
use bevy_cli::commands::{
    build::{build, BuildArgs, BuildSubcommands, BuildWebArgs},
    completions::completions,
    lint::{lint, InstallArgs, LintArgs, LintSubcommands},
    new::{new, NewArgs},
    run::{run, RunArgs, RunSubcommands, RunWebArgs},
};

mod dialog;
use dialog::{spawn_file_dialog, FileDialogPlugin};

/// The internal Bevy Launcher plugin.
pub struct LauncherInternalPlugin;

impl Plugin for LauncherInternalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Launcher".to_string(),
                resolution: WindowResolution::new(1000, 600),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::oklch(0.2046, 0.0, 0.0)))
        .add_plugins(FeathersPlugins)
        .add_plugins(FileDialogPlugin)
        .insert_resource(UiTheme(create_dark_theme()))
        .add_systems(Startup, setup);
    }
}

fn main() {
    App::new().add_plugins(LauncherInternalPlugin).run();
}

fn setup(mut commands: Commands) {
    // ui camera
    commands.spawn(Camera2d);
    let root = base_ui(&mut commands);
    commands.spawn(root);
}

fn base_ui(commands: &mut Commands) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![(
            Node {
                width: Val::Px(100.0),
                height: Val::Px(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            children![button(
                ButtonProps {
                    on_click: Callback::System(commands.register_system(spawn_file_dialog)),
                    ..default()
                },
                (),
                Spawn((Text::new("Open Project"), ThemedText))
            ),],
        )],
    )
}
