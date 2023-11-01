use crate::units::speed::*;

impl From<MeterSec> for MilesHour {
    fn from(u: MeterSec) -> Self {
        Self(u.0 * 2.237136)
    }
}

speed_create_from! {
    MilesHour:(FootMin,KmH,FootSec,MeterMin,)
}
