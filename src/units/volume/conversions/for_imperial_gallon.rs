use crate::units::volume::*;

impl From<Liter> for ImperialGallon {
    fn from(u: Liter) -> Self {
        Self(u.0 * 0.22)
    }
}

volume_create_from! {
    MCube:ImperialGallon,
    InchCube:ImperialGallon,
    CmCube:ImperialGallon,
    UsGallon:ImperialGallon,
    FootCube:ImperialGallon,
}
