extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    // bindgen::Builder::default()
    //     .header("src/x11/aes_helper.h")
    //     .header("src/x11/Blake.h")
    //     .header("src/x11/Bmw.h")
    //     .header("src/x11/CubeHash.h")
    //     .header("src/x11/Echo.h")
    //     .header("src/x11/Groestl.h")
    //     .header("src/x11/Jh.h")
    //     .header("src/x11/Keccak.h")
    //     .header("src/x11/Luffa.h")
    //     .header("src/x11/Shavite.h")
    //     .header("src/x11/Simd.h")
    //     .header("src/x11/Skein.h")
    //     .header("src/x11/sph_types.h")
    //     .generate()
    //     .expect("Error bindings generation")
    //     .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("src.rs"))
    //     .expect("Couldn't write bindings!");

    let mut cc = cc::Build::new();
    cc.file("src/x11_hash.c");
    cc.compiler("clang");
    cc.include("src");
    cc.flag("-Wno-unused-but-set-variable");
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "wasm32" {
        // Include fake wasm-sysroot headers to pass compilation
        cc.include("wasm-sysroot");
        cc.archiver("llvm-ar");
    } else if env::var("CARGO_CFG_TARGET_OS").unwrap() =="android" {
        // this assumes that cargo-ndk is used initiate the build
        let sysroot_path = env::var("CARGO_NDK_SYSROOT_PATH").expect("CARGO_NDK_SYSROOT_PATH is not set");
        // Construct the `CFLAGS` with `--sysroot` pointing to the NDK's sysroot path.
        let cflags = format!("--sysroot={}", sysroot_path);
        cc.flag(&cflags);
    }
    if !cfg!(debug_assertions) {
        cc.opt_level(2);
    }
    cc.compile("src");
}
