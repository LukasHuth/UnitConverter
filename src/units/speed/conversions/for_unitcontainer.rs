use crate::units::speed::*;

macro_rules! create_from_unitcontainer {
    ($($type:ty$(,)?)*) => {
        $(
            impl From<UnitContainer> for $type {
                fn from(uc: UnitContainer) -> Self {
                    use UnitContainer as UC;
                    match uc {
                        UC::MeterSec(v) => Self::from(v),
                        UC::MeterMin(v) => Self::from(v),
                        UC::KmH(v) => Self::from(v),
                        UC::FootSec(v) => Self::from(v),
                        UC::FootMin(v) => Self::from(v),
                        UC::MilesHour(v) => Self::from(v),
                    }
                }
            }
         )*
    }
}
create_from_unitcontainer!{MeterSec,MeterMin,KmH,FootSec,FootMin,MilesHour}
