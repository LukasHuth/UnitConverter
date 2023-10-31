use crate::units::mass::*;

impl From<Kilogram> for Ounce {
    fn from(u: Kilogram) -> Self {
        Self(u.0 * 35.273961949583)
    }
}
create_from! {
    ImperialTon:Ounce,
    Tonne:Ounce,
    Gram:Ounce,
    UsTon:Ounce,
    Pound:Ounce,
    Milligram:Ounce,
}
