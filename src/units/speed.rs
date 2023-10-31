use std::slice::Iter;

use super::Unit;

#[derive(Clone, Copy)]
pub enum UNITS {
    MeterSec,
    MeterMin,
    KmH,
    FootSec,
    FootMin,
    MilesHour,
}
pub enum UnitContainer {
    MeterSec(MeterSec),
    MeterMin(MeterMin),
    KmH(KmH),
    FootSec(FootSec),
    FootMin(FootMin),
    MilesHour(MilesHour),
}
impl UnitContainer {
    pub fn new(unit: UNITS, value: f64) -> Self {
        match unit {
            UNITS::MeterSec => UnitContainer::MeterSec(MeterSec(value)),
            UNITS::MeterMin => UnitContainer::MeterMin(MeterMin(value)),
            UNITS::KmH => UnitContainer::KmH(KmH(value)),
            UNITS::FootSec => UnitContainer::FootSec(FootSec(value)),
            UNITS::FootMin => UnitContainer::FootMin(FootMin(value)),
            UNITS::MilesHour => UnitContainer::MilesHour(MilesHour(value)),
        }
    }
}

pub struct MeterSec(pub f64);
pub struct MeterMin(pub f64);
pub struct KmH(pub f64);
pub struct FootSec(pub f64);
pub struct FootMin(pub f64);
pub struct MilesHour(pub f64);

mod conversions;

impl UNITS {
    pub fn iterator() -> Iter<'static, UNITS> {
        use UNITS::*;
        static DIRECTIONS: [UNITS;6] = [MeterSec,MeterMin,KmH,FootSec,FootMin,MilesHour];
        DIRECTIONS.iter()
    }
}
impl Unit for UNITS {
    fn get_type(n: u32) -> Self {
        use UNITS::*;
        static DIRECTIONS: [UNITS;6] = [MeterSec,MeterMin,KmH,FootSec,FootMin,MilesHour];
        DIRECTIONS[n as usize]
    }
    fn to_string(&self) -> String {
        match self {
            Self::MeterSec=> String::from("metersec"),
            Self::MeterMin=> String::from("metermin"),
            Self::KmH => String::from("kmh"),
            Self::FootSec=> String::from("footsec"),
            Self::FootMin => String::from("footmin"),
            Self::MilesHour => String::from("mileshour"),
        }
    }
    fn by_name(name: &str) -> Self {
        match name {
             "metersec" => Self::MeterSec,
             "metermin" => Self::MeterMin,
             "kmh" => Self::KmH,
             "footsec" => Self::FootSec,
             "footmin" => Self::FootMin,
             "mileshour" => Self::MilesHour,
            _ => panic!("unit with name '{}' does not exist", name)
        }
    }
}
