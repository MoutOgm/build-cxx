use std::fs;
use std::path::PathBuf;

fn get_root_dir() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // Vérifie si un Cargo.toml de workspace existe dans le dossier parent
    let possible_workspace = manifest_dir.parent().unwrap().join("Cargo.toml");
    if possible_workspace.exists() && fs::read_to_string(&possible_workspace)
        .map(|content| content.contains("[workspace]"))
        .unwrap_or(false)
    {
        manifest_dir.parent().unwrap().to_path_buf()
    } else {
        manifest_dir
    }
}

fn main() {
    let crate_name = env!("CARGO_PKG_NAME");

    // Compilation CXX
    cxx_build::bridge("src/lib.rs")
        .std("c++17")
        .compile(&crate_name);

    let root_dir = get_root_dir();
    println!("cargo:warning=Root dir: {}", root_dir.display());

    // Chemins source et destination
    let src = root_dir.join(format!("target/cxxbridge/{}/src/lib.rs.h", crate_name));
    let dst_dir = root_dir.join("target/cxxbridge");
    let dst = dst_dir.join(format!("{}.hpp", crate_name));

    // Copier le fichier
    if src.exists() {
        fs::copy(&src, &dst).expect("Échec de la copie");
        println!("cargo:warning=Fichier copié : {} -> {}", src.display(), dst.display());
    } else {
        println!("cargo:warning=Fichier source non trouvé: {}", src.display());
    }

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rustc-env=TARGET=release");
}
