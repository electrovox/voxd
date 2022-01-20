extern crate bindgen;
use std::env;
use std::path::PathBuf;
fn main() {
    // Link aginst the espeak_ng library.
    println!("cargo:rust-link-libs=libespeak-ng");
    // Rerun the build process if the header has changed.
    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Failed to generate the bindings.");

    // Write them out
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings!");
}
