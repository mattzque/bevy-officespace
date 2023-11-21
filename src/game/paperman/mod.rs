use bevy::prelude::*;
use bevy::{animation::RepeatAnimation, ecs::query::WorldQuery};

use super::{
    assets::{BuildingResource, PapermanResource},
    states::GameState,
};

pub struct PapermanPlugin;

impl Plugin for PapermanPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameLoading), prepare_paperman_system);
        app.add_systems(
            OnExit(GameState::GameLoading),
            update_paperman_transform_system,
        );
        // app.add_systems(
        //     Update,
        //     update_paperman_transform_system.run_if(in_state(GameState::GameRunning)),
        // );
        app.add_systems(
            OnEnter(GameState::GameRunning),
            update_paperman_transform_system,
        );
    }
}

#[derive(Component, Debug)]
pub struct Paperman;

/// Paperman position
#[derive(Component, Debug)]
pub struct PapermanPosition(Vec3);

/// Paperman rotation (in degrees around the y axis)
#[derive(Component, Debug)]
pub struct PapermanRotation(f32);

#[derive(WorldQuery)]
struct PapermanTransformQuery {
    entity: Entity,
    position: &'static PapermanPosition,
    rotation: &'static PapermanRotation,
}

fn transform_from_player(position: &PapermanPosition, rotation: &PapermanRotation) -> Transform {
    Transform::from_translation(position.0)
        .with_scale(Vec3::splat(2.0))
        .with_rotation(Quat::from_axis_angle(Vec3::Y, rotation.0.to_radians()))
}

fn prepare_paperman_system(
    mut commands: Commands,
    building: Res<BuildingResource>,
    paperman: Res<PapermanResource>,
) {
    commands.spawn((
        Paperman,
        PapermanPosition(building.player),
        PapermanRotation(270.0),
        SceneBundle {
            scene: paperman.scene.clone(),
            ..Default::default()
        },
    ));
}

fn update_paperman_transform_system(
    mut commands: Commands,
    mut query: Query<PapermanTransformQuery>,
    camera: Query<Entity, With<Camera3d>>,
    paperman: Res<PapermanResource>,
    mut animation: Query<&mut AnimationPlayer>,
) {
    if let Ok(result) = query.get_single_mut() {
        let transform = transform_from_player(result.position, result.rotation);

        commands.entity(result.entity).insert(transform);

        let mut animation = animation.single_mut();
        animation.play(paperman.animations.get("kick").unwrap().clone());
        animation.set_repeat(RepeatAnimation::Forever);

        if let Ok(camera) = camera.get_single() {
            commands.entity(camera).insert(
                Transform::from_translation(transform.translation + Vec3::new(0.0, 3.0, -5.0))
                    .looking_at(transform.translation, Vec3::Y),
            );
        }
    }
}
