use bevy::gltf::Gltf;
use bevy::prelude::*;

use crate::game::assets::GameAssets;

#[derive(Resource)]
pub struct PaperboxResource {
    pub scene: Handle<Scene>,
}

pub fn prepare_paperbox_resource(
    mut commands: Commands,
    assets: Res<GameAssets>,
    gltfs: Res<Assets<Gltf>>,
) {
    let paperbox = gltfs.get(assets.paperbox.id()).unwrap();
    let scene = paperbox.default_scene.clone().unwrap();
    commands.insert_resource(PaperboxResource { scene });
}
