use std::fs;
use std::path::PathBuf;

fn main() {
    let crate_name = env!("CARGO_PKG_NAME");
    let _build = cxx_build::bridge("src/lib.rs")
        .std("c++17");

    // Get the root directory of the crate
    let crate_root = env!("CARGO_MANIFEST_DIR");

    // Define your source and destination paths relative to the crate root
    let src = PathBuf::from(crate_root).join(format!("target/cxxbridge/{}/src/lib.rs.h", crate_name));
    let dst = PathBuf::from(crate_root).join(format!("target/cxxbridge/{}.hpp", crate_name));

    // Copy the file
    fs::copy(&src, &dst).expect("Failed to copy file");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rustc-env=TARGET=release");
    println!("cargo:warning=Export C++ headers to cxxbridge/{}/src", crate_name);
}