use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Copia memory.x al directorio de salida donde el linker lo busca
    fs::copy("memory.x", out.join("memory.x")).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    
    // Fuerza recompilación si memory.x cambia
    println!("cargo:rerun-if-changed=memory.x");
}