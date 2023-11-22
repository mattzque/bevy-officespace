use std::collections::HashMap;
use std::time::Duration;

use bevy::animation::RepeatAnimation;
use bevy::prelude::*;

use crate::game::assets::PapermanResource;

#[derive(Component, Default, Debug, Clone, PartialEq, Hash, Eq)]
pub enum PapermanAnimationState {
    Turning,
    Walking,
    Running,
    #[default]
    Idle,
}

#[derive(Debug)]
pub struct PapermanAnimationClip {
    pub handle: Handle<AnimationClip>,
    pub looped: bool,
    pub speed: f32,
    pub transition: Duration,
}

impl Default for PapermanAnimationClip {
    fn default() -> Self {
        Self {
            handle: Default::default(),
            looped: false,
            speed: 1.0,
            transition: Duration::from_millis(0),
        }
    }
}

#[derive(Resource, Default, Debug)]
pub struct PapermanAnimationResource {
    animations: HashMap<PapermanAnimationState, PapermanAnimationClip>,
}

impl PapermanAnimationResource {
    pub fn new(animations: HashMap<PapermanAnimationState, PapermanAnimationClip>) -> Self {
        Self { animations }
    }

    pub fn default_clip(&self) -> &PapermanAnimationClip {
        self.animations
            .get(&PapermanAnimationState::default())
            .unwrap()
    }

    pub fn clip_for(&self, animation_type: &PapermanAnimationState) -> &PapermanAnimationClip {
        self.animations.get(animation_type).unwrap_or(
            self.animations
                .get(&PapermanAnimationState::default())
                .unwrap(),
        )
    }
}

pub fn setup_animation_system(mut commands: Commands, paperman: Res<PapermanResource>) {
    let animations = HashMap::from([
        (
            PapermanAnimationState::Idle,
            PapermanAnimationClip {
                handle: paperman.animations.get("idle").unwrap().clone(),
                transition: Duration::from_millis(200),
                looped: true,
                ..Default::default()
            },
        ),
        (
            PapermanAnimationState::Running,
            PapermanAnimationClip {
                handle: paperman.animations.get("running").unwrap().clone(),
                looped: true,
                transition: Duration::from_millis(400),
                ..Default::default()
            },
        ),
        (
            PapermanAnimationState::Walking,
            PapermanAnimationClip {
                handle: paperman.animations.get("walking").unwrap().clone(),
                looped: true,
                transition: Duration::from_millis(400),
                ..Default::default()
            },
        ),
        (
            PapermanAnimationState::Turning,
            PapermanAnimationClip {
                handle: paperman.animations.get("turn180").unwrap().clone(),
                looped: false,
                transition: Duration::from_millis(0),
                ..Default::default()
            },
        ),
    ]);
    commands.insert_resource(PapermanAnimationResource::new(animations));
}

/// Play the clip for the current animation state
pub fn play_animation_state_system(
    query: Query<(&PapermanAnimationState, &Children), Changed<PapermanAnimationState>>,
    // should be a ancestor? confusing with scenes
    mut player: Query<(Entity, &mut AnimationPlayer)>,
    animations: Res<PapermanAnimationResource>,
) {
    let (player_entity, mut player) = player.single_mut();
    if let Ok((state, children)) = query.get_single() {
        let clip = animations.clip_for(state);
        info!("play clip {:?}", state);
        player.play_with_transition(clip.handle.clone_weak(), clip.transition);
        player.play(clip.handle.clone_weak()); // , clip.transition);
        player.set_speed(clip.speed);
        player.set_repeat(if clip.looped {
            RepeatAnimation::Forever
        } else {
            RepeatAnimation::Never
        });
    }
}

/// The event to sent when an animation clip finished playing
#[derive(Event, Debug, Default)]
pub struct PapermanAnimationFinishedEvent {
    pub state: PapermanAnimationState,
}

/// When non-repeating animations finish switch back to the default state and write an event
pub fn finish_animation_state_system(
    mut query: Query<&mut PapermanAnimationState>,
    mut player: Query<&mut AnimationPlayer>,
    animations: Res<PapermanAnimationResource>,
    mut finished_events: EventWriter<PapermanAnimationFinishedEvent>,
) {
    let player = player.single_mut();
    if let Ok(mut state) = query.get_single_mut() {
        let clip = animations.clip_for(&state);
        if player.is_playing_clip(&clip.handle) && player.is_finished() && !clip.looped {
            finished_events.send(PapermanAnimationFinishedEvent {
                state: state.clone(),
            });

            // *state = PapermanAnimationState::default();
        }
    }
}
