use bevy::core_pipeline::core_3d::{Camera3d, Camera3dBundle};
use bevy::ecs::query::With;
use bevy::ecs::schedule::OnEnter;
use bevy::ecs::system::Query;
use bevy::math::Vec3;
use bevy::prelude::{App, Commands, Plugin, Startup};
use bevy::render::camera::Camera;
use bevy::transform::components::Transform;

use crate::states::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
        app.add_systems(OnEnter(GameState::GameRunning), make_camera_visible);
    }
}

fn startup(mut commands: Commands) {
    // let position = Vec3::new(142.0, 27.0, 11.74);
    // let transform = Transform::from_translation(position + Vec3::new(-2.0, 4.0, -34.5))
    //     .looking_at(Vec3::ZERO, position);

    // let translation = Vec3::new(141.39197, 32.601074, -22.928705);
    // let rotation = Quat::from_xyzw(-0.0006912892, -0.9956973, -0.09236238, 0.0074523287);
    // let transform = Transform::from_translation(translation).mul_transform(Transform::from_rotation(rotation));

    let transform = Transform::from_translation(Vec3::ZERO).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn((
        Camera3dBundle {
            transform,
            camera: Camera {
                order: 1,
                is_active: false,
                ..Default::default()
            },
            ..Default::default()
        },
        bevy_flycam::FlyCam,
    ));
}

fn make_camera_visible(mut query: Query<&mut Camera, With<Camera3d>>) {
    query.single_mut().is_active = true;
}
