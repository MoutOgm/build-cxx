use std::fs;

fn main() {
    let _build = cxx_build::bridge("src/lib.rs")
        .std("c++17")
        .compile("libutils.a")
        ;
    fs::copy("../target/cxxbridge/utils/src/lib.rs.h", "../target/cxxbridge/utils.h").unwrap();
    // fs::rename("../target/cxxbridge/utils/src/lib.rs.h", "../target/cxxbridge/utils.h").unwrap();
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rustc-env=TARGET=release");
    println!("cargo::warning=Export C++ headers to cxxbridge/utils/src");
}