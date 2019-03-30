{ stdenv, cmake, fetchFromGitLab, glslang, eigen, pkgconfig, openhmd, vulkan-loader, vulkan-headers, mesa, xlibs, hidapi, libglvnd }:
stdenv.mkDerivation {
  name = "monado";

  src = fetchFromGitLab {
    owner = "monado";
    repo = "monado";
    rev = "5d5d420fb68ffc702d683f0e9f9c8588a9d3480b";
    domain = "gitlab.freedesktop.org";
    sha256 = "1y2sf4fzyr01wgb7kh1m6fp92i6phjf27dsgiwia2j5lz1hsdv1l";
  };

  nativeBuildInputs = [ cmake glslang pkgconfig ];
  buildInputs = [ eigen openhmd vulkan-loader vulkan-headers mesa hidapi libglvnd ] ++ (with xlibs; [libX11 libXrandr libpthreadstubs libXau libXdmcp]);
}
