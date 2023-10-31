use crate::units::mass::*;

impl From<Kilogram> for ImperialTon {
    fn from(u: Kilogram) -> Self {
        Self(u.0 * 0.0009842065276111)
    }
}
create_from! {
    Milligram:ImperialTon,
    Tonne:ImperialTon,
    Gram:ImperialTon,
    UsTon:ImperialTon,
    Pound:ImperialTon,
    Ounce:ImperialTon,
}
