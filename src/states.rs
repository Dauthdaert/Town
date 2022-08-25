#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy)]
pub enum GameStates {
    AssetLoading,
    MapGeneration,
    InGame,
}
