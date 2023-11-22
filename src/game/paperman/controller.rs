use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;

use crate::game::assets::BuildingResource;

use super::{
    animation::{PapermanAnimationFinishedEvent, PapermanAnimationState},
    PapermanDirection, PapermanPosition, PapermanVelocity,
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

#[derive(Component, Debug, Default, PartialEq)]
pub enum PapermanControllerState {
    #[default]
    Idle,
    Turning(PapermanDirection),
    Running(PapermanDirection),
}

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct PapermanControllerQuery {
    entity: Entity,
    position: &'static mut PapermanPosition,
    direction: &'static mut PapermanDirection,
    velocity: &'static mut PapermanVelocity,
    state: &'static mut PapermanControllerState,
    animation_state: &'static mut PapermanAnimationState,
}

/// Returns the direction of movement for the given input keys.
pub fn movement_direction(
    input: &Res<Input<KeyCode>>,
    left: KeyCode,
    right: KeyCode,
) -> Option<PapermanDirection> {
    if input.pressed(left) {
        Some(PapermanDirection::Left)
    } else if input.pressed(right) {
        Some(PapermanDirection::Right)
    } else {
        None
    }
}

/// Update controller state from user input
pub fn update_input_state_system(
    mut query: Query<PapermanControllerQuery>,
    mut player: Query<(Entity, &mut Transform, &mut AnimationPlayer)>,
    input: Res<Input<KeyCode>>,
    options: Res<Options>,
) {
    if let Ok(mut paperman) = query.get_single_mut() {
        let direction = movement_direction(&input, options.keymap.left, options.keymap.right);
        let next_state = if let Some(direction) = direction {
            // if the character is already facing in this direction switch to the running state
            if *paperman.direction == direction {
                PapermanControllerState::Running(direction)
            }
            // if not switch to the turning state
            else {
                PapermanControllerState::Turning(direction)
            }
        }
        // no movement -> switch to idle
        else {
            PapermanControllerState::Idle
        };

        if next_state != *paperman.state {
            // can't go from turning to idle
            if let PapermanControllerState::Turning(_) = *paperman.state {
                if next_state == PapermanControllerState::Idle {
                    return;
                }
            }

            *paperman.state = next_state;
            info!("paperman.state = {:?}", paperman.state);
        }
    }
}

/// Update animation state from controller state
pub fn update_animation_state_system(
    mut query: Query<PapermanControllerQuery, Changed<PapermanControllerState>>,
) {
    if let Ok(mut paperman) = query.get_single_mut() {
        info!("paperman.state changed = {:?}", paperman.state);

        let next_animation_state = match *paperman.state {
            PapermanControllerState::Running(_) => {
                // TODO use velocity to decide between walking and running
                PapermanAnimationState::Walking
            }
            PapermanControllerState::Turning(_) => PapermanAnimationState::Turning,
            PapermanControllerState::Idle => PapermanAnimationState::Idle,
        };

        if next_animation_state != *paperman.animation_state {
            *paperman.animation_state = next_animation_state;
            info!("paperman.animation_state = {:?}", paperman.animation_state);
        }
    }
}

/// Listens for the finished animation event for the turning animation to complete, then turn the character
pub fn finished_turning_animation_system(
    mut query: Query<PapermanControllerQuery>,
    mut event_reader: EventReader<PapermanAnimationFinishedEvent>,
) {
    if let Ok(mut paperman) = query.get_single_mut() {
        for event in event_reader.read() {
            info!("read finished animation event: {:?}", event);
            if event.state == PapermanAnimationState::Turning {
                if let PapermanControllerState::Turning(direction) = paperman.state.as_ref() {
                    info!("turning finished, set direction to {:?}", direction);
                    // then transition to the running controller state:
                    *paperman.direction = direction.clone();
                    *paperman.state = PapermanControllerState::Running(direction.clone());
                    *paperman.animation_state = PapermanAnimationState::Walking;
                }
            }
        }
    }
}

/*
pub fn update_movement_system(
    mut commands: Commands,
    mut query: Query<PapermanControllerQuery>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    options: Res<Options>,
    building: Res<BuildingResource>,
) {
    let mut result = query.single_mut();
    let dt = time.delta_seconds();

    let direction = movement_direction(&input, options.keymap.left, options.keymap.right);

    let forward = direction.as_ref().map_or(Vec3::ZERO, |dir| dir.forward());

    // if let Some(direction) = direction.as_ref() {
    //     if *direction != *result.rotation {
    //         let next_state = if direction == &PapermanDirection::Left {
    //             PapermanAnimationState::TurnLeft
    //         } else {
    //             PapermanAnimationState::TurnRight
    //         };

    //         if next_state != result.animation_state.state {
    //             result.animation_state.set(next_state);
    //         }

    //         // commands
    //         //     .entity(result.entity)
    //         //     .insert();

    //         // wait until the animation turned the player
    //         return;
    //     }
    // }

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

    // if let Some(direction) = direction {
    //     let next_state = if direction == PapermanDirection::Left {
    //         PapermanAnimationState::Walking
    //     } else {
    //         PapermanAnimationState::Walking
    //     };

    //     if next_state != result.animation_state.state {
    //         result.animation_state.set(next_state);
    //     }
    // }

    let state = match direction {
        Some(PapermanDirection::Left) => PapermanAnimationState::Walking,
        Some(PapermanDirection::Right) => PapermanAnimationState::Walking,
        None => PapermanAnimationState::Idle,
    };
    if state != result.animation_state.state {
        result.animation_state.set(state);
    }

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
*/
