use std::{
    os::unix::process::CommandExt,
    path::{Path, PathBuf},
};

use bevy::{
    prelude::*,
    tasks::{
        futures_lite::{self, future},
        AsyncComputeTaskPool, Task,
    },
    ui_widgets::Activate,
};

use rfd::FileDialog;
pub struct FileDialogPlugin;

impl Plugin for FileDialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SelectedFileEvent>()
            .add_systems(
                Update,
                get_file_response.run_if(any_async_threads.and(any_with_component::<SelectedFile>)),
            )
            .add_systems(Update, selected_file_handler);
    }
}

#[derive(Component)]
/// A component that holds a task for selecting a file.
pub struct SelectedFile(Task<Option<PathBuf>>);

#[derive(Message)]
pub struct SelectedFileEvent(PathBuf);

pub fn get_file_response(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut SelectedFile)>,
    mut events: MessageWriter<SelectedFileEvent>,
) {
    for (entity, mut selected_file) in tasks.iter_mut() {
        if let Some(result) = future::block_on(future::poll_once(&mut selected_file.0)) {
            if result.is_none() {
                commands.entity(entity).remove::<SelectedFile>();
                continue;
            }
            info!("{:?}", result);
            events.write(SelectedFileEvent(result.unwrap()));
            commands.entity(entity).remove::<SelectedFile>();
        }
    }
}

/// Spawns a single file dialog future, if one is not already active.
pub fn spawn_file_dialog(
    _: In<Activate>,
    mut commands: Commands,
    active_dialogs: Query<&SelectedFile>,
) {
    if !active_dialogs.is_empty() {
        return;
    }
    let thread_pool = AsyncComputeTaskPool::get();

    let task = thread_pool.spawn(async move { FileDialog::new().pick_folder() });
    commands.spawn(SelectedFile(task));
}

/// Returns true if there are any threads in the async compute task pool.
fn any_async_threads() -> bool {
    let thread_pool = AsyncComputeTaskPool::get();
    thread_pool.thread_num() > 1
}

fn folder_contains_cargo_toml(folder: &Path) -> bool {
    folder.join("Cargo.toml").exists()
}

fn run_project_in_editor_mode(path: &Path) {
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

fn selected_file_handler(
    mut events: MessageReader<SelectedFileEvent>,
    mut app_exit_events: MessageWriter<AppExit>,
) {
    /// Handles the selected file event.
    /// If the selected folder contains a Cargo.toml file, it runs the project in editor mode.
    /// This should be changed later to open the cargo and confirm it is a bevy project with a version check to ensure it has the editor setup.
    /// the main.rs file should also be checked to ensure it has the editor setup.
    /// If the selected folder does not contain a Cargo.toml file, it logs an error
    /// and does not close the launcher.
    ///
    /// This should also be changed to swap the launcher out with a loading screen while the editor is compiling to avoid user confusion.
    for SelectedFileEvent(path) in events.read() {
        if folder_contains_cargo_toml(path) {
            info!("Selected folder: {:?}", path);
            run_project_in_editor_mode(path);
            app_exit_events.write(AppExit::Success);
        } else {
            error!("The selected folder does not contain a Cargo.toml file.");
        }
    }
}
