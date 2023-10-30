use crate::units::length::*;

impl From<M> for KM {
    fn from(u: M) -> Self {
        Self(u.0 / 1000.0)
    }
}
impl From<MM> for KM { fn from(u: MM) -> Self { KM::from(M::from(u)) } }
impl From<CM> for KM { fn from(u: CM) -> Self { KM::from(M::from(u)) } }
impl From<INCH> for KM { fn from(u: INCH) -> Self { KM::from(M::from(u)) } }
impl From<FOOT> for KM { fn from(u: FOOT) -> Self { KM::from(M::from(u)) } }
impl From<MILE> for KM { fn from(u: MILE) -> Self { KM::from(M::from(u)) } }
impl From<YARD> for KM { fn from(u: YARD) -> Self { KM::from(M::from(u)) } }
