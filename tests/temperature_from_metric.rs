fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i64.pow(decimals) as f64;
    (x * y).round() / y
}
macro_rules! metric_tests {
    ($($name:ident: $from_type:ident: $to_type:ident: $value:expr,$expression:tt,)*) => {
    $(
        #[test]
        fn $name() {
            use unit_converter_lib::units::temperature::*;
            let input = $value;
            let test_type = $from_type(input);
            let expected = $expression(input);
            assert_eq!(round(expected, 5), round($to_type::from(test_type).0, 5));
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
    kelvin_to_celsius:KELVIN:CELSIUS: 50.0, k_to_c ,
    kelvin_to_fahrenheit:KELVIN:FAHRENHEIT: 50.0, k_to_f ,
    fahrenheit_to_celsius:FAHRENHEIT:CELSIUS: 50.0, f_to_c ,
    celsius_to_fahrenheit:CELSIUS:FAHRENHEIT: 50.0, c_to_f ,
    celsius_to_kelvin:CELSIUS:KELVIN: 50.0, c_to_k ,
    fahrenheit_to_kelvin:FAHRENHEIT:KELVIN: 50.0, f_to_k ,
}
