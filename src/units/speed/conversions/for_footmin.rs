use crate::units::speed::*;

impl From<MeterSec> for FootMin {
    fn from(u: MeterSec) -> Self {
        Self(u.0 * 3.28084 * 60.0)
    }
}

speed_create_from! {
    FootMin:(FootSec,MeterMin,KmH,MilesHour,)
}
