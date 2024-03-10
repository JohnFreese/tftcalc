use std::cell::RefCell;
use std::mem::MaybeUninit;

use nalgebra::CsStorage;

use crate::interface::interface_model::UserInput;

use super::constants::{MAX_LEVEL, MAX_NUMBER_OF_PLAYERS};
use super::tft_model::{BoardState, Champion, ChampionCost, Round, StarLevel};
use super::tft_utils::convert_star_level_to_amount;

pub struct GameState {
    pub enemy_boards: [BoardState; MAX_NUMBER_OF_PLAYERS - 1],
    pub player_board: BoardState,
    pub round: Round,
}

impl GameState {
    pub fn generate_initial_board(input: UserInput) -> Self {
        GameState {
            player_board: BoardState {
                champions: input
                    .target_champions
                    .iter()
                    .flat_map(|c| {
                        get_champions_by_copies(c.number_owned, c.name.clone()).into_iter()
                    })
                    .collect(),
                ..BoardState::new("our guy".into(), input.level)
            },
            enemy_boards: [
                BoardState::new("player 1".into(), input.level),
                BoardState::new("player 2".into(), input.level),
                BoardState::new("player 3".into(), input.level),
                BoardState::new("player 4".into(), input.level),
                BoardState::new("player 5".into(), input.level),
                BoardState::new("player 6".into(), input.level),
                BoardState::new("player 7".into(), input.level),
            ],
            round: Round::OneOne,
        }
    }

    pub fn get_owned_units_by_cost(&self) -> UnitAmountsByCost {
        let mut owned_units_by_cost = UnitAmountsByCost {
            one_cost: 0,
            two_cost: 0,
            three_cost: 0,
            four_cost: 0,
            five_cost: 0,
        };

        for c in &self.player_board.champions {
            owned_units_by_cost.add_champion(c);
        }

        for e in &self.enemy_boards {
            for c in &e.champions {
                owned_units_by_cost.add_champion(c);
            }
        }

        owned_units_by_cost
    }
}

#[derive(Default)]
pub struct UnitAmountsByCost {
    pub one_cost: u8,
    pub two_cost: u8,
    pub three_cost: u8,
    pub four_cost: u8,
    pub five_cost: u8,
}

impl UnitAmountsByCost {
    #[inline]
    pub fn add_champion(&mut self, champion: &Champion) {
        match champion.cost {
            ChampionCost::OneCost => self.one_cost += convert_star_level_to_amount(&champion.star),
            ChampionCost::TwoCost => self.two_cost += convert_star_level_to_amount(&champion.star),
            ChampionCost::ThreeCost => {
                self.three_cost += convert_star_level_to_amount(&champion.star)
            }
            ChampionCost::FourCost => {
                self.four_cost += convert_star_level_to_amount(&champion.star)
            }
            ChampionCost::FiveCost => {
                self.five_cost += convert_star_level_to_amount(&champion.star)
            }
        }
    }
}

fn get_champions_by_copies(copies: u8, champ: Champion) -> Vec<Champion> {
    let mut copies = copies;
    let mut champs = vec![];

    // there is atleast 1 3 star
    if copies / 9 >= 1 {
        let num_3_star = (copies as f32 / 9.0).floor() as u8;
        copies = copies % 9;

        //insert 3 stars into vec
        for i in 1..=num_3_star {
            champs.push(Champion {
                star: StarLevel::ThreeStar,
                ..champ.clone()
            })
        }
    }

    // there is atleast 1 2 star
    if copies / 3 >= 1 {
        let num_2_star = (copies as f32 / 3.0).floor() as u8;
        copies = copies % 3;

        // insert 2 stars into vec
        for i in 1..=num_2_star {
            champs.push(Champion {
                star: StarLevel::TwoStar,
                ..champ.clone()
            })
        }
    }

    // insert 1 stars into vec
    for i in 1..=copies {
        champs.push(Champion {
            star: StarLevel::OneStar,
            ..champ.clone()
        })
    }

    champs
}

#[cfg(test)]
mod test_champ_copies {
    use crate::tft::{
        game_state::get_champions_by_copies,
        sets::set11::Set11,
        tft_model::{Champion, ChampionCost, StarLevel},
    };

    #[test]
    fn two_stars_and_one_stars() {
        let set11 = Set11::new();
        let num_copies = 7;
        let one_star = set11.set_11_champions.get(0).unwrap().clone();
        let two_star = Champion {
            star: StarLevel::TwoStar,
            ..one_star.clone()
        };
        let expected_champs = vec![two_star.clone(), two_star, one_star.clone()];

        assert_eq!(
            expected_champs,
            get_champions_by_copies(num_copies, one_star)
        )
    }

    #[test]
    fn three_star() {
        let set11 = Set11::new();
        let num_copies = 9;
        let one_star = set11.set_11_champions.get(0).unwrap().clone();
        let three_star = Champion {
            star: StarLevel::ThreeStar,
            ..one_star.clone()
        };
        let expected_champs = vec![three_star];

        assert_eq!(
            expected_champs,
            get_champions_by_copies(num_copies, one_star)
        )
    }

    #[test]
    fn two_stars() {
        let set11 = Set11::new();
        let num_copies = 6;
        let one_star = set11.set_11_champions.get(0).unwrap().clone();
        let two_star = Champion {
            star: StarLevel::TwoStar,
            ..one_star.clone()
        };
        let expected_champs = vec![two_star.clone(), two_star];

        assert_eq!(
            expected_champs,
            get_champions_by_copies(num_copies, one_star)
        )
    }

    #[test]
    fn one_stars() {
        let set11 = Set11::new();
        let num_copies = 2;
        let one_star = set11.set_11_champions.get(0).unwrap().clone();
        let expected_champs = vec![one_star.clone(), one_star.clone()];

        assert_eq!(
            expected_champs,
            get_champions_by_copies(num_copies, one_star)
        )
    }
}
