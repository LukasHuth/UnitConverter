use crate::units::length::*;

impl From<M> for MM {
    fn from(u: M) -> Self {
        Self(u.0 * 1000.0)
    }
}

impl From<CM> for MM { fn from(u: CM) -> Self { MM::from(M::from(u)) } }
impl From<KM> for MM { fn from(u: KM) -> Self { MM::from(M::from(u)) } }
impl From<INCH> for MM { fn from(u: INCH) -> Self { MM::from(M::from(u)) } }
impl From<FOOT> for MM { fn from(u: FOOT) -> Self { MM::from(M::from(u)) } }
impl From<YARD> for MM { fn from(u: YARD) -> Self { MM::from(M::from(u)) } }
impl From<MILE> for MM { fn from(u: MILE) -> Self { MM::from(M::from(u)) } }
