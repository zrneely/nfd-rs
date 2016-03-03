use std::env;

fn main() {
    let ndf_lib_dir = env::var("NFD_LIB_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}",ndf_lib_dir);
    println!("cargo:rustc-link-lib=static=nfd");
    println!("cargo:rustc-link-lib=framework=AppKit");
}
