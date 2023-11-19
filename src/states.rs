use bevy::ecs::schedule::States;

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Init,
    AssetsLoading,
    AssetsLoaded,
    GameLoading,
}
