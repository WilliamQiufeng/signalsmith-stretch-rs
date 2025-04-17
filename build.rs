use std::env;
use std::path::Path;

fn main() {
    let src_path = Path::new("src");

    println!("cargo::rerun-if-changed=src/wrapper.h");
    println!("cargo::rerun-if-changed=src/wrapper.cpp");
    println!("cargo::rerun-if-changed=signalsmith-stretch/signalsmith-stretch.h");

    // Run cmake command on the folder signalsmith-stretch
    let path = cmake::Config::new("signalsmith-stretch")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("CMAKE_PROJECT_NAME", "SignalsmithStretch")
        .define("CMAKE_MINIMUM_REQUIRED_VERSION", "3.15")
        .build_target("all")
        .build();

    cc::Build::new()
        .file(src_path.join("wrapper.cpp"))
        .include(Path::new("signalsmith-stretch"))
        .include(path.join("build/_deps/signalsmith-linear-src/include"))
        .cpp(true)
        .compile("signalsmith-stretch");

    let bindings = bindgen::Builder::default()
        .header(src_path.join("wrapper.h").as_os_str().to_str().unwrap())
        .allowlist_item("signalsmith_stretch_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
