use super::*;

impl From<CELSIUS> for KELVIN {
    fn from(u: CELSIUS) -> Self {
        Self(u.0 + 273.15)
    }
}
impl From<FAHRENHEIT> for KELVIN {
    fn from(u: FAHRENHEIT) -> Self {
        Self((u.0 - 32.0) * (5.0/9.0) + 273.15)
    }
}
impl From<KELVIN> for CELSIUS {
    fn from(u: KELVIN) -> Self {
        Self(u.0 - 273.15)
    }
}
impl From<KELVIN> for FAHRENHEIT {
    fn from(u: KELVIN) -> Self {
        Self((u.0 - 273.15) * (9.0/5.0) + 32.0)
    }
}
impl From<CELSIUS> for FAHRENHEIT {
    fn from(u: CELSIUS) -> Self { FAHRENHEIT::from(KELVIN::from(u)) }
}
impl From<FAHRENHEIT> for CELSIUS {
    fn from(u: FAHRENHEIT) -> Self { CELSIUS::from(KELVIN::from(u)) }
}
