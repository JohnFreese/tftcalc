use super::tft_model::StarLevel;

pub fn convert_star_level_to_amount(star_level: &StarLevel) -> u8 {
    let base_power: u8 = 3;
    let star_val: u8 = star_level.clone().into();
    let exponent: u32 = (star_val - 1).into();

    base_power.pow(exponent)
}
