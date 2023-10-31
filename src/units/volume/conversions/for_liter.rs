use crate::units::volume::*;

impl From<CmCube> for Liter {
    fn from(u: CmCube) -> Self {
        Self(u.0 * 0.001)
    }
}
impl From<MCube> for Liter {
    fn from(u: MCube) -> Self {
        Self(u.0 * 1000.0)
    }
}
impl From<InchCube> for Liter {
    fn from(u: InchCube) -> Self {
        Self(u.0 / 61.024)
    }
}
impl From<FootCube> for Liter {
    fn from(u: FootCube) -> Self {
        Self(u.0 * 28.31685)
    }
}
impl From<UsGallon> for Liter {
    fn from(u: UsGallon) -> Self {
        Self(u.0 * 3.79)
    }
}
impl From<ImperialGallon> for Liter {
    fn from(u: ImperialGallon) -> Self {
        Self(u.0 * 4.55)
    }
}
