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
    }
    if !cfg!(debug_assertions) {
        cc.opt_level(2);
    }
    cc.compile("src");
}
