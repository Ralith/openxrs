fn main() {
    #[cfg(feature = "static")]
    {
        let dst = cmake::Config::new("OpenXR-SDK")
            .define("BUILD_API_LAYERS", "OFF")
            .define("BUILD_TESTS", "OFF")
            .define("OpenGL_GL_PREFERENCE", "GLVND")
            .define("DYNAMIC_LOADER", "OFF")
            .define("CMAKE_INSTALL_LIBDIR", "lib")
            .define("BUILD_WITH_SYSTEM_JSONCPP", "OFF") // See https://github.com/KhronosGroup/OpenXR-SDK-Source/issues/481
            .profile("Release")
            .build();

        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("lib").display()
        );
        println!("cargo:rustc-link-lib=static=openxr_loader");

        let target_os = std::env::var_os("CARGO_CFG_TARGET_OS")
            .expect("missing CARGO_CFG_TARGET_OS environment variable");

        if target_os == "macos" || target_os == "freebsd" {
            println!("cargo:rustc-link-lib=c++");
        } else if target_os == "windows" {
            // The OpenXR loader requires linking to `advapi32` on Windows:
            // https://github.com/KhronosGroup/OpenXR-SDK-Source/blob/58e026e7efa5a4f6a612c554560b53aa482f04d4/src/loader/CMakeLists.txt#L245
            println!("cargo:rustc-link-lib=advapi32");
        } else if target_os == "android" {
            // The OpenXR loader requires linking to the `log` and `android` libraries on Android:
            // https://github.com/KhronosGroup/OpenXR-SDK-Source/blob/c610211f38f4e1e4ac811ced6135e144eedc7cf2/src/loader/CMakeLists.txt#L161-L163
            // Note that the Rust `std` also currently links `log` on Android,
            // but that might change in the future, so we link it as well.
            println!("cargo:rustc-link-lib=log");
            println!("cargo:rustc-link-lib=android");

            // The OpenXR loader needs a C++ runtime library, but Android systems don't have it.
            //
            // A statically linked C++ runtime library is used by default.
            // It's reasonable to assume that the user wants a fully static build when using the `static` feature.
            //
            // The pre-built loader distributed by Khronos is also built using the static C++ runtime library:
            // https://github.com/KhronosGroup/OpenXR-SDK-Source/blob/c610211f38f4e1e4ac811ced6135e144eedc7cf2/maintainer-scripts/build-aar.sh#L50
            //
            // `c++_static` can cause issues when multiple C++ runtime libraries are included in one application:
            // https://developer.android.com/ndk/guides/cpp-support
            // In that case the user can set `OPENXRS_ANDROID_STL` to choose a different method of linking.
            println!("cargo:rerun-if-env-changed=OPENXRS_ANDROID_STL");
            let cxx_runtime_library = std::env::var("OPENXRS_ANDROID_STL");
            match cxx_runtime_library.as_ref().map(String::as_str) {
                Ok("static") | Err(std::env::VarError::NotPresent) => {
                    println!("cargo:rustc-link-lib=c++_static");
                    // I can't find good documentation on this,
                    // but some symbols are defined in `c++abi` instead of `c++_static`.
                    // https://libcxxabi.llvm.org/#:~:text=Why%20are%20the%20destructors%20for%20the%20standard%20exception%20classes%20defined%20in%20libc%2B%2Babi%3F
                    println!("cargo:rustc-link-lib=c++abi");
                }
                Ok("shared") => {
                    // requires `libc++_shared.so` to be included in the APK
                    println!("cargo:rustc-link-lib=c++_shared");
                }
                Ok("none") => {
                    // user will provide required C++ runtime library
                }
                var @ Err(std::env::VarError::NotUnicode(_)) | var @ Ok(_) => {
                    panic!("invalid value for OPENXRS_ANDROID_STL env var: {var:?}");
                }
            }
        } else {
            println!("cargo:rustc-link-lib=stdc++");
        }
    }
    #[cfg(all(not(feature = "static"), feature = "linked"))]
    {
        println!("cargo:rustc-link-lib=openxr_loader");
    }
}
