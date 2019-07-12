// NOTE: Adapted from cortex-m/build.rs
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let name = env::var("CARGO_PKG_NAME").unwrap();

    let feature_compressed_isa = env::var("CARGO_FEATURE_COMPRESSED_ISA").is_ok();
    let feature_interrupts = env::var("CARGO_FEATURE_INTERRUPTS").is_ok();
    let feature_interrupts_qregs = env::var("CARGO_FEATURE_INTERRUPTS_QREGS").is_ok();

    if target.starts_with("riscv") {
        let arch_features = if feature_compressed_isa {
            "ic"
        } else {
            "i"
        };
        let cpu_features = if feature_interrupts_qregs {
            "RV32RT_INTERRUPTS_QREGS"
        } else if feature_interrupts {
            "RV32RT_INTERRUPTS"
        } else {
            "RV32RT_BARE"
        };

        let lib_name = format!("riscv32{}-unknown-none-elf_{}", arch_features, cpu_features);

        fs::copy(
            format!("bin/{}.a", lib_name),
            out_dir.join(format!("lib{}.a", name)),
        ).unwrap();

        println!("cargo:rustc-link-lib=static={}", name);
        println!("cargo:rustc-link-search={}", out_dir.display());
    }

    // Put the linker script somewhere the linker can find it
    fs::File::create(out_dir.join("link.x"))
        .unwrap()
        .write_all(include_bytes!("link.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=link.x");
}
