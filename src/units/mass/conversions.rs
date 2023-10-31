#[macro_export]
macro_rules! create_from {
    ($($from_type:ident:$to_type:ident,)*) => {
        $(
            impl From<$from_type> for $to_type {
                fn from(u: $from_type) -> Self {
                    Self::from(Kilogram::from(u))
                }
            }
         )*
    }
}
mod for_gram;
mod for_milligram;
mod for_kilo;
mod for_tonne;
mod for_imperial_ton;
mod for_us_ton;
mod for_pound;
mod for_ounce;
mod for_unitcontainer;
/*
pub struct Milligram(pub f64);
pub struct Gram(pub f64);
pub struct Kilogram(pub f64);
pub struct Tonne(pub f64);
pub struct ImperialTon(pub f64);
pub struct UsTon(pub f64);
pub struct Pound(pub f64);
pub struct Ounce(pub f64);
*/
