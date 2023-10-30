use crate::units::length::*;

impl From<INCH> for MILE {
    fn from(u: INCH) -> Self {
        Self(u.0 / 63360.0)
    }
}

impl From<MM> for MILE { fn from(u: MM) -> Self { MILE::from(INCH::from(u)) } }
impl From<CM> for MILE { fn from(u: CM) -> Self { MILE::from(INCH::from(u)) } }
impl From<M> for MILE { fn from(u: M) -> Self { MILE::from(INCH::from(u)) } }
impl From<KM> for MILE { fn from(u: KM) -> Self { MILE::from(INCH::from(u)) } }
impl From<FOOT> for MILE { fn from(u: FOOT) -> Self { MILE::from(INCH::from(u)) } }
impl From<YARD> for MILE { fn from(u: YARD) -> Self { MILE::from(INCH::from(u)) } }
