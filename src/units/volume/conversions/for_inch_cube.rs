use crate::units::volume::*;

impl From<Liter> for InchCube {
    fn from(u: Liter) -> Self {
        Self(u.0 * 61.024)
    }
}

volume_create_from! {
    MCube:InchCube,
    ImperialGallon:InchCube,
    CmCube:InchCube,
    UsGallon:InchCube,
    FootCube:InchCube,
}
