use crate::tft::tft_model::{Champion, ChampionCost, StarLevel};

use super::set_model::Set;

pub const SET_11_CHAMPIONS: Vec<Champion> = vec![
    Champion {
        name: "Aatrox".into(),
        cost: ChampionCost::TwoCost,
        ..Champion::default()
    },
    Champion {
        name: "Ahri".into(),
        cost: ChampionCost::OneCost,
        ..Champion::default()
    },
    Champion {
        name: "Alune".into(),
        cost: ChampionCost::ThreeCost,
        ..Champion::default()
    },
];
pub const CHAMPION_NAMES: Vec<String> = SET_11_CHAMPIONS.iter().map(|c| c.name).collect();

struct Set11;
impl Set for Set11 {
    fn get_set_champions() -> Vec<crate::tft::tft_model::Champion> {
        SET_11_CHAMPIONS
    }

    fn get_champion_by_name(name: String) -> Option<Champion> {
        let index = CHAMPION_NAMES.iter().position(|&v| v == name);
        index
            .and_then(|i| SET_11_CHAMPIONS.get(i))
            .map(|c| *c.clone())
    }
}
