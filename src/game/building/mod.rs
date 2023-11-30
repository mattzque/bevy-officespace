use bevy::prelude::*;

use super::{assets::BuildingResource, states::GameState};

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameLoading), prepare_scenes_system);
    }
}

#[derive(Debug)]
pub enum BuildingSceneDirection {
    Left,
    LeftRight,
    Right,
}

#[derive(Component, Debug)]
pub struct BuildingScene {
    pub direction: BuildingSceneDirection,
    pub width: f32,
    pub offset: f32,
}

const BUILDING_LR_WIDTH: f32 = 65.3827;
const BUILDING_L_WIDTH: f32 = 68.8429;
const BUILDING_R_WIDTH: f32 = 67.251;
const BUILDING_LR_OFFSET: f32 = 1.86839;
const BUILDING_L_OFFSET: f32 = 1.86839;

fn prepare_scenes_system(mut commands: Commands, building: Res<BuildingResource>) {
    commands.spawn((
        SceneBundle {
            scene: building.scene_ropen.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        },
        BuildingScene {
            direction: BuildingSceneDirection::Right,
            width: BUILDING_R_WIDTH,
            offset: 0.0,
        },
    ));
    commands.spawn((
        SceneBundle {
            scene: building.scene_lropen.clone(),
            transform: Transform::from_translation(Vec3::new(
                BUILDING_R_WIDTH - BUILDING_LR_OFFSET,
                0.0,
                0.0,
            )),
            ..Default::default()
        },
        BuildingScene {
            direction: BuildingSceneDirection::LeftRight,
            width: BUILDING_LR_WIDTH,
            offset: BUILDING_LR_OFFSET,
        },
    ));
    commands.spawn((
        SceneBundle {
            scene: building.scene_lopen.clone(),
            transform: Transform::from_translation(Vec3::new(
                (BUILDING_R_WIDTH - BUILDING_LR_OFFSET) + (BUILDING_LR_WIDTH - BUILDING_L_OFFSET),
                0.0,
                0.0,
            )),
            ..Default::default()
        },
        BuildingScene {
            direction: BuildingSceneDirection::Left,
            width: BUILDING_L_WIDTH,
            offset: BUILDING_L_OFFSET,
        },
    ));
}
