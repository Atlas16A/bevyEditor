use argh::FromArgs;

use bevy_app::{App, AppExit, Plugins, PluginsState};

#[derive(FromArgs)]
/// Main command line arguments
struct Args {
    #[argh(switch, short = 'e')]
    /// whether to run as the editor or the game
    editor_mode: bool,
}

/// While [App] is Bevy's main application type, [EditorApp] is a thin wrapper around it that
/// conditionally adds editor or game plugins based on command line arguments.
/// This allows the same binary to run in either editor mode or game mode, depending on
/// the presence of the `--editor-mode` flag.
/// Note that when using [EditorApp], you should wrap your game into a single plugin and add it
/// using [EditorAppExt::add_plugins]. This ensures that the game plugins are only added
/// when not in editor mode and ensures systems you wish to register don't get registered in
/// the wrong mode.
/// # Example
///
/// Here is a simple Editor setup for a Bevy app:
///
/// ```
/// # use bevy_app::prelude::*;
/// # use bevy_ecs::prelude::*;
/// #
/// fn main() {
///    App::new()
///         .add_plugins(GamePlugin)
///         .add_editor_plugins(bevy_editor::EditorPlugin::default())
///         .run();
/// }
///
/// fn hello_world_system() {
///    println!("hello world");
/// }
/// ```
#[derive(Debug)]
pub struct EditorApp(App);

pub trait EditorAppExt {
    fn new() -> Self;

    /// Adds editor plugins if the `--editor` flag is present in the command line arguments.
    fn add_editor_plugins<M>(&mut self, plugins: impl Plugins<M>) -> &mut Self;

    fn add_plugins<M>(&mut self, plugins: impl Plugins<M>) -> &mut Self;

    fn run(&mut self) -> AppExit;
}

impl EditorAppExt for EditorApp {
    fn new() -> Self {
        Self(App::new())
    }

    fn add_editor_plugins<M>(&mut self, plugins: impl Plugins<M>) -> &mut Self {
        let args = argh::from_env::<Args>();
        if !args.editor_mode {
            return self;
        }
        if matches!(
            self.0.plugins_state(),
            PluginsState::Cleaned | PluginsState::Finished
        ) {
            panic!(
                "Plugins cannot be added after App::cleanup() or App::finish() has been called."
            );
        }

        plugins.add_to_app(&mut self.0);
        self
    }

    fn add_plugins<M>(&mut self, plugins: impl Plugins<M>) -> &mut Self {
        let args = argh::from_env::<Args>();
        if args.editor_mode {
            return self;
        }
        if matches!(
            self.0.plugins_state(),
            PluginsState::Cleaned | PluginsState::Finished
        ) {
            panic!(
                "Plugins cannot be added after App::cleanup() or App::finish() has been called."
            );
        }

        plugins.add_to_app(&mut self.0);
        self
    }

    fn run(&mut self) -> AppExit {
        self.0.run()
    }
}
