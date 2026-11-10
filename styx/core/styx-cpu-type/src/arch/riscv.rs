use derive_more::Display;
use enum_dispatch::enum_dispatch;

use crate::arch::ArchitectureDef;

pub mod gdb_targets;
pub mod registers;
pub mod variants;

pub use registers::RiscVRegister;

use variants::*;

#[enum_dispatch(ArchitectureVariant, ArchitectureDef)]
#[derive(Clone, Copy, Display, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(from = "RiscVVariants")]
pub enum RiscVMetaVariants {
    RV32IMAC,
}

impl From<RiscVMetaVariants> for crate::arch::backends::ArchVariant {
    fn from(value: RiscVMetaVariants) -> Self {
        crate::arch::backends::ArchVariant::RiscV(value)
    }
}

impl From<RiscVMetaVariants> for Box<dyn ArchitectureDef> {
    fn from(value: RiscVMetaVariants) -> Self {
        let inner = value;
        Box::new(inner)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize)]
pub enum RiscVVariants {
    RV32IMAC = 0,
}

impl From<RiscVVariants> for RiscVMetaVariants {
    fn from(value: RiscVVariants) -> Self {
        match value {
            RiscVVariants::RV32IMAC => RV32IMAC {}.into(),
        }
    }
}

impl From<RiscVVariants> for crate::arch::backends::ArchVariant {
    fn from(value: RiscVVariants) -> Self {
        let tmp: RiscVMetaVariants = value.into();
        tmp.into()
    }
}
