use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("qemu-virt-arm64.ld"))
        .unwrap()
        .write_all(include_bytes!("qemu-virt-arm64.ld"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed=qemu-virt-arm64.ld");
}
