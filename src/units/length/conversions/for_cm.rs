use crate::units::length::*;

impl From<M> for CM {
    fn from(unit: M) -> Self {
        Self(unit.0 * 100.0)
    }
}

impl From<MM> for CM { fn from(u: MM) -> Self { CM::from(M::from(u)) } }
impl From<KM> for CM { fn from(u: KM) -> Self { CM::from(M::from(u)) } }
impl From<FOOT> for CM { fn from(u: FOOT) -> Self { CM::from(M::from(u)) } }
impl From<INCH> for CM { fn from(u: INCH) -> Self { CM::from(M::from(u)) } }
impl From<MILE> for CM { fn from(u: MILE) -> Self { CM::from(M::from(u)) } }
impl From<YARD> for CM { fn from(u: YARD) -> Self { CM::from(M::from(u)) } }
// more to do
