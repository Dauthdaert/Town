#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy)]
pub enum GameStates {
    Splash,
    MapGeneration,
    InGame,
}
