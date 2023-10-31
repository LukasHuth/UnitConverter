use crate::units::mass::*;

impl From<Kilogram> for Milligram {
    fn from(u: Kilogram) -> Self {
        Self(u.0 * 1_000_000.0)
    }
}
mass_create_from! {
    ImperialTon:Milligram,
    Tonne:Milligram,
    Gram:Milligram,
    UsTon:Milligram,
    Pound:Milligram,
    Ounce:Milligram,
}
