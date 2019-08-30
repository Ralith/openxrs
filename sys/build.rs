#[cfg(feature = "static")]
fn main() {
    let dst = cmake::Config::new("OpenXR-SDK")
        .define("BUILD_API_LAYERS", "OFF")
        .define("BUILD_TESTS", "OFF")
        .define("OpenGL_GL_PREFERENCE", "GLVND")
        .define("DYNAMIC_LOADER", "OFF")
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    let mut library_name = "openxr_loader".to_string();
    if cfg!(target_os = "windows") {
        library_name = library_name + "-1_0";
    }
    println!("cargo:rustc-link-lib=static={}", library_name);

    if cfg!(any(target_os = "macos", target_os = "freebsd")) {
        println!("cargo:rustc-link-lib=c++");
    } else if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=ShLwApi");
    } else {
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=stdc++fs");
    }
}

#[cfg(not(feature = "static"))]
fn main() {}
