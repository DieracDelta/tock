fn main() {
    println!("cargo:rerun-if-changed=layout.ld");
    println!("cargo:rerun-if-changed=../kernel_layout.ld");
    println!("cargo:rustc-link-ar=-T/home/dieraca/uropnew/toolchain/tock/boards/hifive1/layout.ld");
    println!("cargo:rustc-link-ar=linker-flavor=ld.lld");
    println!("cargo:rustc-link-ar=relocation-model=dynamic-no-pic");
    println!("cargo:rustc-link-ar=link-arg=-zmax-page-size=512");
}
