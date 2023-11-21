use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;

use crate::game::assets::GameAssets;

#[derive(Resource)]
pub struct PapermanResource {
    pub scene: Handle<Scene>,
    pub animations: HashMap<String, Handle<AnimationClip>>,
}

pub fn prepare_paperman_resource(
    mut commands: Commands,
    assets: Res<GameAssets>,
    gltfs: Res<Assets<Gltf>>,
) {
    let paperman = gltfs.get(assets.paperman.id()).unwrap();

    let animations = paperman.named_animations.clone();

    let scene = paperman.default_scene.clone().unwrap();

    commands.insert_resource(PapermanResource { scene, animations });
    info!("PapermanResource prepared!");
}
