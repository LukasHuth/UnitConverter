use crate::units::length::{UnitContainer, self};

pub fn to_cm(unit: UnitContainer) -> length::CM {
    use UnitContainer::*;
    match unit {
        MM(v) => length::CM::from(v),
        CM(v) => length::CM::from(v),
        M(v) => length::CM::from(v),
        KM(v) => length::CM::from(v),
        INCH(v) => length::CM::from(v),
        FOOT(v) => length::CM::from(v),
        YARD(v) => length::CM::from(v),
        MILE(v) => length::CM::from(v),
    }
}
pub fn to_km(unit: UnitContainer) -> length::KM {
    use UnitContainer::*;
    match unit {
        MM(v) => length::KM::from(v),
        CM(v) => length::KM::from(v),
        M(v) => length::KM::from(v),
        KM(v) => length::KM::from(v),
        INCH(v) => length::KM::from(v),
        FOOT(v) => length::KM::from(v),
        YARD(v) => length::KM::from(v),
        MILE(v) => length::KM::from(v),
    }
}
pub fn to_mm(unit: UnitContainer) -> length::MM {
    use UnitContainer::*;
    match unit {
        MM(v) => length::MM::from(v),
        CM(v) => length::MM::from(v),
        M(v) => length::MM::from(v),
        KM(v) => length::MM::from(v),
        INCH(v) => length::MM::from(v),
        FOOT(v) => length::MM::from(v),
        YARD(v) => length::MM::from(v),
        MILE(v) => length::MM::from(v),
    }
}
pub fn to_m(unit: UnitContainer) -> length::M {
    use UnitContainer::*;
    match unit {
        MM(v) => length::M::from(v),
        CM(v) => length::M::from(v),
        M(v) => length::M::from(v),
        KM(v) => length::M::from(v),
        INCH(v) => length::M::from(v),
        FOOT(v) => length::M::from(v),
        YARD(v) => length::M::from(v),
        MILE(v) => length::M::from(v),
    }
}
pub fn to_inch(unit: UnitContainer) -> length::INCH {
    use UnitContainer::*;
    match unit {
        MM(v) => length::INCH::from(v),
        CM(v) => length::INCH::from(v),
        M(v) => length::INCH::from(v),
        KM(v) => length::INCH::from(v),
        INCH(v) => length::INCH::from(v),
        FOOT(v) => length::INCH::from(v),
        YARD(v) => length::INCH::from(v),
        MILE(v) => length::INCH::from(v),
    }
}
pub fn to_mile(unit: UnitContainer) -> length::MILE {
    use UnitContainer::*;
    match unit {
        MM(v) => length::MILE::from(v),
        CM(v) => length::MILE::from(v),
        M(v) => length::MILE::from(v),
        KM(v) => length::MILE::from(v),
        INCH(v) => length::MILE::from(v),
        FOOT(v) => length::MILE::from(v),
        YARD(v) => length::MILE::from(v),
        MILE(v) => length::MILE::from(v),
    }
}
pub fn to_foot(unit: UnitContainer) -> length::FOOT {
    use UnitContainer::*;
    match unit {
        MM(v) => length::FOOT::from(v),
        CM(v) => length::FOOT::from(v),
        M(v) => length::FOOT::from(v),
        KM(v) => length::FOOT::from(v),
        INCH(v) => length::FOOT::from(v),
        FOOT(v) => length::FOOT::from(v),
        YARD(v) => length::FOOT::from(v),
        MILE(v) => length::FOOT::from(v),
    }
}
pub fn to_yard(unit: UnitContainer) -> length::YARD {
    use UnitContainer::*;
    match unit {
        MM(v) => length::YARD::from(v),
        CM(v) => length::YARD::from(v),
        M(v) => length::YARD::from(v),
        KM(v) => length::YARD::from(v),
        INCH(v) => length::YARD::from(v),
        FOOT(v) => length::YARD::from(v),
        YARD(v) => length::YARD::from(v),
        MILE(v) => length::YARD::from(v),
    }
}
