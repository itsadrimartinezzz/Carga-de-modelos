use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=vendor/SDL2");
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        if env::var("CARGO_CFG_TARGET_ENV").as_deref() != Ok("msvc") {
            panic!("En Windows usa Rust MSVC x86_64; consulta el README.");
        }
        let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let lib = root.join("vendor/SDL2/lib/x64");
        assert!(
            lib.join("SDL2.lib").exists(),
            "Falta vendor/SDL2/lib/x64/SDL2.lib"
        );
        println!("cargo:rustc-link-search=native={}", lib.display());
        // Cargo deja OUT_DIR en target/<perfil>/build/<paquete>/out.
        let out = PathBuf::from(env::var("OUT_DIR").unwrap());
        let profile = out.ancestors().nth(3).unwrap();
        fs::copy(lib.join("SDL2.dll"), profile.join("SDL2.dll"))
            .expect("No se pudo copiar SDL2.dll junto al ejecutable");
        // Los ejecutables de cargo test se generan dentro de deps/.
        fs::create_dir_all(profile.join("deps")).unwrap();
        fs::copy(lib.join("SDL2.dll"), profile.join("deps/SDL2.dll")).unwrap();
    }
}
