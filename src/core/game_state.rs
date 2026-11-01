pub mod error;
pub mod moves;
pub mod rule;
pub mod status;
pub mod validation;

pub use error::Error as GameStateError;
pub use moves::{Moves as GameStateMoves, MovesError as GameStateMovesError};
pub use status::Status as GameStateStatus;
use validation::{validate_changeability_moves, validate_changeability_status};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    status: GameStateStatus,
    moves: GameStateMoves,
}

impl GameState {
    pub const DEFAULT_STATUS: GameStateStatus = GameStateStatus::InProgress;

    pub fn status(&self) -> GameStateStatus {
        self.status
    }

    pub fn moves(&self) -> GameStateMoves {
        self.moves
    }

    pub fn status_mut(&mut self) -> Result<&mut GameStateStatus, GameStateError> {
        validate_changeability_status(self.status)?;
        Ok(&mut self.status)
    }

    pub fn moves_mut(&mut self) -> Result<&mut GameStateMoves, GameStateError> {
        validate_changeability_moves(self.moves)?;
        Ok(&mut self.moves)
    }

    pub fn new(status: GameStateStatus, moves: GameStateMoves) -> Self {
        Self { status, moves }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            status: Self::DEFAULT_STATUS,
            moves: GameStateMoves::default(),
        }
    }
}
