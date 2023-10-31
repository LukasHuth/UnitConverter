use crate::units::mass::*;

impl From<Kilogram> for Gram {
    fn from(u: Kilogram) -> Self {
        Self(u.0 * 1_000.0)
    }
}
mass_create_from! {
    Milligram:Gram,
    Tonne:Gram,
    ImperialTon:Gram,
    UsTon:Gram,
    Pound:Gram,
    Ounce:Gram,
}
