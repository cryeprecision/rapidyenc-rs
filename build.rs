use std::{env, path::PathBuf};

fn main() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let mut build_config = cmake::Config::new("vendor/rapidyenc");

    build_config.define("BUILD_NATIVE", "OFF");
    build_config.define("DISABLE_AVX256", "OFF");
    build_config.define("DISABLE_CRCUTIL", "ON");
    build_config.define("DISABLE_ENCODE", "OFF");
    build_config.define("DISABLE_DECODE", "OFF");
    build_config.define("DISABLE_CRC", "ON");

    build_config.build_target("install");
    let dst = build_config.build().join("lib");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=rapidyenc");
    println!("cargo:rerun-if-changed=vendor/rapidyenc");
    println!("cargo:rerun-if-changed=wrapper.h");
}
