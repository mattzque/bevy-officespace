use std::collections::{HashMap, VecDeque};

use bevy::animation::RepeatAnimation;
use bevy::prelude::*;

use crate::game::assets::PapermanResource;

#[derive(Component, Default, Debug)]
pub struct PapermanAnimationQueue(VecDeque<PapermanAnimationType>);

#[derive(Default, Debug, Clone, PartialEq, Hash, Eq)]
pub enum PapermanAnimationType {
    TurnLeft,
    TurnRight,
    Walking,
    Running,
    #[default]
    Idle,
}

#[derive(Event)]
pub struct NextPapermanAnimation(pub PapermanAnimationType);

#[derive(Default, Debug)]
pub struct PapermanAnimationClip {
    pub handle: Handle<AnimationClip>,
    pub looped: bool,
    // timings, mirror, etc.
}

#[derive(Resource, Default, Debug)]
pub struct PapermanAnimationResource {
    animations: HashMap<PapermanAnimationType, PapermanAnimationClip>,
}

impl PapermanAnimationResource {
    pub fn new(animations: HashMap<PapermanAnimationType, PapermanAnimationClip>) -> Self {
        Self { animations }
    }

    pub fn default_clip(&self) -> &PapermanAnimationClip {
        self.animations
            .get(&PapermanAnimationType::default())
            .unwrap()
    }

    pub fn clip_for(&self, animation_type: &PapermanAnimationType) -> &PapermanAnimationClip {
        self.animations.get(animation_type).unwrap_or(
            self.animations
                .get(&PapermanAnimationType::default())
                .unwrap(),
        )
    }
}

pub fn setup_animation_system(mut commands: Commands, paperman: Res<PapermanResource>) {
    let animations = HashMap::from([
        (
            PapermanAnimationType::Idle,
            PapermanAnimationClip {
                handle: paperman.animations.get("idle").unwrap().clone(),
                looped: true,
            },
        ),
        (
            PapermanAnimationType::Running,
            PapermanAnimationClip {
                handle: paperman.animations.get("running").unwrap().clone(),
                looped: true,
            },
        ),
        (
            PapermanAnimationType::Walking,
            PapermanAnimationClip {
                handle: paperman.animations.get("walking").unwrap().clone(),
                looped: true,
            },
        ),
        (
            PapermanAnimationType::TurnLeft,
            PapermanAnimationClip {
                handle: paperman.animations.get("turn180").unwrap().clone(),
                looped: true,
            },
        ),
        (
            PapermanAnimationType::TurnRight,
            PapermanAnimationClip {
                handle: paperman.animations.get("turn180").unwrap().clone(),
                looped: true,
            },
        ),
    ]);
    commands.insert_resource(PapermanAnimationResource::new(animations));
}

pub fn update_animation_system(
    mut player: Query<&mut AnimationPlayer>,
    animations: Res<PapermanAnimationResource>,
    mut events: EventReader<NextPapermanAnimation>,
) {
    let mut player = player.single_mut();

    if let Some(next_animation) = events.read().next() {
        let clip = animations.clip_for(&next_animation.0);
        player.play(clip.handle.clone_weak());
    } else if player.is_finished() {
        let clip = animations.default_clip();
        player.play(clip.handle.clone_weak());
        player.set_repeat(RepeatAnimation::Forever);
    }
}
