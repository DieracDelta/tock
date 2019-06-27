/* The HiFive1a board has 512 MB of flash. The first 0x400000 is reserved for
 * the default bootloader provided by SiFive. We also reserve room for apps to
 * make all of the linker files work, but don't really support them on this
 * chip.
 */
/*NOTE THAT THIS WILL NOT WORK ON THE BOARD */
/*Rationale behind this: I can compile with the --release flag and everything will fit*/
/*if I compile with debugging symbols, the rom section is too large and the linker will fail*/
/*So for now, just so I can debug things, I've increased the length of the rom section.*/
/*The correct linker script is below:*/
/*rom (rx)  : ORIGIN = 0x20400000, LENGTH = 0x30000*/
/*prog (rx) : ORIGIN = 0x20430000, LENGTH = 512M*/

MEMORY
{
        rom (rx)  : ORIGIN = 0x20400000, LENGTH = 0x50000
        prog (rx) : ORIGIN = 0x20450000, LENGTH = 512M-0x430000
        ram (rwx) : ORIGIN = 0x80000000, LENGTH = 32K
}

MPU_MIN_ALIGN = 1K;

INCLUDE /home/dieraca/uropnew/toolchain/tock/boards/kernel_layout.ld
