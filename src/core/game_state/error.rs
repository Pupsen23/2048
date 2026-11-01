use super::GameStateMoves;
use super::GameStateStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    UnchangeableStatus(GameStateStatus),
    UnchangeableMoves(GameStateMoves),
}
