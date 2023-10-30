use std::slice::Iter;

use super::Unit;

#[derive(Clone, Copy)]
pub enum UNITS {
    MM,
    CM,
    M,
    KM,
    INCH,
    FOOT,
    YARD,
    MILE,
}
pub enum UnitContainer {
    MM(MM),
    CM(CM),
    M(M),
    KM(KM),
    INCH(INCH),
    FOOT(FOOT),
    YARD(YARD),
    MILE(MILE),
}
impl UnitContainer {
    pub fn new(unit: UNITS, value: f64) -> Self {
        match unit {
            UNITS::MM => UnitContainer::MM(MM(value)),
            UNITS::CM => UnitContainer::CM(CM(value)),
            UNITS::M => UnitContainer::M(M(value)),
            UNITS::KM => UnitContainer::KM(KM(value)),
            UNITS::INCH => UnitContainer::INCH(INCH(value)),
            UNITS::FOOT => UnitContainer::FOOT(FOOT(value)),
            UNITS::MILE => UnitContainer::MILE(MILE(value)),
            UNITS::YARD => UnitContainer::YARD(YARD(value)),
        }
    }
}

pub struct MM(pub f64);
pub struct CM(pub f64);
pub struct M(pub f64);
pub struct KM(pub f64);
pub struct INCH(pub f64);
pub struct FOOT(pub f64);
pub struct YARD(pub f64);
pub struct MILE(pub f64);

mod conversions;

impl UNITS {
    pub fn iterator() -> Iter<'static, UNITS> {
        use UNITS::*;
        static DIRECTIONS: [UNITS;8] = [MM,CM,M,KM,INCH,FOOT,YARD,MILE];
        DIRECTIONS.iter()
    }
}
impl Unit for UNITS {
    fn get_type(n: u32) -> Self {
        use UNITS::*;
        static DIRECTIONS: [UNITS;8] = [MM,CM,M,KM,INCH,FOOT,YARD,MILE];
        DIRECTIONS[n as usize]
    }
    fn to_string(&self) -> String {
        match self {
            Self::MM => String::from("mm"),
            Self::CM => String::from("cm"),
            Self::M => String::from("m"),
            Self::KM => String::from("km"),
            Self::INCH => String::from("inch"),
            Self::FOOT => String::from("foot"),
            Self::YARD => String::from("yard"),
            Self::MILE => String::from("mile"),
        }
    }
    fn by_name(name: &str) -> Self {
        match name {
            "mm" => Self::MM,
            "cm" => Self::CM,
            "m" => Self::M,
            "km" => Self::KM,
            "inch" => Self::INCH,
            "foot" => Self::FOOT,
            "yard" => Self::YARD,
            "mile" => Self::MILE,
            _ => panic!("unit with name '{}' does not exist", name)
        }
    }
}
