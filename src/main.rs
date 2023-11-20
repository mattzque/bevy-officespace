// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod camera;
mod loader;
mod navmesh;
mod player;
mod render;
mod states;

use bevy::{
    app::{App, PluginGroup},
    window::{PresentMode, Window, WindowPlugin},
    DefaultPlugins,
};
use camera::CameraPlugin;
use loader::LoaderPlugin;
use player::PlayerPlugin;
use render::RenderPlugin;
use states::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync, // Reduces input lag.
                fit_canvas_to_parent: true,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(bevy_flycam::NoCameraPlayerPlugin)
        .add_plugins((LoaderPlugin, CameraPlugin, RenderPlugin, PlayerPlugin))
        .add_state::<GameState>()
        .run();
}
