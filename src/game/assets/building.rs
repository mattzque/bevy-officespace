use bevy::ecs::system::RunSystemOnce;
use bevy::gltf::{Gltf, GltfMesh, GltfNode};
use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;

use crate::{common::navmesh::NavMesh, game::assets::GameAssets};

#[derive(Resource)]
pub struct BuildingResource {
    pub scene: Handle<Scene>,
    pub player: Vec3,
    pub navmesh: NavMesh,
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

    let scene = building.default_scene.clone().unwrap();

    println!("building: {:#?}", building);

    let player = building.named_nodes.get("player").unwrap();
    let player_node = gltf_nodes.get(player.id()).unwrap();
    let player = player_node.transform.translation;

    let handle_navmesh = building.named_nodes.get("navmesh").unwrap();
    let handle_navmesh = gltf_nodes
        .get(handle_navmesh.id())
        .unwrap()
        .clone()
        .mesh
        .unwrap();
    let handle_navmesh = gltf_meshes
        .get(handle_navmesh.id())
        .unwrap()
        .clone()
        .primitives
        .first()
        .unwrap()
        .mesh
        .clone();
    let navmesh = meshes.get(handle_navmesh.id()).unwrap();
    let navmesh = NavMesh::from_mesh(navmesh).unwrap();

    // hide the navmesh in the scene:
    if let Some(scene) = scenes.get_mut(scene.clone()) {
        hide_by_mesh_in_world(&mut scene.world, handle_navmesh.id());
        disable_frustum_culling_in_world(&mut scene.world);
    }

    debug!("position: {:?}", player);
    debug!("navmesh: {:#?}", navmesh);

    assert!(
        navmesh.contains_point(player),
        "Player marker not positioned on navmesh!"
    );

    commands.insert_resource(BuildingResource {
        scene,
        player,
        navmesh,
    });
    info!("BuildingResource prepared.");
}
