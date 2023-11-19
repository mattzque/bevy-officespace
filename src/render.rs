use bevy::animation::{AnimationPlayer, RepeatAnimation};
use bevy::asset::Assets;
use bevy::ecs::system::{Query, ResMut};
use bevy::gltf::Gltf;
use bevy::math::{Quat, Vec3};
use bevy::pbr::{AmbientLight, PbrBundle, PointLightBundle, StandardMaterial};
use bevy::prelude::{in_state, App, Commands, IntoSystemConfigs, OnEnter, Plugin, Res, Update};
use bevy::render::color::Color;
use bevy::render::mesh::{shape, Mesh};
use bevy::scene::SceneBundle;
use bevy::transform::components::Transform;

use crate::loader::GameAssets;
use crate::states::GameState;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameLoading), startup);
        app.add_systems(Update, update.run_if(in_state(GameState::GameLoading)));
    }
}

fn startup(
    mut commands: Commands,
    assets: Res<GameAssets>,
    gltfs: Res<Assets<Gltf>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.4,
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 500000.0,
            subdivisions: 100,
        })),
        material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(142.0, 30.0, 11.74)),
        ..Default::default()
    });

    let paperman = gltfs.get(assets.paperman.id()).unwrap();

    commands.spawn(SceneBundle {
        transform: Transform::from_translation(Vec3::new(142.0, 19.0, 11.74))
            .with_scale(Vec3::splat(7.0))
            .with_rotation(Quat::from_axis_angle(Vec3::Y, (270.0_f32).to_radians())),
        scene: paperman.default_scene.clone().unwrap(),
        ..Default::default()
    });

    let building = gltfs.get(assets.building.id()).unwrap();

    commands.spawn(SceneBundle {
        scene: building.default_scene.clone().unwrap(),
        ..Default::default()
    });
}

fn update(
    assets: Res<GameAssets>,
    gltfs: Res<Assets<Gltf>>,
    mut animation_player: Query<&mut AnimationPlayer>,
) {
    if let Ok(mut player) = animation_player.get_single_mut() {
        let paperman = gltfs.get(assets.paperman.id()).unwrap();
        player.play(paperman.named_animations.get("running").unwrap().clone());
        player.set_repeat(RepeatAnimation::Forever);
    }
}
