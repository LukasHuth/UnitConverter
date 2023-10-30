use crate::units::length::*;

impl From<KM> for M {
    fn from(unit: KM) -> Self {
        Self(unit.0 * 1000.0)
    }
}
impl From<CM> for M {
    fn from(unit: CM) -> Self {
        Self(unit.0 / 100.0)
    }
}
impl From<MM> for M {
    fn from(unit: MM) -> Self {
        Self(unit.0 / 1000.0)
    }
}
impl From<INCH> for M {
    fn from(unit: INCH) -> Self {
        Self(unit.0 / 39.37)
    }
}
impl From<FOOT> for M { fn from(unit: FOOT) -> Self { M::from(INCH::from(unit)) } }
impl From<YARD> for M { fn from(unit: YARD) -> Self { M::from(INCH::from(unit)) } }
impl From<MILE> for M { fn from(unit: MILE) -> Self { M::from(INCH::from(unit)) } }
