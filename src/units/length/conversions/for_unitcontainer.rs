use crate::units::length::*;

macro_rules! from_unitcontainer {
    ($unit:ident) => {
        impl From<UnitContainer> for $unit {
            fn from(uc: UnitContainer) -> Self {
                match uc {
                    UnitContainer::MM(v) => Self::from(v),
                    UnitContainer::CM(v) => Self::from(v),
                    UnitContainer::M(v) => Self::from(v),
                    UnitContainer::KM(v) => Self::from(v),
                    UnitContainer::INCH(v) => Self::from(v),
                    UnitContainer::MILE(v) => Self::from(v),
                    UnitContainer::YARD(v) => Self::from(v),
                    UnitContainer::FOOT(v) => Self::from(v),
                }
            }
        }
    }
}

from_unitcontainer!(MM);
from_unitcontainer!(CM);
from_unitcontainer!(M);
from_unitcontainer!(KM);
from_unitcontainer!(INCH);
from_unitcontainer!(MILE);
from_unitcontainer!(YARD);
from_unitcontainer!(FOOT);
