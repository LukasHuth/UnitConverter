use std::slice::Iter;

#[derive(Clone, Copy)]
pub enum UNITS {
    CELSIUS,
    FAHRENHEIT,
    KELVIN,
}
pub enum UnitContainer {
    CELSIUS(CELSIUS),
    FAHRENHEIT(FAHRENHEIT),
    KELVIN(KELVIN),
}

impl UnitContainer {
    pub fn new(unit: UNITS, value: f64) -> Self {
        match unit {
            UNITS::KELVIN => Self::KELVIN(KELVIN(value)),
            UNITS::FAHRENHEIT => Self::FAHRENHEIT(FAHRENHEIT(value)),
            UNITS::CELSIUS => Self::CELSIUS(CELSIUS(value)),
        }
    }
}
pub struct CELSIUS(pub f64);
pub struct FAHRENHEIT(pub f64);
pub struct KELVIN(pub f64);

mod conversions;

impl UNITS {
    pub fn iterator() -> Iter<'static, UNITS> {
        use UNITS::*;
        static DIRECTIONS: [UNITS;3] = [CELSIUS,FAHRENHEIT,KELVIN];
        DIRECTIONS.iter()
    }
}
impl super::Unit for UNITS {
    fn by_name(name: &str) -> Self {
        match name {
            "kelvin" => Self::KELVIN,
            "fahrenheit" => Self::FAHRENHEIT,
            "celsius" => Self::CELSIUS,
            &_ => panic!("no unit with name '{name}'"),
        }
    }
    fn to_string(&self) -> String {
        match self {
            Self::CELSIUS => String::from("Celsius"),
            Self::KELVIN => String::from("Kelvin"),
            Self::FAHRENHEIT => String::from("Fahrenheit"),
        }
    }
    fn get_type(n: u32) -> Self {
        use UNITS::*;
        static DIRECTIONS: [UNITS;3] = [CELSIUS,FAHRENHEIT,KELVIN];
        DIRECTIONS[n as usize]
    }
}
