use crate::tft::tft_model::{Champion, ChampionCost, StarLevel};

use super::set_model::Set;

pub struct Set11 {
    pub set_11_champions: Vec<Champion>,
    champion_names: Vec<String>,
}

impl Set11 {
    pub fn new() -> Self {
        let set_11_champions = vec![
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
        let champion_names = set_11_champions.iter().map(|c| c.name.clone()).collect();

        Set11 {
            set_11_champions,
            champion_names,
        }
    }
}

impl Set for Set11 {
    fn get_set_champions(&self) -> &Vec<Champion> {
        &self.set_11_champions
    }

    fn get_champion_by_name(&self, name: String) -> Option<Champion> {
        let index = self.champion_names.iter().position(|v| v == &name);
        index
            .and_then(|i| self.set_11_champions.get(i))
            .map(|c| c.clone())
    }
}
