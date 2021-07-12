# Examples

## `hello`

Show information about the OpenXR runtime and XR headset.

## `vulkan`

Display a head-locked gradient spanning both eyes. Controllers position is reported on the terminal.

## `vulkan-android`

Same as `vulkan` but it can run on Android, specifically on the Oculus Quest and Quest 2. It shares the same source file.  

* Install [cargo-apk](https://crates.io/crates/cargo-apk)
* Get `libopenxr_loader.so` from the Oculus OpenXR Mobile SDK and add it to `openxr/examples/libs/arm64-v8a`
* Run:

    ```sh
    cd openxr
    cargo apk run --example vulkan-android
    ```

[Cargo.toml](../Cargo.toml) contains some metadata used by cargo-apk to generate the `AndroidManifest.xml`. In this example it is configured for the Oculus Quest and Quest 2, but changes are necessary to enable features like hand tracking or adding support for more headsets like the Oculus Go. You can read more details in the developer portal of your headset.
