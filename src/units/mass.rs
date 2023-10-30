use std::slice::Iter;

#[derive(Clone, Copy)]
pub enum UNITS {
    Milligram,
    Gram,
    Kilogram,
    Tonne,
    ImperialTon,
    UsTon,
    Pound,
    Ounce,
}
pub enum UnitContainer {
    Milligram(Milligram),
    Gram(Gram),
    Kilogram(Kilogram),
    Tonne(Tonne),
    ImperialTon(ImperialTon),
    UsTon(UsTon),
    Pound(Pound),
    Ounce(Ounce),
}

impl UnitContainer {
    pub fn new(unit: UNITS, value: f64) -> Self {
        match unit {
            UNITS::Milligram => Self::Milligram(Milligram(value)),
            UNITS::Gram => Self::Gram(Gram(value)),
            UNITS::Kilogram => Self::Kilogram(Kilogram(value)),
            UNITS::Tonne => Self::Tonne(Tonne(value)),
            UNITS::ImperialTon => Self::ImperialTon(ImperialTon(value)),
            UNITS::UsTon => Self::UsTon(UsTon(value)),
            UNITS::Pound => Self::Pound(Pound(value)),
            UNITS::Ounce => Self::Ounce(Ounce(value)),
        }
    }
}
pub struct Milligram(pub f64);
pub struct Gram(pub f64);
pub struct Kilogram(pub f64);
pub struct Tonne(pub f64);
pub struct ImperialTon(pub f64);
pub struct UsTon(pub f64);
pub struct Pound(pub f64);
pub struct Ounce(pub f64);

mod conversions;

impl UNITS {
    pub fn iterator() -> Iter<'static, UNITS> {
        use UNITS::*;
        static DIRECTIONS: [UNITS;8] = [Milligram,Gram,Kilogram,Tonne,ImperialTon,UsTon,Pound,Ounce];
        DIRECTIONS.iter()
    }
}
impl super::Unit for UNITS {
    fn by_name(name: &str) -> Self {
        match name {
            "milligram" => Self::Milligram,
            "gram" => Self::Gram,
            "kilogram" => Self::Kilogram,
            "tonne" => Self::Tonne,
            "imperialton" => Self::ImperialTon,
            "uston" => Self::UsTon,
            "pound" => Self::Pound,
            "ounce" => Self::Ounce,
            &_ => panic!("no unit with name '{name}'"),
        }
    }
    fn to_string(&self) -> String {
        match self {
            Self::Milligram => String::from("Milligram"),
            Self::Gram => String::from("Gram"),
            Self::Kilogram => String::from("Kilogram"),
            Self::Tonne => String::from("Tonne"),
            Self::ImperialTon => String::from("ImperialTon"),
            Self::UsTon => String::from("UsTon"),
            Self::Pound => String::from("Pound"),
            Self::Ounce => String::from("Ounce"),
        }
    }
    fn get_type(n: u32) -> Self {
        use UNITS::*;
        static DIRECTIONS: [UNITS;8] = [Milligram,Gram,Kilogram,Tonne,ImperialTon,UsTon,Pound,Ounce];
        DIRECTIONS[n as usize]
    }
}
