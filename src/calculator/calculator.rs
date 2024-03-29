use std::ops::Range;

use nalgebra::DMatrix;

use itertools::Itertools;

use crate::{
    interface::interface_model::TargetChampion,
    tft::{
        game_state::{GameState, UnitAmountsByCost},
        tft_model::Champion,
        tft_utils::convert_star_level_to_amount,
    },
};

pub fn calculate_slot_transistion_matrix(
    player_level: u8,
    target_champions: Vec<TargetChampion>,
    game_state: GameState,
) {
    let other_owned_units_by_cost = game_state.get_owned_units_by_cost();
    let matrix_size = target_champions
        .iter()
        .map(|tc| tc.get_amount_needed())
        .fold(1, |acc, cur| acc * (cur + 1));

    let transition_matrix: DMatrix<u8> = DMatrix::zeros(matrix_size.into(), matrix_size.into());

    /// each sub vec represents the range of a given champion you can find from 0 to your desired amount
    let ranges: Vec<Vec<u8>> = target_champions
        .iter()
        .map(|tc| tc.get_amount_needed())
        .map(|amount| {
            let mut vec = Vec::new();
            for i in 0..=amount {
                vec.push(i);
            }
            vec
        })
        .collect();

    /// states is the cartesian product of the above ranges
    /// it represents every permutation of outcomes possible
    /// i.e. 2 kaisa, 0 sett
    ///      1 kaisa, 1 sett
    ///      0 kaisa, 2 seTt
    ///      etc..
    let states: Vec<Vec<u8>> = ranges
        .into_iter()
        .map(|v| v.into_iter())
        .multi_cartesian_product()
        .collect();

    states.iter().for_each(|state| {
        let index = state_index(&target_champions, state);
        let units_found = UnitAmountsByCost::default();
    })
}

// Returns a consistent multidimensional matrix index mapped onto 1D
pub fn state_index(target_champions: &Vec<TargetChampion>, indices: &Vec<u8>) -> u32 {
    let mut index: u32 = 0;
    let mut mult: u32 = 1;

    Iterator::zip(target_champions.iter(), indices.iter()).for_each(|(tc, i)| {
        index += mult * u32::from(*i);
        mult *= u32::from(convert_star_level_to_amount(&tc.target_star_level)) + 1;
    });

    index
}
