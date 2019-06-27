use std::process::Command;
fn main() {
    let mut version_wrapped = Command::new("git");
    version_wrapped.args(&["describe", "--always"]);
    println!("cargo:rustc-env=TOCK_KERNEL_VERSION={}", String::from_utf8_lossy(&version_wrapped.output().expect("1.3+").stdout));
}
