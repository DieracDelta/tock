use kernel::common::registers::register_bitfields;
use riscv_csr::csr::RiscvCsr;

pub mod mcause;
pub mod mip;
pub mod mscratch;

#[repr(C)]
pub struct CSR {
    mip: RiscvCsr<u32, mip::mip::Register>,
    mscratch: RiscvCsr<u32, mscratch::mscratch::Register>,
    mcause: RiscvCsr<u32, mcause::mcause::Register>,
}

//const CSR_BASE: StaticRef<CSR> = unsafe { StaticRef::new(0x340 as *const CSR) };

pub const CSR: &CSR = unsafe {
    &CSR {
        mip: RiscvCsr::new(0x344),
        mscratch: RiscvCsr::new(0x340),
        mcause: RiscvCsr::new(0x342),
    }
};
