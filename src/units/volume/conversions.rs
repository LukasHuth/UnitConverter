#[macro_export]
macro_rules! volume_create_from {
    ($($from_type:ty:$to_type:ty,)*) => {
        $(
            impl From<$from_type> for $to_type {
                fn from(u: $from_type) -> Self {
                    Self::from(Liter::from(u))
                }
            }
         )*
    }
}
pub mod for_cm_cube;
pub mod for_m_cube;
pub mod for_liter;
pub mod for_inch_cube;
pub mod for_foot_cube;
pub mod for_us_gallon;
pub mod for_imperial_gallon;
pub mod for_unitcontainer;
/*
pub struct CmCube(pub f64);
pub struct MCube(pub f64);
pub struct Liter(pub f64);
pub struct InchCube(pub f64);
pub struct FootCube(pub f64);
pub struct UsGallon(pub f64);
pub struct ImperialGallon(pub f64);
*/
