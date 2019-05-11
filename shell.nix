with import <nixpkgs> { };
let
openhmd_git = openhmd.overrideAttrs (_: {
  src = fetchFromGitHub {
    owner = "OpenHMD";
    repo = "OpenHMD";
    rev = "c6d4f70689f884f9ded06aa13cb04f5ef74998a0";
    sha256 = "0xpjkmi9rsvp64ridg7q57fv4q7nj79c5xs4w160zv813idk0c1q";
  };
});
monado = callPackage ./monado.nix { openhmd = openhmd_git; };
dlopen-libs = with xlibs; [ monado vulkan-loader vulkan-validation-layers libX11 libXcursor libXrandr libXi ];
in stdenv.mkDerivation {
  name = "openxrs";
  nativeBuildInputs = with pkgs; [ rustChannels.stable.rust cmake python3 pkgconfig ];
  buildInputs = [ monado vulkan-headers vulkan-loader libGL ] ++ (with xlibs; [ libX11 libXxf86vm libpthreadstubs libXrandr ]);
  shellHook = ''
    export RUST_BACKTRACE=1
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${stdenv.lib.makeLibraryPath dlopen-libs}"
    export VK_INSTANCE_LAYERS=VK_LAYER_LUNARG_standard_validation
    export XDG_DATA_DIRS="$XDG_DATA_DIRS:${vulkan-validation-layers}/share"
    export XR_RUNTIME_JSON="${monado}/share/openxr/0/openxr_monado.json"
  '';
}
