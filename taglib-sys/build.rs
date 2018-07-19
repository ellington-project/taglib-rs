extern crate bindgen;
extern crate cmake;

use cmake::Config;

use std::env;
use std::path::PathBuf;

fn main() { 
    // tell cargo to build our taglib branch
    let dst = Config::new("taglib")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("ENABLE_STATIC_RUNTIME", "ON")
        .define("CMAKE_C_FLAGS","-fPIC -Wall -O3")
        .build();

    // tell cargo to look for it when trying to link
    println!("cargo:rustc-link-search={}/lib", dst.display());
    // link libc++, as the static linker doesn't, and we need it for the tag internals
    // and tell cargo to link the static library that it finds there! 
    // note, we want to do this to avoid linking in the system tag_c, which might not have bpm support
    println!("cargo:rustc-flags=-l dylib=stdc++");
    println!("cargo:rustc-flags=-l tag_c -l tag");

    // create bindings for the static c library
    // let heder = format!("{}/include/taglib/tag_c.h", dst.display());
    let header = dst.join("include").join("taglib").join("tag_c.h");
    let bindings = bindgen::Builder::default()
        // use the header from the dst, where cmake has writen the headers
        .header(header.to_str().unwrap())
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
