fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i64.pow(decimals) as f64;
    (x * y).round() / y
}
macro_rules! metric_tests {
    ($($from_type:ident: $to_type:ident: $value:expr,$expression:tt,)*) => {
    $(
        paste::item! {
            #[test]
            fn [< temperature_test_ $from_type:lower _to_ $to_type:lower >]() {
                use unit_converter_lib::units::temperature::*;
                let input = $value;
                let test_type = $from_type(input);
                let expected = $expression(input);
                assert_eq!(round(expected, 5), round($to_type::from(test_type).0, 5));
            }
        }
    )*
    }
}
fn k_to_c(x: f64) -> f64 {
    x - 273.15
}
fn k_to_f(x: f64) -> f64 {
    (x - 273.15) * 9.0/5.0 + 32.0
}
fn f_to_c(x: f64) -> f64 {
    (x - 32.0) * 5.0/9.0
}
fn c_to_f(x: f64) -> f64 {
    x * 9.0/5.0 + 32.0
}
fn c_to_k(x: f64) -> f64 {
    x + 273.15
}
fn f_to_k(x: f64) -> f64 {
    (x - 32.0) * 5.0/9.0 + 273.15
}
metric_tests!{
    KELVIN:CELSIUS: 50.0, k_to_c ,
    KELVIN:FAHRENHEIT: 50.0, k_to_f ,
    FAHRENHEIT:CELSIUS: 50.0, f_to_c ,
    CELSIUS:FAHRENHEIT: 50.0, c_to_f ,
    CELSIUS:KELVIN: 50.0, c_to_k ,
    FAHRENHEIT:KELVIN: 50.0, f_to_k ,
}
