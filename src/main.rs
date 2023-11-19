// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod camera;
mod loader;
mod render;
mod states;

use bevy::{app::App, DefaultPlugins};
use camera::CameraPlugin;
use loader::LoaderPlugin;
use render::RenderPlugin;
use states::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((LoaderPlugin, CameraPlugin, RenderPlugin))
        .add_state::<GameState>()
        .run();
}
