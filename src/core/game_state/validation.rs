use super::game_status::GameStatus;
use super::moves::Moves;
use super::rules::CHANGEABLE_UNDER_STATUS;

pub fn is_changeable_status(status: GameStatus) -> bool {
    status == CHANGEABLE_UNDER_STATUS
}

pub fn is_changeable_moves(moves: &Moves) -> bool {
    moves.count() < moves.limit()
}
