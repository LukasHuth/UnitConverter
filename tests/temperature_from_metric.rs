fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i64.pow(decimals) as f64;
    (x * y).round() / y
}
macro_rules! metric_tests {
    ($($name:ident: $type:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            use unit_converter_lib::units::temperature::*;
            let (input, multiplicate) = $value;
            let expected = input * multiplicate;
            let meter = M(input);
            assert_eq!(round(expected, 5), round($type::from(meter).0, 5));
        }
    )*
    }
}
