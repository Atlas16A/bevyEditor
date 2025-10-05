//! Internal Bevy Launcher plugin, where the bevy launcher is built.

use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};

use bevy::feathers::controls::{button, ButtonProps};
use bevy::feathers::dark_theme::create_dark_theme;
use bevy::feathers::theme::{ThemedText, UiTheme};
use bevy::feathers::FeathersPlugins;
use bevy::tasks::Task;
use bevy::ui_widgets::{observe, Activate};
use bevy::{prelude::*, window::WindowResolution};
use bevy_cli::commands::{
    build::{build, BuildArgs, BuildSubcommands, BuildWebArgs},
    completions::completions,
    lint::{lint, InstallArgs, LintArgs, LintSubcommands},
    new::{new, NewArgs},
    run::{run, RunArgs, RunSubcommands, RunWebArgs},
};

mod dialog;
use dialog::spawn_folder_dialog;

use crate::dialog::{any_async_threads, get_future, DialogRequest};

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
        //.add_plugins(FileDialogPlugin)
        .insert_resource(UiTheme(create_dark_theme()))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            get_future::<OpenProjectFileDialog, _>
                .pipe(open_project)
                .run_if(any_async_threads.and(any_with_component::<OpenProjectFileDialog>)),
        );
    }
}

fn main() {
    App::new().add_plugins(LauncherInternalPlugin).run();
}

fn setup(mut commands: Commands) {
    // ui camera
    commands.spawn(Camera2d);
    let root = base_ui();
    commands.spawn(root);
}

fn base_ui() -> impl Bundle {
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
            children![(
                button(
                    ButtonProps::default(),
                    (),
                    Spawn((Text::new("Open Project"), ThemedText))
                ),
                observe(spawn_folder_dialog::<OpenProjectFileDialog>)
            ),]
        ),],
    )
}

//spawn_folder_dialog::<OpenProjectFileDialog>
#[derive(Component)]
struct OpenProjectFileDialog(Task<Option<PathBuf>>);

impl DialogRequest for OpenProjectFileDialog {
    fn new(task: Task<Option<PathBuf>>) -> Self {
        Self(task)
    }
}

impl Future for OpenProjectFileDialog {
    type Output = Option<PathBuf>;
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        Future::poll(core::pin::Pin::new(&mut self.get_mut().0), cx)
    }
}

fn open_project(
    In(path): In<Option<Option<PathBuf>>>,
    mut app_exit_events: MessageWriter<AppExit>,
) {
    // If the selected folder contains a Cargo.toml file, it runs the project in editor mode.
    // This should be changed later to open the cargo and confirm it is a bevy project with a version check to ensure it has the editor setup.
    // the main.rs file should also be checked to ensure it has the editor setup.
    // If the selected folder does not contain a Cargo.toml file, it logs an error
    // and does not close the launcher.
    //
    // This should also be changed to swap the launcher out with a loading screen while the editor is compiling to avoid user confusion.
    let Some(Some(path)) = path else {
        return;
    };
    info!("Opening project at {:?}", path);
    if folder_contains_cargo_toml(path.as_path()) {
        info!("Selected folder: {:?}", path);
        run_project_in_editor_mode(path.as_path());
        //app_exit_events.write(AppExit::Success);
    } else {
        error!("The selected folder does not contain a Cargo.toml file.");
    }
}

fn run_project_in_editor_mode(path: &Path) {
    // Runs the project in editor mode by executing `cargo run --release -- --editor-mode` in the specified path.
    // This assumes that the project is a valid Cargo project and that the main.rs file is set up to handle the `--editor-mode` argument.
    // This should be changed later to spawn the editor as a child process of the lancher with a IPC channel to tell the launcher of any error events that may have killed the editor.
    // This will allow the launcher to inform the user of any issues with the editor without having to check the terminal output.
    std::process::Command::new("cargo")
        .arg("run")
        .arg("--release")
        .arg("--")
        // Editor mode argument
        .arg("--editor-mode")
        .current_dir(path)
        .process_group(0)
        .spawn()
        .expect("Failed to start the project in editor mode");
}

fn folder_contains_cargo_toml(folder: &Path) -> bool {
    folder.join("Cargo.toml").exists()
}
