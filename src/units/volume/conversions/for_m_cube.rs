use crate::units::volume::*;

impl From<Liter> for MCube {
    fn from(u: Liter) -> Self {
        Self(u.0 * 0.001)
    }
}

volume_create_from! {
    InchCube:MCube,
    ImperialGallon:MCube,
    CmCube:MCube,
    UsGallon:MCube,
    FootCube:MCube,
}
