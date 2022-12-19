extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    bindgen::Builder::default()
        .header("src/x11/aes_helper.h")
        .header("src/x11/Blake.h")
        .header("src/x11/Bmw.h")
        .header("src/x11/CubeHash.h")
        .header("src/x11/Echo.h")
        .header("src/x11/Groestl.h")
        .header("src/x11/Jh.h")
        .header("src/x11/Keccak.h")
        .header("src/x11/Luffa.h")
        .header("src/x11/Shavite.h")
        .header("src/x11/Simd.h")
        .header("src/x11/Skein.h")
        .header("src/x11/sph_types.h")
        .generate()
        .expect("Error bindings generation")
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("src.rs"))
        .expect("Couldn't write bindings!");
    cc::Build::new()
        .file("src/x11_hash.c")
        .include("src")
        .flag("-Wno-unused-but-set-variable")
        .compile("src");
}
