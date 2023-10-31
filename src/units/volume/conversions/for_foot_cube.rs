use crate::units::volume::*;

impl From<Liter> for FootCube {
    fn from(u: Liter) -> Self {
        Self(u.0 * 0.035)
    }
}

volume_create_from! {
    MCube:FootCube,
    InchCube:FootCube,
    CmCube:FootCube,
    UsGallon:FootCube,
    ImperialGallon:FootCube,
}
