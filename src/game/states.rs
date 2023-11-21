use anyhow::Result;
use bevy::log::{error, info};
use bevy::prelude::*;

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    /// Schedule asset loading
    #[default]
    Init,
    /// Assets are loading
    AssetsLoading,
    /// All Assets are loaded
    AssetsLoaded,
    /// Game is initializing
    GameLoading,
    /// Game is running
    GameRunning,
}

pub fn finished_init_system(In(_): In<()>, mut state: ResMut<NextState<GameState>>) {
    info!("finished init system -> AssetsLoading");
    state.set(GameState::AssetsLoading);
}

/// Transition system for AssetsLoading to AssetsLoaded
pub fn finished_loading_system(
    In(result): In<Result<bool>>,
    mut state: ResMut<NextState<GameState>>,
) {
    info!("finished loading system -> AssetsLoaded");
    match result {
        Ok(true) => {
            state.set(GameState::AssetsLoaded);
        }
        Err(error) => {
            error!("Error loading assets: {}", error);
        }
        _ => {}
    }
}

/// Transition system for AssetsLoaded to GameLoading
pub fn finished_loaded_system(mut state: ResMut<NextState<GameState>>) {
    info!("finished loaded system -> GameLoading");
    state.set(GameState::GameLoading);
}

/// Transition system for GameLoading to GameRunning
pub fn finished_game_loading_system(mut state: ResMut<NextState<GameState>>) {
    info!("finished loaded system -> GameRunning");
    state.set(GameState::GameRunning);
}
