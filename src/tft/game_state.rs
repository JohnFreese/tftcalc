use std::cell::RefCell;
use std::mem::MaybeUninit;

use crate::interface::interface_model::UserInput;

use super::constants::{MAX_LEVEL, MAX_NUMBER_OF_PLAYERS};
use super::tft_model::{BoardState, Champion, Round, StarLevel};

struct GameState {
    pub enemy_boards: [BoardState; MAX_NUMBER_OF_PLAYERS - 1],
    pub player_board: BoardState,
    pub round: Round,
}

impl GameState {
    pub fn generate_initial_board(input: UserInput) -> Self {
        GameState {
            player_board: BoardState {
                champions: RefCell::new(
                    input
                        .target_champions
                        .iter()
                        .flat_map(|c| {
                            get_champions_by_copies(c.number_owned, c.name.clone()).into_iter()
                        })
                        .collect(),
                ),
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
}

fn get_champions_by_copies(copies: u8, champ: Champion) -> Vec<Champion> {
    let mut copies = copies;
    let mut champs = vec![];

    // there is atleast 1 3 star
    if copies / 9 > 1 {
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
    if copies / 3 > 1 {
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
mod test {
    use crate::tft::{
        game_state::get_champions_by_copies,
        sets::set11::Set11,
        tft_model::{Champion, ChampionCost, StarLevel},
    };

    #[test]
    fn champ_copies() {
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
}
