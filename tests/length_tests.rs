const M_TO_MM: f64 = 1000.0;
const M_TO_CM: f64 = 100.0;
const M_TO_KM: f64 = 1.0 / 1000.0;
const M_TO_INCH: f64 = 39.37008;
const M_TO_FOOT: f64 = M_TO_INCH / 12.0;
const M_TO_MILE: f64 = M_TO_INCH / 63360.0;
const M_TO_YARD: f64 = M_TO_INCH / 36.0;

fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i64.pow(decimals) as f64;
    (x * y).round() / y
}
macro_rules! metric_tests {
    ($($name:ident: $type:ident: $value:expr,)*) => {
    $(
        paste::item! {
            #[test]
            fn [< length_test_ $name >]() {
                use unit_converter_lib::units::length::*;
                let (input, multiplicate) = $value;
                let expected = input * multiplicate;
                let meter = M(input);
                assert_eq!(round(expected, 5), round($type::from(meter).0, 5));
            }
        }
    )*
    }
}
macro_rules! length_unitcontainer_tests {
    ($($type:ty$(,)?)*) => {
        $(
            paste::item! {
                #[test]
                fn [< length_test_unit_container_ $type:lower >]() {
                    use unit_converter_lib::units::length::*;
                    let unit_type = UNITS::$type;
                    let value = 1.0;
                    let uc = UnitContainer::new(unit_type, value);
                    assert_eq!(M::from(uc).0, M::from($type(value)).0);
                }
            }
         )*
    }
}
metric_tests! {
    m_to_mm:MM: (2.0, M_TO_MM),
    m_to_cm:CM: (2.0, M_TO_CM),
    m_to_km:KM: (2.0, M_TO_KM),
    m_to_foot:FOOT: (2.0, M_TO_FOOT),
    m_to_inch:INCH: (2.0, M_TO_INCH),
    m_to_mile:MILE: (2.0, M_TO_MILE),
    m_to_yard:YARD: (2.0, M_TO_YARD),
}
length_unitcontainer_tests!{MM,CM,M,KM,FOOT,INCH,MILE,YARD}
