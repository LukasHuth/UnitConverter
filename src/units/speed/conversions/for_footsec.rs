use crate::units::speed::*;

impl From<MeterSec> for FootSec {
    fn from(u: MeterSec) -> Self {
        Self(u.0 * 59.998 / 18.28434)
    }
}

speed_create_from! {
    FootSec:(FootMin,MeterMin,KmH,MilesHour,)
}
