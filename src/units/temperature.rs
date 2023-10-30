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
    pub fn get_type(n: u32) -> Self {
        use UNITS::*;
        static DIRECTIONS: [UNITS;3] = [CELSIUS,FAHRENHEIT,KELVIN];
        DIRECTIONS[n as usize]
    }
    pub fn to_string(&self) -> String {
        match self {
            Self::CELSIUS => String::from("Celsius"),
            Self::KELVIN => String::from("Kelvin"),
            Self::FAHRENHEIT => String::from("Fahrenheit"),
        }
    }
}
