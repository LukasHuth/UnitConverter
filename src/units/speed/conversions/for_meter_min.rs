use crate::units::speed::*;

impl From<MeterSec> for MeterMin {
    fn from(u: MeterSec) -> Self {
        Self(u.0 * 59.998)
    }
}

speed_create_from! {
    MeterMin:(FootMin,KmH,FootSec,MilesHour,)
}
