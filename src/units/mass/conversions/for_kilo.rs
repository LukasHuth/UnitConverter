use crate::units::mass::*;

impl From<Gram> for Kilogram {
    fn from(u: Gram) -> Self {
        Self(u.0 / 1_000.0)
    }
}
impl From<Milligram> for Kilogram {
    fn from(u: Milligram) -> Self {
        Self(u.0 / 1_000_000.0)
    }
}
impl From<Tonne> for Kilogram {
    fn from(u: Tonne) -> Self {
        Self(u.0 * 1_000.0)
    }
}
impl From<ImperialTon> for Kilogram {
    fn from(u: ImperialTon) -> Self {
        Self(u.0 * 1_016.0469088)
    }
}
impl From<UsTon> for Kilogram {
    fn from(u: UsTon) -> Self {
        Self(u.0 * 907.185)
    }
}
impl From<Pound> for Kilogram {
    fn from(u: Pound) -> Self {
        Self(u.0 * 0.45359237)
    }
}
impl From<Ounce> for Kilogram {
    fn from(u: Ounce) -> Self {
        Self(u.0 * 0.028349523)
    }
}
