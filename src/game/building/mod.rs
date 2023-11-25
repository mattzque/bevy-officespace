use bevy::prelude::*;

use super::{assets::BuildingResource, states::GameState};

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameLoading), prepare_scenes_system);
    }
}

#[derive(Component, Debug)]
pub enum BuildingScene {
    Left,
    LeftRight,
    Right,
}

const BUILDING_LR_WIDTH: f32 = 65.0;
const BUILDING_L_WIDTH: f32 = 68.8427;
const BUILDING_R_WIDTH: f32 = 66.8975;

fn prepare_scenes_system(mut commands: Commands, building: Res<BuildingResource>) {
    commands.spawn((
        SceneBundle {
            scene: building.scene_ropen.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        },
        BuildingScene::Right,
    ));
    commands.spawn((
        SceneBundle {
            scene: building.scene_lropen.clone(),
            transform: Transform::from_translation(Vec3::new(BUILDING_R_WIDTH, 0.0, 0.0)),
            ..Default::default()
        },
        BuildingScene::LeftRight,
    ));
    // commands.spawn((
    //     SceneBundle {
    //         scene: building.scene_lopen.clone(),
    //         transform: Transform::from_translation(Vec3::new(
    //             BUILDING_R_WIDTH + BUILDING_LR_WIDTH,
    //             0.0,
    //             0.0,
    //         )),
    //         ..Default::default()
    //     },
    //     BuildingScene::Left,
    // ));
}
