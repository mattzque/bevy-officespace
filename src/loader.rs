use bevy::asset::{Assets, RecursiveDependencyLoadState, UntypedAssetId};
use bevy::ecs::schedule::NextState;
use bevy::ecs::system::{ResMut, Resource};
use bevy::gltf::Gltf;
use bevy::log::{error, info};
use bevy::prelude::{
    in_state, App, AssetServer, Commands, Handle, IntoSystemConfigs, Plugin, Res, Startup, Update,
};
use bevy::utils::HashSet;

use crate::states::GameState;

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
        app.add_systems(
            Update,
            (
                update_loading.run_if(in_state(GameState::AssetsLoading)),
                finished_loading.run_if(in_state(GameState::AssetsLoaded)),
            ),
        );
    }
}

#[derive(Resource)]
pub struct GameAssets {
    pending: HashSet<UntypedAssetId>,
    pub paperman: Handle<Gltf>,
    pub building: Handle<Gltf>,
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<NextState<GameState>>,
) {
    // animations: ["idle", "walking", "death", "tpose", "attack", "kick", "running"]
    let paperman: Handle<Gltf> = asset_server.load("paperman.gltf");
    let building: Handle<Gltf> = asset_server.load("building.gltf");
    let pending = HashSet::from([paperman.id().untyped(), building.id().untyped()]);
    let assets = GameAssets {
        pending,
        paperman,
        building,
    };
    commands.insert_resource(assets);
    state.set(GameState::AssetsLoading);
}

fn update_loading(
    asset_server: Res<AssetServer>,
    mut assets: ResMut<GameAssets>,
    mut state: ResMut<NextState<GameState>>,
) {
    for pending in assets.pending.clone().iter() {
        let path = asset_server.get_path(*pending);
        if let Some((_, _, loading_state)) = asset_server.get_load_states(*pending) {
            if loading_state == RecursiveDependencyLoadState::Loaded {
                info!("loaded asset: {:?}", path);

                // remove from pending
                assets.pending.remove(pending);
            } else if loading_state == RecursiveDependencyLoadState::Failed {
                error!("error loading asset: {:?}", path);
            }
        }
    }

    if assets.pending.is_empty() {
        info!("all assets loaded!");
        state.set(GameState::AssetsLoaded);
    }
}

fn finished_loading(
    assets: Res<GameAssets>,
    gltfs: Res<Assets<Gltf>>,
    mut state: ResMut<NextState<GameState>>,
) {
    if let Some(paperman) = gltfs.get(assets.paperman.id()) {
        println!("loaded paperman :)");
        println!("animations: {:?}", paperman.named_animations.keys());
    }
    // if let Some(building) = gltfs.get(assets.building.id()) {
    //     // println!("building: {:#?}", building);
    // }
    state.set(GameState::GameLoading);
}
