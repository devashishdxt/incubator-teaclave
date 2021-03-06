use std::env;
use std::path::Path;
use std::process::Command;
use std::str;

fn main() {
    let is_sim = env::var("SGX_MODE").unwrap_or_else(|_| "HW".to_string());
    match is_sim.as_ref() {
        "HW" => {}
        _ => println!("cargo:rustc-cfg=sgx_sim"),
    }

    let out_dir = env::var("OUT_DIR").expect("$OUT_DIR not set. Please build with cargo");
    let dest_file = Path::new(&out_dir).join("build_config.rs");
    println!("cargo:rerun-if-changed=config_gen/main.rs");
    println!("cargo:rerun-if-changed=config_gen/templates/config.j2");
    println!("cargo:rerun-if-changed=build.config.toml");
    println!("cargo:rerun-if-changed=build.rs");
    let target_dir = Path::new(&env::var("TEACLAVE_SYMLINKS").expect("TEACLAVE_SYMLINKS"))
        .join("teaclave_build/target/config_gen");
    let c = Command::new("cargo")
        .args(&[
            "run",
            "--target-dir",
            &target_dir.to_string_lossy(),
            "--manifest-path",
            "config_gen/Cargo.toml",
            "--",
            "-t",
            "build.config.toml",
            "-o",
            &dest_file.to_string_lossy(),
        ])
        .output()
        .expect("Cannot generate build_config.rs");
    if !c.status.success() {
        panic!(
            "stdout: {:?}, stderr: {:?}",
            str::from_utf8(&c.stderr).unwrap(),
            str::from_utf8(&c.stderr).unwrap()
        );
    }
}
