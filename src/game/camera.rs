use bevy::prelude::*;

use super::states::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cameras);
        app.add_systems(OnEnter(GameState::GameRunning), make_camera_visible);
    }
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::ZERO).looking_at(Vec3::ZERO, Vec3::Y),
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
