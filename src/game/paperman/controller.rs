use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;

use crate::game::assets::BuildingResource;

use super::{
    animation::{NextPapermanAnimation, PapermanAnimationType},
    PapermanPosition, PapermanRotation, PapermanVelocity,
};

pub struct KeyMap {
    left: KeyCode,
    right: KeyCode,
}

#[derive(Resource)]
pub struct Options {
    keymap: KeyMap,
    acceleration: f32,
    friction: f32,
    max_speed: f32,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            keymap: KeyMap {
                left: KeyCode::Left,
                right: KeyCode::Right,
            },
            acceleration: 0.103,
            friction: 0.07,
            max_speed: 0.01,
        }
    }
}

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct PapermanControllerQuery {
    entity: Entity,
    position: &'static mut PapermanPosition,
    rotation: &'static mut PapermanRotation,
    velocity: &'static mut PapermanVelocity,
}

/// Returns the direction of movement for the given input keys.
pub fn movement_direction(
    input: &Res<Input<KeyCode>>,
    left: KeyCode,
    right: KeyCode,
) -> Option<PapermanRotation> {
    if input.pressed(left) {
        Some(PapermanRotation::Left)
    } else if input.pressed(right) {
        Some(PapermanRotation::Right)
    } else {
        None
    }
}

pub fn update_movement_system(
    mut query: Query<PapermanControllerQuery>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    options: Res<Options>,
    building: Res<BuildingResource>,
    mut animation_events: EventWriter<NextPapermanAnimation>,
    mut next_direction: Local<PapermanRotation>,
) {
    let mut result = query.single_mut();
    let dt = time.delta_seconds();

    let direction = movement_direction(&input, options.keymap.left, options.keymap.right);

    let forward = direction.as_ref().map_or(Vec3::ZERO, |dir| dir.forward());

    if let Some(direction) = direction.as_ref() {
        if *direction != *result.rotation {
            if *next_direction != *direction {
                animation_events.send(if direction == &PapermanRotation::Left {
                    NextPapermanAnimation(PapermanAnimationType::TurnLeft)
                } else {
                    NextPapermanAnimation(PapermanAnimationType::TurnRight)
                });
            }
            *next_direction = direction.clone();

            // wait until the animation turned the player
            return;
        }
    }

    let accel = forward;

    let accel: Vec3 = if accel.length() != 0.0 {
        accel.normalize() * options.acceleration
    } else {
        Vec3::ZERO
    };

    let mut velocity = result.velocity.0;

    let friction: Vec3 = if velocity.length() != 0.0 {
        velocity.normalize() * -1.0 * options.friction
    } else {
        Vec3::ZERO
    };

    velocity += accel * dt;

    // clamp within max speed
    if velocity.length() > options.max_speed {
        velocity = velocity.normalize() * options.max_speed;
    }

    let delta_friction = friction * dt;

    velocity = if (velocity + delta_friction).signum() != velocity.signum() {
        Vec3::ZERO
    } else {
        velocity + delta_friction
    };

    result.velocity.0 = velocity;
    let position = result.position.0 + velocity;

    if building.navmesh.contains_point(position) {
        result.position.0 = position;
    }
}
