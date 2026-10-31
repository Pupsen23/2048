use super::game_status::GameStatus;
use super::moves::Moves;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error {
    UnchangeableStatus(GameStatus),
    UnchangeableMoves(Moves),
}
