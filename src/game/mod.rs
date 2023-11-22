use bevy::prelude::*;

use self::states::{finished_game_loading_system, GameState};

mod assets;
mod building;
mod camera;
mod paperman;
mod render;
mod states;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<states::GameState>();
        app.add_plugins((
            assets::GameAssetPlugin,
            building::BuildingPlugin,
            paperman::PapermanPlugin,
            camera::CameraPlugin,
            render::RenderPlugin,
        ));
        app.add_systems(
            Update,
            finished_game_loading_system.run_if(in_state(GameState::GameLoading)),
        );
    }
}
