# openxr-sys

## Cloning

If you can, clone the repo using `git clone --recurse-submodules`. 

If you already cloned the repo without, you must run (from anywhere in the repo): `git submodule update --init`.

## Updating the OpenXR SDK

1. Update the OpenSR-SDK git submodule to a new commit hash or tag:
  a. `cd openxrs/sys/OpenXR-SDK`.
  b. `git up <tag/xxx or hash>`.
  c. `cd ../..` to get back to the `openxrs` repo root and check that the 
  submodule is updated (should be a single line change to what looks like a file
  at `openxrs/sys/OpenXR-SDK`).
2. `cd generator; cargo run --bin generator` to regenerate the `sys` crate.
3. `cd ..` to go back to the `openxrs` repo root`.
3. `cargo fmt && cargo build && cargo test` to find any issues that need fixing.
