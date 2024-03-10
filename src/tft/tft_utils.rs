pub fn convert_star_level_to_amount(star_level: StarLevel) -> u8 {
    let base_power: u8 = 3;
    base_power.pow(star_level.into() - 1)
}
