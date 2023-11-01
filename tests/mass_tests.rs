use seq_macro::seq;
fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i64.pow(decimals) as f64;
    (x * y).round() / y
}
//const MASS: f64 = 2.0;
macro_rules! mass_tests {
    ($($from_type:ty, ($($to_type:ty,$value:expr,)*),)*) => {
        seq!(MASS in 0..2 {
            paste::item! {
                $(
                    $(
                        #[test]
                        fn [< mass_test_ $from_type:lower _to_ $to_type:lower _with_ MASS >]() {
                            use unit_converter_lib::units::mass::*;
                            let input = $from_type(MASS as f64);
                            let expected = $value * MASS as f64;
                            assert_eq!(round($to_type::from(input).0, 6),round(expected, 6));
                        }
                     )*
                 )*
            }
        });
    }
}
macro_rules! mass_unitcontainer_tests {
    ($($type:ty$(,)?)*) => {
        $(
            paste::item! {
                #[test]
                fn [< mass_test_unit_container_ $type:lower >]() {
                    use unit_converter_lib::units::mass::*;
                    let unit_type = UNITS::$type;
                    let value = 1.0;
                    let uc = UnitContainer::new(unit_type, value);
                    assert_eq!(Kilogram::from(uc).0, Kilogram::from($type(value)).0);
                }
            }
         )*
    }
}
mass_tests! {
    Kilogram, (
        Gram, 1_000.0,
        Tonne, 1.0/1_000.0,
        Milligram, 1_000_000.0,
        Pound, 2.2046226218488,
        Ounce, 35.273961949583,
        ImperialTon, 0.0009842065276111,
        UsTon, 0.0011023113109244,),
    Gram, (Kilogram, 1.0/1000.0,),
    Tonne, (Kilogram, 1_000.0,),
    Milligram, (Kilogram, 1.0/1_000_000.0,),
    Pound, (Kilogram, 1.0/2.2046226218488,),
    Ounce, (Kilogram, 0.028349523,),
    ImperialTon, (Kilogram, 1.0/0.0009842065276111,),
    UsTon, (Kilogram, 907.185,),
}
mass_unitcontainer_tests!{Milligram,Gram,Kilogram,Tonne,ImperialTon,UsTon,Pound,Ounce}
