# OpenXRS

[![Documentation](https://docs.rs/openxr/badge.svg)](https://docs.rs/openxr/)
[![Crates.io](https://img.shields.io/crates/v/openxr.svg)](https://crates.io/crates/openxr)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE-APACHE)

Rust bindings for the OpenXR virtual/augmented reality runtime
API. Refer to [the
specification](https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html)
for detailed documentation on individual API calls.

## `openxr` 

The high-level bindings provide abstractions focusing on ergonomics
and safety. Escape hatches to the raw API are exposed to support
unforeseen requirements, and patterns that cannot be efficiently
exposed in safe terms are preserved as unsafe.

The crate exposes a number of cargo features:
- `static` builds in the Khronos OpenXR loader, which can then be
  accessed with `Entry::linked()`. This is the easiest way to get
  going, provided your environment has a working C++ compiler and
  CMake installation.
- `loaded` allows access to a manually identified OpenXR
  implementation at run time. This allows for cases where a built-in
  Khronos loader, normally responsible for that task, cannot be used.
- `linked` attempts to link to an OpenXR loader in the build
  environment. This is appropriate for target environments like
  desktop Linux which guarantee the presence of an OpenXR
  implementation or loader at a specific location, making a built-in
  loader redundant.
- `mint` exposes `From` impls for converting to and from
  [mint](https://github.com/kvark/mint) types where appropriate.

See `openxr/examples/vulkan.rs` for an example high-performance Vulkan
rendering workflow.

## `openxr-sys`

The low-level bindings provide faithful unsafe access to the raw API,
with ergonomics and type safety improved as much as feasible compared
to a `bindgen`-style binding without reducing expressiveness. For
example, symbols are named according to Rust conventions, enums and
bitmasks are strongly typed, and many types have helpful `Debug`
impls. This crate is almost entirely generated from the Khronos XML
registry.
