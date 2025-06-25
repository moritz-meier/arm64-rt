use std::{cell::LazyCell, env, fs, path::PathBuf};

const MEMORY_LD: &[u8] = include_bytes!("memory.ld");
const OUT_DIR: LazyCell<PathBuf> = LazyCell::new(|| env::var("OUT_DIR").unwrap().into());

pub fn main() {
    fs::write(OUT_DIR.join("memory.ld"), MEMORY_LD).unwrap();

    println!("cargo:rustc-link-search={}", OUT_DIR.display());
    println!("cargo:rustc-link-arg=-Tlink.ld");
}
