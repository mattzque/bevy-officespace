use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;

use self::animation::{setup_animation_system, NextPapermanAnimation, PapermanAnimationQueue};
use self::controller::Options;

use super::{
    assets::{BuildingResource, PapermanResource},
    states::GameState,
};

mod animation;
mod controller;

pub struct PapermanPlugin;

impl Plugin for PapermanPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Options::default());
        app.add_event::<NextPapermanAnimation>();
        app.add_systems(
            OnEnter(GameState::GameLoading),
            (prepare_paperman_system, setup_animation_system),
        );
        app.add_systems(
            OnExit(GameState::GameLoading),
            update_paperman_transform_system,
        );
        app.add_systems(
            Update,
            (
                update_paperman_transform_system,
                controller::update_movement_system,
                animation::update_animation_system,
            )
                .run_if(in_state(GameState::GameRunning)),
        );
        // app.add_systems(
        //     OnEnter(GameState::GameRunning),
        //     update_paperman_transform_system,
        // );
    }
}

#[derive(Component, Debug)]
pub struct Paperman;

/// Paperman position
#[derive(Component, Debug)]
pub struct PapermanPosition(Vec3);

/// Paperman rotation (in degrees around the y axis)
#[derive(Component, Debug, PartialEq, Clone, Default)]
pub enum PapermanRotation {
    Left,
    #[default]
    Right,
}

impl PapermanRotation {
    const ROTATION_LEFT: f32 = 90.0;
    const ROTATION_RIGHT: f32 = 270.0;

    pub fn as_quat(&self) -> Quat {
        match self {
            Self::Left => Quat::from_axis_angle(Vec3::Y, Self::ROTATION_LEFT.to_radians()),
            Self::Right => Quat::from_axis_angle(Vec3::Y, Self::ROTATION_RIGHT.to_radians()),
        }
    }

    pub fn forward(&self) -> Vec3 {
        match self {
            Self::Left => Vec3::X,
            Self::Right => Vec3::X * -1.0,
        }
    }
}

/// Paperman velocity
#[derive(Component)]
pub struct PapermanVelocity(Vec3);

#[derive(WorldQuery)]
struct PapermanTransformQuery {
    entity: Entity,
    position: &'static PapermanPosition,
    rotation: &'static PapermanRotation,
}

fn transform_from_player(position: &PapermanPosition, rotation: &PapermanRotation) -> Transform {
    Transform::from_translation(position.0)
        .with_scale(Vec3::splat(2.0))
        .with_rotation(rotation.as_quat())
}

fn prepare_paperman_system(
    mut commands: Commands,
    building: Res<BuildingResource>,
    paperman: Res<PapermanResource>,
) {
    commands
        .spawn((
            Paperman,
            PapermanPosition(building.player),
            PapermanRotation::Right,
            PapermanVelocity(Vec3::ZERO),
            PapermanAnimationQueue::default(),
            SceneBundle {
                scene: paperman.scene.clone(),
                ..Default::default()
            },
        ))
        .with_children(|children| {
            children.spawn(PointLightBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 3.0, 0.0)),
                ..Default::default()
            });
        });
}

fn update_paperman_transform_system(
    mut commands: Commands,
    mut query: Query<PapermanTransformQuery>,
    camera: Query<Entity, With<Camera3d>>,
) {
    if let Ok(result) = query.get_single_mut() {
        let transform = transform_from_player(result.position, result.rotation);

        commands.entity(result.entity).insert(transform);

        if let Ok(camera) = camera.get_single() {
            commands.entity(camera).insert(
                Transform::from_translation(transform.translation + Vec3::new(0.0, 3.0, -28.0))
                    .looking_at(transform.translation + Vec3::new(0.0, 2.0, 0.0), -Vec3::Y),
            );
        }
    }
}
