use crate::units::volume::*;

macro_rules! from_unitcontainer {
    ($($type:ty,)*) => {
        $(
            impl From<UnitContainer> for $type {
                fn from(uc: UnitContainer) -> Self {
                    match uc {
                        UnitContainer::CmCube(v) => Self::from(v),
                        UnitContainer::MCube(v) => Self::from(v),
                        UnitContainer::Liter(v) => Self::from(v),
                        UnitContainer::InchCube(v) => Self::from(v),
                        UnitContainer::FootCube(v) => Self::from(v),
                        UnitContainer::UsGallon(v) => Self::from(v),
                        UnitContainer::ImperialGallon(v) => Self::from(v),
                    }
                }
            }
         )*
    }
}
from_unitcontainer! {
    CmCube,
    MCube,
    Liter,
    InchCube,
    FootCube,
    UsGallon,
    ImperialGallon,
}
