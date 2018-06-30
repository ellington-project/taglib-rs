extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;

fn main() { 
    //try to build taglib 
    let dst = cmake::build("taglib");

    println!("Searching via: {:?}", dst.display());

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search={}/lib/", dst.display());
    println!("cargo:rustc-link-lib=static=libtag_c.a");
}