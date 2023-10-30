use crate::units::length::*;

impl From<INCH> for YARD {
    fn from(u: INCH) -> Self {
        Self(u.0 / 36.0)
    }
}

impl From<MM> for YARD { fn from(u: MM) -> Self { YARD ::from(INCH::from(u)) } }
impl From<CM> for YARD { fn from(u: CM) -> Self { YARD ::from(INCH::from(u)) } }
impl From<M> for YARD { fn from(u: M) -> Self { YARD ::from(INCH::from(u)) } }
impl From<KM> for YARD { fn from(u: KM) -> Self { YARD ::from(INCH::from(u)) } }
impl From<FOOT> for YARD { fn from(u: FOOT) -> Self { YARD ::from(INCH::from(u)) } }
impl From<MILE> for YARD { fn from(u: MILE) -> Self { YARD ::from(INCH::from(u)) } }
