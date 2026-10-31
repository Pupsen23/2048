pub mod game_status;
pub mod moves;
pub mod rules;
pub mod validation;

use game_status::GameStatus;
use moves::Moves;
use validation::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameStateError {
    StatusIsNotChangeable(GameStatus),
    MovesIsNotChangeable(Moves),
}

#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    status: GameStatus,
    moves: Moves,
}

impl GameState {
    pub fn status(&self) -> GameStatus {
        self.status
    }

    pub fn moves(&self) -> &Moves {
        &self.moves
    }

    pub fn status_mut(&mut self) -> Result<&mut GameStatus, GameStateError> {
        self.ensure_changeability()?;
        Ok(&mut self.status)
    }

    pub fn moves_mut(&mut self) -> Result<&mut Moves, GameStateError> {
        self.ensure_changeability()?;
        Ok(&mut self.moves)
    }

    pub fn new() -> Self {
        Self {
            status: GameStatus::default(),
            moves: Moves::new(),
        }
    }

    pub fn with(status: GameStatus, moves: Moves) -> Self {
        Self { status, moves }
    }

    fn ensure_changeability(&self) -> Result<(), GameStateError> {
        is_changeable_status(self.status)
            .ok_or(GameStateError::StatusIsNotChangeable(self.status))?;
        is_changeable_moves(&self.moves).ok_or(GameStateError::MovesIsNotChangeable(self.moves))?;
        Ok(())
    }
}
