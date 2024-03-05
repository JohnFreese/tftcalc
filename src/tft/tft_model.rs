use std::cell::RefCell;

use super::constants::{MAX_NUMBER_OF_PLAYERS, MAX_LEVEL};

pub enum ChampionCost {
    OneCost,
    TwoCost,
    ThreeCost,
    FourCost,
    FiveCost
}

pub enum StarLevel {
    OneStar,
    TwoStar,
    ThreeStar
}

pub struct Champion {
    pub cost: ChampionCost,
    pub name: String,
    pub star: StarLevel
}

pub enum PlayerState {
    Dead,
    Alive
}

pub struct BoardState {
    pub champions: RefCell<Vec<Champion>>,
    level: u8,
    pub state: PlayerState,
    pub player_name: String
}

pub struct PlayerLevelOutOfBoundsError {
    attempted_level: u8
}

impl BoardState {
    pub fn new(player_name: String) -> BoardState {
        BoardState {
            champions: RefCell::new(Vec::new()),
            level: 1,
            state: PlayerState::Alive,
            player_name
        }
    }

    pub fn set_level(mut self, attempted_level: u8) -> Result<Self, PlayerLevelOutOfBoundsError>  {
       match attempted_level {
           1..=MAX_LEVEL => {
               self.level = attempted_level;
               Ok(self)
           }
           _ => Err(PlayerLevelOutOfBoundsError{ attempted_level }),
       }
    } 

    pub fn get_level(&self) -> u8 {
        self.level
    }
}

struct GameState {
    enemy_boards: [BoardState; MAX_NUMBER_OF_PLAYERS - 1],
    player_board: BoardState,
    round: Round
}

pub enum Round {
    OneOne,
    OneTwo,
    OneThree,
    OneFour,

    TwoOne,
    TwoTwo,
    TwoThree,
    TwoFour,
    TwoFive,
    TwoSix,
    TwoSeven,

    ThreeOne,
    ThreeTwo,
    ThreeThree,
    ThreeFour,
    ThreeFive,
    ThreeSix,
    ThreeSeven,

    FourOne,
    FourTwo,
    FourThree,
    FourFour,
    FourFive,
    FourSix,
    FourSeven,

    FiveOne,
    FiveTwo,
    FiveThree,
    FiveFour,
    FiveFive,
    FiveSix,
    FiveSeven,

    SixOne,
    SixTwo,
    SixThree,
    SixFour,
    SixFive,
    SixSix,
    SixSeven,

    SevenOne,
    SevenTwo,
    SevenThree,
    SevenFour,
    SevenFive,
    SevenSix,
    SevenSeven,
}
