use crate::arch::backends::{ArchRegister, BasicArchRegister};
use std::{collections::BTreeMap, marker::PhantomData};
use styx_util::gdb_xml::RISCV_32_CPU;

use styx_macros::gdb_target_description;
use styx_sync::lazy_static;

use crate::arch::{riscv::registers::RiscVRegister, CpuRegister};

lazy_static! {
    pub static ref RISCV_CORE_REGISTER_MAP: BTreeMap<usize, CpuRegister> =
        BTreeMap::from([(0, RiscVRegister::X0.register())]);
}

#[gdb_target_description]
#[derive(Debug, Default)]
pub struct RiscVCoreDescription {
    #[args(
        gdb_arch_name("riscv"),
        gdb_feature_xml(RISCV_32_CPU),
        register_map(RISCV_CORE_REGISTER_MAP),
        pc_register(ArchRegister::Basic(BasicArchRegister::RiscV(RiscVRegister::Pc))),
        endianness(ArchEndian::LittleEndian)
    )]
    args: PhantomData<()>,
}
