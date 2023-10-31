use crate::units::mass::*;

impl From<Kilogram> for Tonne {
    fn from(u: Kilogram) -> Self {
        Self(u.0 * 0.001)
    }
}
create_from! {
    ImperialTon:Tonne,
    Pound:Tonne,
    Gram:Tonne,
    UsTon:Tonne,
    Ounce:Tonne,
    Milligram:Tonne,
}
