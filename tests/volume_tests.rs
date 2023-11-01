use seq_macro::seq;
fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i64.pow(decimals) as f64;
    (x * y).round() / y
}
//const MASS: f64 = 2.0;
macro_rules! volume_tests {
    ($($from_type:ty, ($($to_type:ty,$value:expr$(,)?)*)$(,)?)*) => {
        seq!(MASS in 1..=2 {
            paste::item! {
                $(
                    $(
                        #[test]
                        fn [< volume_test_ $from_type:lower _to_ $to_type:lower _with_ MASS >]() {
                            use unit_converter_lib::units::volume::*;
                            let input = $from_type(MASS as f64);
                            let expected = $value * MASS as f64;
                            assert_eq!(round($to_type::from(input).0, 10),round(expected, 10));
                        }
                     )*
                 )*
            }
        });
    }
}
macro_rules! volume_unitcontainer_tests {
    ($($type:ty$(,)?)*) => {
        $(
            paste::item! {
                #[test]
                fn [< volume_test_unit_container_ $type:lower >]() {
                    use unit_converter_lib::units::volume::*;
                    let unit_type = UNITS::$type;
                    let value = 1.0;
                    let uc = UnitContainer::new(unit_type, value);
                    assert_eq!(Liter::from(uc).0, Liter::from($type(value)).0);
                }
            }
         )*
    }
}
volume_tests! {
    CmCube, (
        MCube, 0.000001,
        Liter, 0.001,
        InchCube, 0.061024,
        FootCube, 0.000035,
        UsGallon, 0.000264201,
        ImperialGallon, 0.00022
        ),
    MCube, (
        CmCube, 1_000_000.0,
        Liter, 1_000.0,
        InchCube, 61024.0,
        FootCube, 35.0,
        UsGallon, 264.201,
        ImperialGallon, 220.0
        )
}
volume_unitcontainer_tests!{
    CmCube,
    MCube,
    Liter,
    InchCube,
    FootCube,
    UsGallon,
    ImperialGallon
}
