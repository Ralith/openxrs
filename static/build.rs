fn main() {
    let dst = cmake::Config::new("OpenXR-SDK")
        .no_build_target(true)
        .define("BUILD_API_LAYERS", "OFF")
        .define("BUILD_TESTS", "OFF")
        .define("OpenGL_GL_PREFERENCE", "GLVND")
        .build();

    println!("cargo:rustc-link-search=native={}/build/src/loader", dst.display());
    println!("cargo:rustc-link-lib=static=openxr_loader");
}
