use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;

use self::animation::{
    setup_animation_system, PapermanAnimationFinishedEvent, PapermanAnimationState,
};
use self::controller::{Options, PapermanControllerState};

use super::{
    assets::{BuildingResource, PapermanResource},
    states::GameState,
};

mod animation;
mod controller;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum PapermanSystemSet {
    Controller,
    Animation,
    Update,
}

pub struct PapermanPlugin;

impl Plugin for PapermanPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Options::default());
        app.insert_resource(CameraZoom::default());
        app.add_event::<PapermanAnimationFinishedEvent>();
        app.add_systems(
            OnEnter(GameState::GameLoading),
            (prepare_paperman_system, setup_animation_system),
        );
        app.add_systems(
            OnExit(GameState::GameLoading),
            update_paperman_transform_system,
        );
        app.configure_sets(
            Update,
            (
                PapermanSystemSet::Controller,
                PapermanSystemSet::Update,
                PapermanSystemSet::Animation,
            )
                .chain(),
        );
        // app.add_systems(Update);
        // .run_if(in_state(GameState::GameRunning)),
        app.add_systems(
            Update,
            (
                (
                    controller::update_input_state_system,
                    controller::update_animation_state_system,
                    controller::finished_turning_animation_system,
                    controller::movement_system,
                )
                    .in_set(PapermanSystemSet::Controller),
                (zoom_camera, update_paperman_transform_system).in_set(PapermanSystemSet::Update),
                (
                    animation::play_animation_state_system,
                    animation::finish_animation_state_system,
                )
                    .in_set(PapermanSystemSet::Animation),
            )
                .chain()
                .run_if(in_state(GameState::GameRunning)),
        );
    }
}

#[derive(Resource, Default)]
pub struct CameraZoom(f32);

#[derive(Component, Debug)]
pub struct Paperman;

/// Paperman position
#[derive(Component, Debug)]
pub struct PapermanPosition(Vec3);

/// Paperman direction, left or right
#[derive(Component, Debug, PartialEq, Clone, Default)]
pub enum PapermanDirection {
    Left,
    #[default]
    Right,
}

impl PapermanDirection {
    const ROTATION_LEFT: f32 = 270.0;
    const ROTATION_RIGHT: f32 = 90.0;

    pub fn as_quat(&self) -> Quat {
        match self {
            Self::Left => Quat::from_axis_angle(Vec3::Y, Self::ROTATION_LEFT.to_radians()),
            Self::Right => Quat::from_axis_angle(Vec3::Y, Self::ROTATION_RIGHT.to_radians()),
        }
    }

    pub fn forward(&self) -> Vec3 {
        match self {
            Self::Left => -Vec3::X,
            Self::Right => Vec3::X,
        }
    }
}

/// Paperman velocity
#[derive(Component)]
pub struct PapermanVelocity(Vec3);

#[derive(WorldQuery)]
#[world_query(mutable)]
struct PapermanTransformQuery {
    entity: Entity,
    position: &'static PapermanPosition,
    rotation: &'static PapermanDirection,
    transform: &'static mut Transform,
}

fn transform_from_player(position: &PapermanPosition, rotation: &PapermanDirection) -> Transform {
    Transform::from_translation(position.0)
        .with_scale(Vec3::splat(2.0))
        .with_rotation(rotation.as_quat())
}

fn prepare_paperman_system(
    mut commands: Commands,
    building: Res<BuildingResource>,
    paperman: Res<PapermanResource>,
) {
    commands.spawn((
        Paperman,
        PapermanPosition(building.tracks.first().unwrap().first() + (Vec3::X * 3.0)),
        PapermanDirection::Right,
        PapermanVelocity(Vec3::ZERO),
        PapermanControllerState::default(),
        PapermanAnimationState::default(),
        SceneBundle {
            scene: paperman.scene.clone(),
            ..Default::default()
        },
    ));
}

fn zoom_camera(keyboard_input: Res<Input<KeyCode>>, mut zoom: ResMut<CameraZoom>, time: Res<Time>) {
    let dt = time.delta_seconds();
    const ZOOM_SPEED: f32 = 23.0;
    if keyboard_input.pressed(KeyCode::Up) {
        zoom.0 += ZOOM_SPEED * dt;
    } else if keyboard_input.pressed(KeyCode::Down) {
        zoom.0 -= ZOOM_SPEED * dt;
    }
}

fn update_paperman_transform_system(
    mut commands: Commands,
    mut query: Query<PapermanTransformQuery>,
    camera: Query<Entity, With<Camera3d>>,
    zoom: Res<CameraZoom>,
) {
    if let Ok(mut result) = query.get_single_mut() {
        let transform = transform_from_player(result.position, result.rotation);

        *result.transform = transform;

        if let Ok(camera) = camera.get_single() {
            commands.entity(camera).insert(
                Transform::from_translation(
                    transform.translation + Vec3::new(0.0, 3.0, 28.0 + zoom.0),
                )
                .looking_at(transform.translation + Vec3::new(0.0, 2.0, 0.0), -Vec3::Y),
            );
        }
    }
}
