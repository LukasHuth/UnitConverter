use crate::units::mass::*;

impl From<Kilogram> for Pound {
    fn from(u: Kilogram) -> Self {
        Self(u.0 * 2.2046226218488)
    }
}
create_from! {
    ImperialTon:Pound,
    Tonne:Pound,
    Gram:Pound,
    UsTon:Pound,
    Ounce:Pound,
    Milligram:Pound,
}
