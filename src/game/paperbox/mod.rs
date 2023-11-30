use bevy::prelude::*;

// use super::{
//     assets::{BuildingResource, PaperboxResource},
//     states::GameState,
// };

pub struct PaperboxPlugin;

impl Plugin for PaperboxPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_systems(OnEnter(GameState::GameLoading), prepare_paperbox_system);
        // app.add_systems(
        //     Update,
        //     debug_paperbox_system.run_if(in_state(GameState::GameRunning)),
        // );
    }
}

#[derive(Component, Debug)]
pub struct Paperbox;

// pub fn prepare_paperbox_system(
//     mut commands: Commands,
//     paperbox: Res<PaperboxResource>,
//     building: Res<BuildingResource>,
// ) {
// }

// pub fn debug_paperbox_system(query: Query<(&Transform), With<Paperbox>>, mut gizmos: Gizmos) {}
