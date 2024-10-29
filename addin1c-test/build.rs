fn main() {
    let mut build = cc::Build::new();

    if let Ok(target) = std::env::var("TARGET") {
        if target.contains("windows") {
            build.define("_WINDOWS", "");
        }
    }

    build
        .cpp(true)
        .file("src/memory_manager.cpp")
        .file("src/variant.cpp")
        .file("src/component_base.cpp")
        .include("include")
        .compile("helper1c");

    println!("cargo:rerun-if-changed=src/component_base.cpp");
    println!("cargo:rerun-if-changed=src/memory_manager.cpp");
    println!("cargo:rerun-if-changed=src/variant.cpp");
    println!("cargo:rerun-if-changed=include");
}
