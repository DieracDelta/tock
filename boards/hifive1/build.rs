use std::env;

fn main() {
    
    let mut dir_path = env::current_dir().unwrap();


    println!("cargo:rerun-if-changed=layout.ld");
    println!("cargo:rerun-if-changed=../kernel_layout.ld");
    println!("cargo:rustc-link-ar=-T./layout.ld");//, dir_path.to_str());
    println!("cargo:rustc-link-ar=linker-flavor=ld.lld");
    println!("cargo:rustc-link-ar=relocation-model=dynamic-no-pic");
    println!("cargo:rustc-link-ar=link-arg=-zmax-page-size=512");
}
