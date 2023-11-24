use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the Kaitai Struct files change
    println!("cargo:rerun-if-changed=alias8.ksy");
    println!("cargo:rerun-if-changed=split_path.ksy");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let generated = kaitai_gen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .add_ksy_file("alias8.ksy")
        .add_ksy_file("split_path.ksy")
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    generated
        .write_to_file(out_path.join("ksy.rs"))
        .expect("Couldn't write bindings!");
}
