use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::core_pipeline::prepass::{DepthPrepass, NormalPrepass};
use bevy::math::{Quat, Vec3};
use bevy::prelude::{App, Commands, Plugin, Startup};
use bevy::render::camera::Camera;
use bevy::transform::components::Transform;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

fn startup(mut commands: Commands) {
    // let position = Vec3::new(142.0, 27.0, 11.74);
    // let transform = Transform::from_translation(position + Vec3::new(-2.0, 4.0, -34.5))
    //     .looking_at(Vec3::ZERO, position);

    let translation = Vec3::new(141.39197, 32.601074, -22.928705);
    let rotation = Quat::from_xyzw(-0.0006912892, -0.9956973, -0.09236238, 0.0074523287);

    let transform =
        Transform::from_translation(translation).mul_transform(Transform::from_rotation(rotation));

    commands.spawn((
        Camera3dBundle {
            transform,
            camera: Camera {
                order: 2,
                ..Default::default()
            },
            ..Default::default()
        },
        DepthPrepass,
        NormalPrepass,
    ));
}
