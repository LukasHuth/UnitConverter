use crate::units::mass::*;

impl From<Kilogram> for UsTon {
    fn from(u: Kilogram) -> Self {
        Self(u.0 * 0.0011023113109244)
    }
}
create_from! {
    ImperialTon:UsTon,
    Pound:UsTon,
    Gram:UsTon,
    Tonne:UsTon,
    Ounce:UsTon,
    Milligram:UsTon,
}
