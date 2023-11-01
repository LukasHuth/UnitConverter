use crate::units::mass::*;

impl From<Gram> for Kilogram {
    fn from(u: Gram) -> Self {
        Self(u.0 * 0.001)
    }
}
impl From<Milligram> for Kilogram {
    fn from(u: Milligram) -> Self {
        Self(u.0 * 0.000_001)
    }
}
impl From<Tonne> for Kilogram {
    fn from(u: Tonne) -> Self {
        Self(u.0 * 1_000.0)
    }
}
impl From<ImperialTon> for Kilogram {
    fn from(u: ImperialTon) -> Self {
        Self(u.0 * 1_016.0)
    }
}
impl From<UsTon> for Kilogram {
    fn from(u: UsTon) -> Self {
        Self(u.0 * 907.2)
    }
}
impl From<Pound> for Kilogram {
    fn from(u: Pound) -> Self {
        Self(u.0 / 2.204586)
    }
}
impl From<Ounce> for Kilogram {
    fn from(u: Ounce) -> Self {
        Self(u.0 * 0.02835)
    }
}
