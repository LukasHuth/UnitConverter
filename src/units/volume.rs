use std::slice::Iter;

use super::Unit;

#[derive(Clone, Copy)]
pub enum UNITS {
    CmCube,
    MCube,
    Liter,
    InchCube,
    FootCube,
    UsGallon,
    ImperialGallon,
}
pub enum UnitContainer {
    CmCube(CmCube),
    MCube(MCube),
    Liter(Liter),
    InchCube(InchCube),
    FootCube(FootCube),
    UsGallon(UsGallon),
    ImperialGallon(ImperialGallon),
}
impl UnitContainer {
    pub fn new(unit: UNITS, value: f64) -> Self {
        match unit {
            UNITS::CmCube => UnitContainer::CmCube(CmCube(value)),
            UNITS::MCube => UnitContainer::MCube(MCube(value)),
            UNITS::Liter => UnitContainer::Liter(Liter(value)),
            UNITS::InchCube => UnitContainer::InchCube(InchCube(value)),
            UNITS::FootCube => UnitContainer::FootCube(FootCube(value)),
            UNITS::UsGallon => UnitContainer::UsGallon(UsGallon(value)),
            UNITS::ImperialGallon => UnitContainer::ImperialGallon(ImperialGallon(value)),
        }
    }
}

pub struct CmCube(pub f64);
pub struct MCube(pub f64);
pub struct Liter(pub f64);
pub struct InchCube(pub f64);
pub struct FootCube(pub f64);
pub struct UsGallon(pub f64);
pub struct ImperialGallon(pub f64);

mod conversions;

impl UNITS {
    pub fn iterator() -> Iter<'static, UNITS> {
        use UNITS::*;
        static DIRECTIONS: [UNITS;7] = [CmCube,MCube,Liter,InchCube,FootCube,UsGallon,ImperialGallon];
        DIRECTIONS.iter()
    }
}
impl Unit for UNITS {
    fn get_type(n: u32) -> Self {
        use UNITS::*;
        static DIRECTIONS: [UNITS;7] = [CmCube,MCube,Liter,InchCube,FootCube,UsGallon,ImperialGallon];
        DIRECTIONS[n as usize]
    }
    fn to_string(&self) -> String {
        match self {
            Self::CmCube => String::from("cmcube"),
            Self::MCube => String::from("mcube"),
            Self::Liter => String::from("liter"),
            Self::InchCube => String::from("inchcube"),
            Self::FootCube => String::from("footcube"),
            Self::UsGallon => String::from("us_gallon"),
            Self::ImperialGallon => String::from("imperial_gallon"),
        }
    }
    fn by_name(name: &str) -> Self {
        match name {
            "cmcube" => Self::CmCube,
            "mcube" => Self::MCube,
            "liter" => Self::Liter,
            "inchcube" => Self::InchCube,
            "footcube" => Self::FootCube,
            "us_gallon" => Self::UsGallon,
            "imperial_gallon" => Self::ImperialGallon,
            _ => panic!("unit with name '{}' does not exist", name)
        }
    }
}
