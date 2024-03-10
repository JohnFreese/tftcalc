pub const MAX_LEVEL: u8 = 11;
pub const MAX_NUMBER_OF_PLAYERS: usize = 8;

pub const ONE_COST_POOL: u8 = 22;
pub const ONE_COST_ODDS: ChampionTierOddsByLevel = ChampionTierOddsByLevel {
    level_one: 1.0,
    level_two: 1.0,
    level_three: 0.75,
    level_four: 0.55,
    level_five: 0.45,
    level_six: 0.30,
    level_seven: 0.19,
    level_eight: 0.18,
    level_nine: 0.10,
    level_ten: 0.05,
    level_eleven: 0.01,
};

pub const TWO_COST_POOL: u8 = 20;
pub const TWO_COST_ODDS: ChampionTierOddsByLevel = ChampionTierOddsByLevel {
    level_one: 0.0,
    level_two: 0.0,
    level_three: 0.25,
    level_four: 0.30,
    level_five: 0.33,
    level_six: 0.40,
    level_seven: 0.30,
    level_eight: 0.25,
    level_nine: 0.20,
    level_ten: 0.10,
    level_eleven: 0.02,
};

pub const THREE_COST_POOL: u8 = 17;
pub const THREE_COST_ODDS: ChampionTierOddsByLevel = ChampionTierOddsByLevel {
    level_one: 0.0,
    level_two: 0.0,
    level_three: 0.0,
    level_four: 0.15,
    level_five: 0.20,
    level_six: 0.25,
    level_seven: 0.35,
    level_eight: 0.36,
    level_nine: 0.25,
    level_ten: 0.20,
    level_eleven: 0.12,
};

pub const FOUR_COST_POOL: u8 = 10;
pub const FOUR_COST_ODDS: ChampionTierOddsByLevel = ChampionTierOddsByLevel {
    level_one: 0.0,
    level_two: 0.0,
    level_three: 0.0,
    level_four: 0.0,
    level_five: 0.02,
    level_six: 0.05,
    level_seven: 0.10,
    level_eight: 0.18,
    level_nine: 0.35,
    level_ten: 0.40,
    level_eleven: 0.50,
};

pub const FIVE_COST_POOL: u8 = 9;
pub const FIVE_COST_ODDS: ChampionTierOddsByLevel = ChampionTierOddsByLevel {
    level_one: 0.0,
    level_two: 0.0,
    level_three: 0.0,
    level_four: 0.0,
    level_five: 0.0,
    level_six: 0.0,
    level_seven: 0.01,
    level_eight: 0.08,
    level_nine: 0.15,
    level_ten: 0.25,
    level_eleven: 0.35,
};

pub struct ChampionTierOddsByLevel {
    level_one: f64,
    level_two: f64,
    level_three: f64,
    level_four: f64,
    level_five: f64,
    level_six: f64,
    level_seven: f64,
    level_eight: f64,
    level_nine: f64,
    level_ten: f64,
    level_eleven: f64,
}
