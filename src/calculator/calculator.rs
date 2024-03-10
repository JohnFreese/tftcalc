use std::ops::Range;

use nalgebra::DMatrix;

use itertools::Itertools;

use crate::{
    interface::interface_model::TargetChampion,
    tft::{
        game_state::{GameState, UnitAmountsByCost},
        tft_model::Champion,
    },
};

pub fn calculateSlotTransistionMatrix(
    player_level: u8,
    target_champions: Vec<TargetChampion>,
    game_state: GameState,
) {
    let otherOwnedUnitsByCost = game_state.get_owned_units_by_cost();
    let matrix_size = target_champions
        .iter()
        .map(|tc| tc.get_amount_needed())
        .fold(1, |acc, cur| acc * (cur + 1));

    let transition_matrix: DMatrix<u8> = DMatrix::zeros(matrix_size.into(), matrix_size.into());

    let ranges: Vec<Vec<u8>> = target_champions
        .iter()
        .map(|tc| tc.get_amount_needed())
        .map(|amount| {
            let mut vec = Vec::new();
            for i in 0..amount {
                vec.push(i);
            }
            vec
        })
        .collect();

    let states: Vec<Vec<u8>> = ranges
        .iter()
        .reduce(|acc, cur| acc.iter().cartesian_product(cur).collect());
}
