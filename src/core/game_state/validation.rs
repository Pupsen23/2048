use super::error::Error;
use super::game_status::GameStatus;
use super::moves::Moves;
use super::rules::CHANGEABLE_STATUS;

fn is_changeable_status(status: GameStatus) -> bool {
    status == CHANGEABLE_STATUS
}

fn is_changeable_moves(moves: Moves) -> bool {
    moves.count() < moves.limit()
}

pub fn validate_changeability_status(status: GameStatus) -> Result<(), Error> {
    is_changeable_status(status).ok_or(Error::UnchangeableStatus(status))
}

pub fn validate_changeability_moves(moves: Moves) -> Result<(), Error> {
    is_changeable_moves(moves).ok_or(Error::UnchangeableMoves(moves.clone()))
}
