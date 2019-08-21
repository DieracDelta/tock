use kernel::common::registers::register_bitfields;
use riscv_csr::csr::RiscvCsr;

register_bitfields![u32,
mscratch [
    scratch OFFSET(0) NUMBITS(32) []
]
];
