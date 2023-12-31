use crate::units::mass::*;

impl From<Kilogram> for UsTon {
    fn from(u: Kilogram) -> Self {
        Self(u.0 / 907.2)
    }
}
mass_create_from! {
    ImperialTon:UsTon,
    Pound:UsTon,
    Gram:UsTon,
    Tonne:UsTon,
    Ounce:UsTon,
    Milligram:UsTon,
}
