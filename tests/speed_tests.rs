use seq_macro::seq;
fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i64.pow(decimals) as f64;
    (x * y).round() / y
}
macro_rules! speed_tests {
    ($($from_type:ty, ($($to_type:ty,$value:expr$(,)?)*)$(,)?)*) => {
        seq!(MASS in 1..=2 {
            paste::item! {
                $(
                    $(
                        #[test]
                        fn [< speed_test_ $from_type:lower _to_ $to_type:lower _with_ MASS >]() {
                            use unit_converter_lib::units::speed::*;
                            let input = $from_type(MASS as f64);
                            let expected = $value * MASS as f64;
                            assert_eq!(round($to_type::from(input).0, 9),round(expected, 9));
                        }
                     )*
                 )*
            }
        });
    }
}
macro_rules! speed_unitcontainer_tests {
    ($($type:ty$(,)?)*) => {
        $(
            paste::item! {
                #[test]
                fn [< speed_test_unit_container_ $type:lower >]() {
                    use unit_converter_lib::units::speed::*;
                    let unit_type = UNITS::$type;
                    let value = 1.0;
                    let uc = UnitContainer::new(unit_type, value);
                    assert_eq!(MeterSec::from(uc).0, MeterSec::from($type(value)).0);
                }
            }
         )*
    }
}
speed_tests! {
    MeterSec, (
        MeterMin, 59.998,
        KmH, 3.6003113173,
        FootSec, 3.281387242,
        FootMin, 196.8504,
        MilesHour, 2.237136
        ),
    MeterMin, (MeterSec, 0.0166700007),
    KmH, (MeterSec, 0.27780010104),
    FootSec, (MeterSec, 0.30479999024),
    FootMin, (MeterSec, 0.00507999984),
    MilesHour, (MeterSec, 0.447),
}
speed_unitcontainer_tests!{
    MeterSec,
    MeterMin,
    KmH,
    FootSec,
    FootMin,
    MilesHour
}
