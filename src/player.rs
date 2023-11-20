use anyhow::{anyhow, Result};
use bevy::a11y::accesskit::Vec2;
use bevy::animation::{AnimationPlayer, RepeatAnimation};
use bevy::asset::Assets;
use bevy::ecs::component::Component;
use bevy::ecs::schedule::NextState;
use bevy::ecs::system::{Query, ResMut, Resource};
use bevy::gltf::{Gltf, GltfMesh, GltfNode};
use bevy::log::info;
use bevy::math::{Quat, Vec3};
use bevy::pbr::{AmbientLight, PbrBundle, PointLightBundle, StandardMaterial};
use bevy::prelude::{in_state, App, Commands, IntoSystemConfigs, OnEnter, Plugin, Res, Update};
use bevy::render::color::Color;
use bevy::render::mesh::{shape, Mesh};
use bevy::scene::SceneBundle;
use bevy::transform::components::Transform;

use crate::loader::GameAssets;
use crate::navmesh::NavMesh;
use crate::states::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameLoading), setup_player);
    }
}

#[derive(Component, Debug)]
pub struct Player;

/// Player position on the navmesh in 2d
#[derive(Component, Debug)]
pub struct PlayerPosition(Vec3);

/// Player rotation in radians
#[derive(Component, Debug)]
pub struct PlayerRotation(f32);

fn setup_player(
    mut commands: Commands,
    assets: Res<GameAssets>,
    gltfs: Res<Assets<Gltf>>,
    gltf_nodes: Res<Assets<GltfNode>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    meshes: Res<Assets<Mesh>>,
    mut state: ResMut<NextState<GameState>>,
) {
    let building = gltfs.get(assets.building.id()).unwrap();
    let player = building.named_nodes.get("player").unwrap();
    info!(":: {:?}", gltf_nodes.get(player.id()).unwrap().transform);
    let position = gltf_nodes.get(player.id()).unwrap().transform.translation;

    let handle = building.named_nodes.get("navmesh").unwrap();
    let handle = gltf_nodes.get(handle.id()).unwrap().clone().mesh.unwrap();
    let handle = gltf_meshes
        .get(handle.id())
        .unwrap()
        .clone()
        .primitives
        .first()
        .unwrap()
        .mesh
        .clone();
    let navmesh = meshes.get(handle.id()).unwrap();
    let navmesh = NavMesh::from_mesh(navmesh).unwrap();

    info!("position: {:?}", position);
    info!("navmesh: {:#?}", navmesh);

    assert!(
        navmesh.contains_point(position),
        "Player marker not positioned on navmesh!"
    );

    commands.insert_resource(navmesh);

    commands.spawn((Player, PlayerPosition(position), PlayerRotation(0.0)));

    state.set(GameState::GameRunning);
}
