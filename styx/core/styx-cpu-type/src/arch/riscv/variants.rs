use super::gdb_targets::RiscVCoreDescription;
use derive_more::Display;

use crate::{
    arch::{
        riscv::{gdb_targets::RISCV_CORE_REGISTER_MAP, registers::RiscVRegister},
        ArchitectureDef, CpuRegisterBank, GdbTargetDescriptionImpl,
    },
    Arch,
};

#[derive(Default)]
pub struct RiscVCoreRegisters {}

impl CpuRegisterBank for RiscVCoreRegisters {
    fn registers(&self) -> Vec<crate::arch::CpuRegister> {
        RISCV_CORE_REGISTER_MAP.values().cloned().collect()
    }

    fn pc(&self) -> crate::arch::CpuRegister {
        RiscVRegister::Pc.register()
    }

    fn sp(&self) -> crate::arch::CpuRegister {
        RiscVRegister::X2.register()
    }
}

#[derive(Debug, PartialEq, Eq, Display, Clone, Copy)]
pub struct RV32IMAC {}

impl ArchitectureDef for RV32IMAC {
    #[doc = " The size in bits for `usize` on this architecture"]
    fn usize(&self) -> usize {
        32
    }

    #[doc = " The size in bits for `pc` on this architecture"]
    fn pc_size(&self) -> usize {
        32
    }

    #[doc = " The size in bits for `general purpose registers` on this architecture"]
    fn core_register_size(&self) -> usize {
        32
    }

    #[doc = " The size in bits for `data words` on this architecture"]
    fn data_word_size(&self) -> usize {
        32
    }

    #[doc = " The size in bits for `instruction words` on this architecture"]
    fn insn_word_size(&self) -> usize {
        32
    }

    #[doc = " The size in bits for `addresses` on this architecture"]
    fn addr_size(&self) -> usize {
        32
    }

    #[doc = " Returns an enum variant of the Architecture family"]
    #[doc = " for this specific Architecture variant"]
    fn architecture(&self) -> Arch {
        Arch::Riscv
    }

    #[doc = " Returns a struct of the registers"]
    fn registers(&self) -> Box<dyn CpuRegisterBank> {
        Box::new(RiscVCoreRegisters {})
    }

    fn gdb_target_description(&self) -> GdbTargetDescriptionImpl {
        RiscVCoreDescription::default().into()
    }
}
