use crate::tft::tft_model::{Champion, Round};

pub trait Set {
    fn get_set_champions(&self) -> &Vec<Champion>;
    fn get_champion_by_name(&self, name: String) -> Option<Champion>;

    // lifecycle methods that we provide a default implementation for
    // the idea is we'll use these to encapsulate set mechanics
    // i.e.
    // we would overload the below function and actually change the generated_shop for headliners
    fn generate_shop_lifecycle(
        round: Round,
        player_level: u8,
        generated_shop: Vec<Champion>,
    ) -> Vec<Champion> {
        generated_shop
    }
}
