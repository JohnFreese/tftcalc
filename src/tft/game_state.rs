use crate::interface::interface_model::UserInput;

use super::tft_model::{Round, BoardState};
use super::constants::{MAX_LEVEL, MAX_NUMBER_OF_PLAYERS};

struct GameState {
    enemy_boards: [BoardState; MAX_NUMBER_OF_PLAYERS - 1],
    player_board: BoardState,
    round: Round,
}

impl GameState {
    pub fn generate_initial_board(input: UserInput) -> Self {
        
    }
}
