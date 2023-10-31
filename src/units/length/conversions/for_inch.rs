use crate::units::length::*;

impl From<FOOT> for INCH {
    fn from(unit: FOOT) -> Self {
        Self(unit.0 * 12.0)
    }
}
impl From<YARD> for INCH {
    fn from(unit: YARD) -> Self {
        Self(unit.0 * 36.0)
    }
}
impl From<MILE> for INCH {
    fn from(unit: MILE) -> Self {
        Self(unit.0 * 63360.0)
    }
}
impl From<M> for INCH {
    fn from(unit: M) -> Self {
        Self(unit.0 * 39.37008)
    }
}

impl From<CM> for INCH { fn from(u: CM) -> Self { INCH::from(M::from(u)) } }
impl From<MM> for INCH { fn from(u: MM) -> Self { INCH::from(M::from(u)) } }
impl From<KM> for INCH { fn from(u: KM) -> Self { INCH::from(M::from(u)) } }
