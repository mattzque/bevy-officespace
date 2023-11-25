use bevy::ecs::system::RunSystemOnce;
use bevy::gltf::{Gltf, GltfMesh, GltfNode};
use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;

use crate::common::track::Track;
use crate::game::assets::GameAssets;

#[derive(Resource)]
pub struct BuildingResource {
    pub scene_lopen: Handle<Scene>,
    pub scene_ropen: Handle<Scene>,
    pub scene_lropen: Handle<Scene>,
    pub tracks: Vec<Track>,
}

fn hide_by_mesh_in_world(world: &mut World, mesh: AssetId<Mesh>) {
    for (mut visibility, world_mesh) in world
        .query::<(&mut Visibility, &Handle<Mesh>)>()
        .iter_mut(world)
    {
        if world_mesh.id() == mesh {
            *visibility = Visibility::Hidden;
        }
    }
}

fn disable_frustum_culling_in_world(world: &mut World) {
    world.run_system_once(
        |query: Query<Entity, With<Handle<Mesh>>>, mut commands: Commands| {
            for entity in query.iter() {
                commands.entity(entity).insert(NoFrustumCulling);
            }
        },
    );
}

pub fn prepare_building_resource(
    mut commands: Commands,
    assets: Res<GameAssets>,
    gltfs: Res<Assets<Gltf>>,
    gltf_nodes: Res<Assets<GltfNode>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    meshes: Res<Assets<Mesh>>,
    mut scenes: ResMut<Assets<Scene>>,
) {
    let building = gltfs.get(assets.building.id()).unwrap();

    let scene_lopen = building
        .named_scenes
        .get("LOpen")
        .expect("Missing scene LOpen")
        .clone();
    let scene_ropen = building
        .named_scenes
        .get("ROpen")
        .expect("Missing scene ROpen")
        .clone();
    let scene_lropen = building
        .named_scenes
        .get("LROpen")
        .expect("Missing scene LROpen")
        .clone();

    // find tracks:
    const TRACK_PREFIX: &str = "track_l";
    let mut tracks: Vec<_> = building
        .named_nodes
        .iter()
        .filter(|(name, _)| name.starts_with(TRACK_PREFIX))
        .map(|(name, node_handle)| {
            let layer = name[TRACK_PREFIX.len()..].parse::<usize>().unwrap();
            // what the fuck is this API GltfNode -> GltfMesh -> GltfPrimitive -> Mesh
            let node = gltf_nodes.get(node_handle.id()).expect("no gltf node");
            let mesh = gltf_meshes
                .get(node.mesh.as_ref().unwrap())
                .expect("no gltf mesh");
            let mesh_handle = mesh.primitives.first().unwrap().mesh.clone();
            let mesh = meshes.get(mesh_handle.clone()).unwrap();
            let track = Track::from_mesh(mesh).unwrap();
            (layer, track, mesh_handle)
        })
        .collect();
    tracks.sort_by_key(|(layer, _, _)| *layer);

    // hide track meshes in the scene and disable frustum culling:
    for scene in building.scenes.iter() {
        if let Some(scene) = scenes.get_mut(scene.clone()) {
            for (_, _, mesh_handle) in tracks.iter() {
                hide_by_mesh_in_world(&mut scene.world, mesh_handle.id());
            }
            disable_frustum_culling_in_world(&mut scene.world);
        }
    }

    let tracks: Vec<Track> = tracks.into_iter().map(|(_, track, _)| track).collect();

    commands.insert_resource(BuildingResource {
        scene_lopen,
        scene_ropen,
        scene_lropen,
        tracks,
    });
}
