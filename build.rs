extern crate cc;

use std::env;

fn main() {
    let mut cc = cc::Build::new();
    cc.file("src/x11_hash.c");
    cc.include("src");
    // Only set clang for wasm, otherwise let cc auto-detect
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "wasm32" {
        cc.compiler("clang");
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
