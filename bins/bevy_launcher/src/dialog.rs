use std::path::PathBuf;

use bevy::{
    prelude::*,
    tasks::{futures_lite::future, IoTaskPool, Task},
    ui_widgets::Activate,
};

use rfd::FileDialog;

pub fn get_future<T, R>(mut commands: Commands, mut tasks: Query<(Entity, &mut T)>) -> Option<R>
where
    T: Component<Mutability = bevy::ecs::component::Mutable> + Unpin + Future<Output = R>,
{
    for (entity, mut task) in tasks.iter_mut() {
        let mut pinned = core::pin::Pin::new(&mut *task);
        let poll_once = future::poll_once(&mut pinned);
        if let Some(result) = future::block_on(poll_once) {
            commands.entity(entity).remove::<T>();
            return Some(result);
        }
    }
    None
}

pub trait DialogRequest {
    fn new(task: Task<Option<PathBuf>>) -> Self;
}

/// Spawns a single file dialog future, if one is not already active.
pub fn spawn_folder_dialog<T>(
    entity: On<Activate>,
    mut commands: Commands,
    active_dialogs: Query<&T>,
) where
    T: Component + DialogRequest,
{
    if !active_dialogs.is_empty() {
        return;
    }
    let thread_pool = IoTaskPool::get();

    let task = thread_pool.spawn(async move { FileDialog::new().pick_folder() });
    commands.entity(entity.entity).insert(T::new(task));
}

/// Returns true if there are any threads in the async compute task pool.
pub fn any_async_threads() -> bool {
    let thread_pool = IoTaskPool::get();
    thread_pool.thread_num() > 1
}
