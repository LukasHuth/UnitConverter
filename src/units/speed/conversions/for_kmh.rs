use crate::units::speed::*;

impl From<MeterSec> for KmH {
    fn from(u: MeterSec) -> Self {
        Self(u.0 * 59.998 / 16.66467)
    }
}

speed_create_from! {
    KmH:(FootMin,MeterMin,FootSec,MilesHour,)
}
