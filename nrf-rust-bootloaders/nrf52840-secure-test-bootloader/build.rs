extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the static object file - nrf_cc310_bl_0.9.12 
    // shared library.
    println!("cargo:rustc-link-search={}","/Users/Nil/devspace/rust/projects/nrf52-rust/nrf-rust-bootloaders/nrf52840-secure-test-bootloader");
    println!("cargo:rustc-link-lib=static=nrf_cc310_bl_0.9.12");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    // println!("cargo:rerun-if-changed=bindings/bindings_cc310_bl.h");

    // // eprintln!("--target={}", env::var("TARGET").unwrap());
    // // The bindgen::Builder is the main entry point
    // // to bindgen, and lets you build up options for
    // // the resulting bindings.
    let bb = bindgen::builder()
        // The input header we would like to generate
        // bindings for.
        .header("bindings/bindings_cc310_bl.h")
        .ctypes_prefix("cty")
        .generate_comments(true)
        .rustfmt_bindings(true)
        // .clang_arg("-Inrf_cc310_bl_0.9.12")
        .clang_arg(format!("--target={}", env::var("TARGET").unwrap()))
        .use_core();

    eprintln!(
            "please run bindgen --output src/bindings.rs {}",
            bb.command_line_flags().join(" ")
        );     
        // Finish the builder and generate the bindings. Unwrap the Result and panic on failure.
    let bindings = bb.generate().expect("Unable to generate bindings");   
        

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bl_cc310.rs"))
        .expect("Couldn't write bindings!");
}