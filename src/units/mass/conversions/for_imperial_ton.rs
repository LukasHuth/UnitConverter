use crate::units::mass::*;

impl From<Kilogram> for ImperialTon {
    fn from(u: Kilogram) -> Self {
        Self(u.0 / 1_016.0)
    }
}
mass_create_from! {
    Milligram:ImperialTon,
    Tonne:ImperialTon,
    Gram:ImperialTon,
    UsTon:ImperialTon,
    Pound:ImperialTon,
    Ounce:ImperialTon,
}
