fn main() {
    #[cfg(feature = "static")]
    {
        let dst = cmake::Config::new("OpenXR-SDK")
            .define("BUILD_API_LAYERS", "OFF")
            .define("BUILD_TESTS", "OFF")
            .define("OpenGL_GL_PREFERENCE", "GLVND")
            .define("DYNAMIC_LOADER", "OFF")
            .define("CMAKE_INSTALL_LIBDIR", "lib")
            .profile("Release")
            .build();

        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("lib").display()
        );
        println!("cargo:rustc-link-lib=static=openxr_loader");

        if cfg!(any(target_os = "macos", target_os = "freebsd")) {
            println!("cargo:rustc-link-lib=c++");
        } else if cfg!(target_os = "windows") {
            println!("cargo:rustc-link-lib=pathcch");
        } else {
            println!("cargo:rustc-link-lib=stdc++");
            println!("cargo:rustc-link-lib=stdc++fs");
        }
    }
    #[cfg(all(not(feature = "static"), feature = "linked"))]
    {
        println!("cargo:rustc-link-lib=openxr_loader");
    }
}
