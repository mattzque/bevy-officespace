use bevy::prelude::*;

use super::{assets::BuildingResource, states::GameState};

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::GameLoading),
            prepare_render_building_system,
        );
    }
}

fn prepare_render_building_system(mut commands: Commands, building: Res<BuildingResource>) {
    commands.spawn(SceneBundle {
        scene: building.scene.clone(),
        ..Default::default()
    });
}
