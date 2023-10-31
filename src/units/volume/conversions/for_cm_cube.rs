use crate::units::volume::*;

impl From<Liter> for CmCube {
    fn from(u: Liter) -> Self {
        Self(u.0 * 1000.0)
    }
}

volume_create_from! {
    MCube:CmCube,
    InchCube:CmCube,
    FootCube:CmCube,
    UsGallon:CmCube,
    ImperialGallon:CmCube,
}
