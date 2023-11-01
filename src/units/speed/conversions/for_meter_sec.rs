use crate::units::speed::*;

impl From<MeterMin> for MeterSec {
    fn from(u: MeterMin) -> Self {
        Self(u.0 / 59.988)
    }
}
impl From<KmH> for MeterSec {
    fn from(u: KmH) -> Self {
        Self(u.0 * 1000.0 / 60.00718 / 59.988) // -> to km -> to min -> to sec
    }
}
impl From<FootSec> for MeterSec {
    fn from(u: FootSec) -> Self {
        Self(u.0 / 3.28084)
    }
}
impl From<FootMin> for MeterSec {
    fn from(u: FootMin) -> Self {
        Self(u.0 / 60.0 / 3.28084)
    }
}
impl From<MilesHour> for MeterSec {
    fn from(u: MilesHour) -> Self {
        Self(u.0 * 0.447)
    }
}
