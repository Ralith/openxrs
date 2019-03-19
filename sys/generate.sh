#!/usr/bin/env sh
bindgen --whitelist-type '(Xr|PFN_xr).*' --whitelist-var 'XR.*' bindgen.h -- -I OpenXR-Docs/include/ > src/generated.rs
