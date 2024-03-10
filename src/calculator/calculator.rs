use crate::{
    interface::interface_model::TargetChampion,
    tft::{game_state::GameState, tft_model::Champion},
};

pub fn calculateSlotTransistionMatrix(
    player_level: u8,
    target_champions: Vec<TargetChampion>,
    game_state: GameState,
) {
    let otherOwnedUnitsByCost: Vec<u8> = [0; 5];
}

pub fn calculateSlotTransistionMatrix() -> Vec<Vec<Double>> {}
