use bevy::{
    app::{App, PluginGroup},
    log::LogPlugin,
    render::pipelined_rendering::PipelinedRenderingPlugin,
    window::{PresentMode, Window, WindowPlugin},
    DefaultPlugins,
};
use bevy_officespace::GamePlugin;

const WINDOW_TITLE: &str = "bevy-officespace";

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy_officespace=debug".into(),
                    level: bevy::log::Level::DEBUG,
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: WINDOW_TITLE.to_string(),
                        present_mode: PresentMode::AutoVsync,
                        fit_canvas_to_parent: true,
                        resizable: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .disable::<PipelinedRenderingPlugin>(),
        )
        .add_plugins(GamePlugin)
        .run();
}
