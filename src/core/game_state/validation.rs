use super::error::Error;
use super::rules::CHANGEABLE_STATUS;
use super::GameStateMoves;
use super::GameStateStatus;

pub fn validate_changeability_status(status: GameStateStatus) -> Result<(), Error> {
    if status != CHANGEABLE_STATUS {
        Err(Error::UnchangeableStatus(status))
    } else {
        Ok(())
    }
}

pub fn validate_changeability_moves(moves: GameStateMoves) -> Result<(), Error> {
    if moves.count() == moves.limit() {
        Err(Error::UnchangeableMoves(moves))
    } else {
        Ok(())
    }
}
