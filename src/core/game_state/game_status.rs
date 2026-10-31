#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum GameStatus {
    #[default]
    InProgress,
    Win,
    Lose,
}
