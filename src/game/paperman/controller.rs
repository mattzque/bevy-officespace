use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;

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
}

impl Default for Options {
    fn default() -> Self {
        Self {
            keymap: KeyMap {
                left: KeyCode::Left,
                right: KeyCode::Right,
            },
            acceleration: 10.0,
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

pub fn movement_system(
    mut query: Query<PapermanControllerQuery>,
    time: Res<Time>,
    options: Res<Options>,
) {
    let mut result = query.single_mut();
    let dt = time.delta_seconds();

    let velocity = if let PapermanControllerState::Running(direction) = result.state.as_ref() {
        direction.forward() * options.acceleration * dt
    } else {
        Vec3::ZERO
    };

    let position = result.position.0 + velocity;
    result.position.0 = position;
}
