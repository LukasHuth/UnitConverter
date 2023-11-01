use crate::units::mass::*;

impl From<Kilogram> for Ounce {
    fn from(u: Kilogram) -> Self {
        Self(u.0 * 35.27337)
    }
}
mass_create_from! {
    ImperialTon:Ounce,
    Tonne:Ounce,
    Gram:Ounce,
    UsTon:Ounce,
    Pound:Ounce,
    Milligram:Ounce,
}
