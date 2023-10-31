#[macro_export]
macro_rules! speed_create_from {
    ($($from_type:ty:$to_type:ty,)*) => {
        $(
            impl From<$from_type> for $to_type {
                fn from(u: $from_type) -> Self {
                    Self::from(MeterSec::from(u))
                }
            }
         )*
    }
}
pub mod for_meter_sec;
pub mod for_meter_min;
pub mod for_kmh;
pub mod for_footsec;
pub mod for_footmin;
pub mod for_mileshour;
pub mod for_unitcontainer;
