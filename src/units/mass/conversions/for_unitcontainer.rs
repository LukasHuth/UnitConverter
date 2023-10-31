use crate::units::mass::*;

macro_rules! from_unitcontainer {
    ($($unit:ident,)*) => {
        $(
        impl From<UnitContainer> for $unit {
            fn from(uc: UnitContainer) -> Self {
                match uc {
                    UnitContainer::Gram(v) => Self::from(v),
                    UnitContainer::Milligram(v) => Self::from(v),
                    UnitContainer::Kilogram(v) => Self::from(v),
                    UnitContainer::Tonne(v) => Self::from(v),
                    UnitContainer::ImperialTon(v) => Self::from(v),
                    UnitContainer::UsTon(v) => Self::from(v),
                    UnitContainer::Pound(v) => Self::from(v),
                    UnitContainer::Ounce(v) => Self::from(v),
                }
            }
        }
        )*
    }
}
from_unitcontainer! {
    Gram,
    Milligram,
    Kilogram,
    Tonne,
    ImperialTon,
    UsTon,
    Pound,
    Ounce,
}
