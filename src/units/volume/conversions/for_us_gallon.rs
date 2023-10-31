use crate::units::volume::*;

impl From<Liter> for UsGallon {
    fn from(u: Liter) -> Self {
        Self(u.0 * 0.264201)
    }
}

volume_create_from! {
    InchCube:UsGallon,
    ImperialGallon:UsGallon,
    CmCube:UsGallon,
    MCube:UsGallon,
    FootCube:UsGallon,
}
