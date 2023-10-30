use crate::units::length::*;

impl From<INCH> for FOOT {
    fn from(u: INCH) -> Self {
        Self(u.0 / 12.0)
    }
}

impl From<MM> for FOOT { fn from(u: MM) -> Self { FOOT::from(INCH::from(u)) } }
impl From<CM> for FOOT { fn from(u: CM) -> Self { FOOT::from(INCH::from(u)) } }
impl From<M> for FOOT { fn from(u: M) -> Self { FOOT::from(INCH::from(u)) } }
impl From<KM> for FOOT { fn from(u: KM) -> Self { FOOT::from(INCH::from(u)) } }
impl From<MILE> for FOOT { fn from(u: MILE) -> Self { FOOT::from(INCH::from(u)) } }
impl From<YARD> for FOOT { fn from(u: YARD) -> Self { FOOT::from(INCH::from(u)) } }

