//use riscv_csr::csr::{ReadOnlyRiscvCsr, ReadWriteRiscvCsr, WriteOnlyRiscvCsr};
use riscv_csr::csr::ReadWriteRiscvCsr;

pub mod mcause;
pub mod mepc;
pub mod mie;
pub mod mip;
pub mod mscratch;
pub mod mtval;
pub mod mtvec;
pub mod pmpaddr;
pub mod pmpconfig;

#[repr(C)]
pub struct CSR {
    pmpcfg0: ReadWriteRiscvCsr<u32, pmpconfig::pmpcfg::Register>,
    pmpcfg1: ReadWriteRiscvCsr<u32, pmpconfig::pmpcfg::Register>,
    pmpcfg2: ReadWriteRiscvCsr<u32, pmpconfig::pmpcfg::Register>,
    pmpcfg3: ReadWriteRiscvCsr<u32, pmpconfig::pmpcfg::Register>,
    pmpaddr0: ReadWriteRiscvCsr<u32, pmpaddr::pmpaddr::Register>,
    pmpaddr1: ReadWriteRiscvCsr<u32, pmpaddr::pmpaddr::Register>,
    pmpaddr2: ReadWriteRiscvCsr<u32, pmpaddr::pmpaddr::Register>,
    pmpaddr3: ReadWriteRiscvCsr<u32, pmpaddr::pmpaddr::Register>,
    pmpaddr4: ReadWriteRiscvCsr<u32, pmpaddr::pmpaddr::Register>,
    pmpaddr5: ReadWriteRiscvCsr<u32, pmpaddr::pmpaddr::Register>,
    pmpaddr6: ReadWriteRiscvCsr<u32, pmpaddr::pmpaddr::Register>,
    pmpaddr7: ReadWriteRiscvCsr<u32, pmpaddr::pmpaddr::Register>,
    pmpaddr8: ReadWriteRiscvCsr<u32, pmpaddr::pmpaddr::Register>,
    pmpaddr9: ReadWriteRiscvCsr<u32, pmpaddr::pmpaddr::Register>,
    pmpaddr10: ReadWriteRiscvCsr<u32, pmpaddr::pmpaddr::Register>,
    pmpaddr11: ReadWriteRiscvCsr<u32, pmpaddr::pmpaddr::Register>,
    pmpaddr12: ReadWriteRiscvCsr<u32, pmpaddr::pmpaddr::Register>,
    pmpaddr13: ReadWriteRiscvCsr<u32, pmpaddr::pmpaddr::Register>,
    pmpaddr14: ReadWriteRiscvCsr<u32, pmpaddr::pmpaddr::Register>,
    pmpaddr15: ReadWriteRiscvCsr<u32, pmpaddr::pmpaddr::Register>,
    mie: ReadWriteRiscvCsr<u32, mie::mie::Register>,
    mscratch: ReadWriteRiscvCsr<u32, mscratch::mscratch::Register>,
    mepc: ReadWriteRiscvCsr<u32, mepc::mepc::Register>,
    mcause: ReadWriteRiscvCsr<u32, mcause::mcause::Register>,
    mtval: ReadWriteRiscvCsr<u32, mtval::mtval::Register>,
    mip: ReadWriteRiscvCsr<u32, mip::mip::Register>,
    mtvec: ReadWriteRiscvCsr<u32, mtvec::mtvec::Register>,
}

// to be used to access csrs
pub const CSR: &CSR = &CSR {
    mie: ReadWriteRiscvCsr::new(0x304),
    mtvec: ReadWriteRiscvCsr::new(0x305),
    mscratch: ReadWriteRiscvCsr::new(0x340),
    mepc: ReadWriteRiscvCsr::new(0x341),
    mcause: ReadWriteRiscvCsr::new(0x342),
    mtval: ReadWriteRiscvCsr::new(0x343),
    mip: ReadWriteRiscvCsr::new(0x344),
    pmpcfg0: ReadWriteRiscvCsr::new(0x3A0),
    pmpcfg1: ReadWriteRiscvCsr::new(0x3A1),
    pmpcfg2: ReadWriteRiscvCsr::new(0x3A2),
    pmpcfg3: ReadWriteRiscvCsr::new(0x3A3),
    pmpaddr0: ReadWriteRiscvCsr::new(0x3B0),
    pmpaddr1: ReadWriteRiscvCsr::new(0x3B1),
    pmpaddr2: ReadWriteRiscvCsr::new(0x3B2),
    pmpaddr3: ReadWriteRiscvCsr::new(0x3B3),
    pmpaddr4: ReadWriteRiscvCsr::new(0x3B4),
    pmpaddr5: ReadWriteRiscvCsr::new(0x3B5),
    pmpaddr6: ReadWriteRiscvCsr::new(0x3B6),
    pmpaddr7: ReadWriteRiscvCsr::new(0x3B7),
    pmpaddr8: ReadWriteRiscvCsr::new(0x3B8),
    pmpaddr9: ReadWriteRiscvCsr::new(0x3B9),
    pmpaddr10: ReadWriteRiscvCsr::new(0x3B10),
    pmpaddr11: ReadWriteRiscvCsr::new(0x3B11),
    pmpaddr12: ReadWriteRiscvCsr::new(0x3B12),
    pmpaddr13: ReadWriteRiscvCsr::new(0x3B13),
    pmpaddr14: ReadWriteRiscvCsr::new(0x3B14),
    pmpaddr15: ReadWriteRiscvCsr::new(0x3B15),
};
