use anyhow::Result;
use bevy::gltf::Gltf;
use bevy::prelude::*;

use crate::common::loader::AssetLoader;

use super::states::{
    finished_init_system, finished_loaded_system, finished_loading_system, GameState,
};

mod building;
mod paperman;

pub use building::BuildingResource;
pub use paperman::PapermanResource;

pub struct GameAssetPlugin;

impl Plugin for GameAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Init),
            load_assets_system.pipe(finished_init_system),
        );
        app.add_systems(
            Update,
            update_loading_system
                .pipe(finished_loading_system)
                .run_if(in_state(GameState::AssetsLoading)),
        );
        app.add_systems(
            OnEnter(GameState::AssetsLoaded),
            (
                building::prepare_building_resource.before(finished_loaded_system),
                paperman::prepare_paperman_resource.before(finished_loaded_system),
                finished_loaded_system,
            ), // .pipe(finished_loaded_system), // prepare_resources
               //     .map(finished_loaded_system),
        );
    }
}

#[derive(Resource)]
pub struct GameAssets {
    pub paperman: Handle<Gltf>,
    pub building: Handle<Gltf>,
}

fn load_assets_system(mut commands: Commands, server: Res<AssetServer>) {
    // animations: ["idle", "walking", "death", "tpose", "attack", "kick", "running"]
    let paperman: Handle<Gltf> = server.load("paperman.gltf");
    let building: Handle<Gltf> = server.load("building.gltf");
    let mut loader = AssetLoader::new();

    loader.add_pending(paperman.clone_weak().id().untyped());
    loader.add_pending(building.clone_weak().id().untyped());

    commands.insert_resource(GameAssets { paperman, building });
    commands.insert_resource(loader);
}

fn update_loading_system(
    mut loader: ResMut<AssetLoader>,
    server: Res<AssetServer>,
) -> Result<bool> {
    loader.update_loading_state(&server)?;
    Ok(loader.is_finished())
}

// fn prepare_resources(

// )
