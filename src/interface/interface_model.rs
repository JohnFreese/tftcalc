use crate::tft::{
    tft_model::{Champion, StarLevel},
    tft_utils::convert_star_level_to_amount,
};

pub struct UserInput {
    pub target_champions: Vec<TargetChampion>,
    pub level: u8,
}

pub struct TargetChampion {
    pub name: Champion,
    pub number_owned: u8,
    pub number_in_pool: u8,
    pub target_star_level: StarLevel,
}

impl TargetChampion {
    #[inline]
    pub fn get_amount_needed(&self) -> u8 {
        convert_star_level_to_amount(&self.target_star_level) - self.number_owned
    }
}
