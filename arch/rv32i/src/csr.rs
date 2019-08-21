//! CSRs

use kernel::common::registers::register_bitfields;
use kernel::common::StaticRef;

use riscv_csr::csr::RiscvCsr;

#[repr(C)]
struct CSR {
    mip: RiscvCsr<u32, MIP::Register>,
    mscratch: RiscvCsr<u32, mscratch::Register>,
}

register_bitfields![u32,
mscratch [
    scratch OFFSET(0) NUMBITS(32) []
],
MIP [
    usoft OFFSET(0) NUMBITS(1) [],
    ssoft OFFSET(1) NUMBITS(1) [],
    msoft OFFSET(3) NUMBITS(1) [],
    utimer OFFSET(4) NUMBITS(1) [],
    stimer OFFSET(5) NUMBITS(1) [],
    mtimer OFFSET(7) NUMBITS(1) [],
    uext OFFSET(8) NUMBITS(1) [],
    sext OFFSET(9) NUMBITS(1) [],
    mext OFFSET(11) NUMBITS(1) []
]
];

//const CSR_BASE: StaticRef<CSR> = unsafe { StaticRef::new(0x340 as *const CSR) };

const CSR: StaticRef<CSR> = unsafe {
    StaticRef::new(&CSR {
        mip: RiscvCsr::new(0x344, 0),
        mscratch: RiscvCsr::new(0x340, 0),
    })
};

unsafe fn test() {
    CSR.mip.set(5);
    let _ = CSR.mip.get();
}
