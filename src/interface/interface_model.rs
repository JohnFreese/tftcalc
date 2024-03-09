use crate::tft::tft_model::{Champion, StarLevel};

pub struct UserInput {
    pub target_champions: Vec<TargetChampion>,
    pub level: u8 
}

pub struct TargetChampion {
    pub name: Champion,
    pub number_owned: u8,
    pub number_in_pool: u8,
    pub target_star_level: StarLevel,
}
