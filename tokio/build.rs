use std::{env, path::Path, process::Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR").unwrap();

    let compile = Command::new("cc")
        .args(&["usdt.c", "-c", "-fPIC", "-o"])
        .arg(&format!("{out_dir}/usdt.o"))
        .current_dir(&Path::new("./usdt"))
        .status()?;

    let probes = Command::new("dtrace")
        .args(&["-G", "-s", "probes.d"])
        .arg(&format!("{out_dir}/usdt.o"))
        .arg("-o")
        .arg(&format!("{out_dir}/systemtap.o"))
        .current_dir(&Path::new("./usdt"))
        .status()?;

    let archive = Command::new("ar")
        .args(&["crus", "libusdt.a", "usdt.o", "systemtap.o"])
        .current_dir(&Path::new(&out_dir))
        .status()?;

    if !compile.success() || !probes.success() || !archive.success() {
        return Err("failed to build usdt probes".into());
    }
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=usdt");
    println!("cargo:rerun-if-changed=usdt/usdt.c");
    println!("cargo:rerun-if-changed=usdt/usdt.h");
    println!("cargo:rerun-if-changed=usdt/probes.d");

    Ok(())
}
