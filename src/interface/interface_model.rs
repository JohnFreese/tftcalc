use crate::tft::tft_model::{Champion, StarLevel};

pub struct UserInput {
    target_champions: Vec<TargetChampion>
}

pub struct TargetChampion {
    name: Champion,
    number_owned: u8,
    number_in_pool: u8,
    target_star_level: StarLevel
}
