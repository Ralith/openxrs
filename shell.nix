with import (fetchTarball https://github.com/NixOS/nixpkgs/archive/master.tar.gz) { };
let dlopen-libs = with xlibs; [ vulkan-loader vulkan-validation-layers libX11 libXcursor libXrandr libXi ]; in
stdenv.mkDerivation {
  name = "openxrs";
  nativeBuildInputs = with pkgs; [ rustChannels.stable.rust ];
  buildInputs = [ (callPackage ./monado.nix {}) vulkan-headers ];
  shellHook = ''
    export RUST_BACKTRACE=1
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${stdenv.lib.makeLibraryPath dlopen-libs}"
    export VK_INSTANCE_LAYERS=VK_LAYER_LUNARG_standard_validation
    export XDG_DATA_DIRS="$XDG_DATA_DIRS:${vulkan-validation-layers}/share"
  '';
}
