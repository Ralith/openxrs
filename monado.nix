{ stdenv, cmake, fetchFromGitLab, glslang, eigen, pkgconfig, openhmd, vulkan-loader, vulkan-headers, mesa, xlibs, hidapi, libglvnd }:
stdenv.mkDerivation {
  name = "monado";

  src = fetchFromGitLab {
    owner = "monado";
    repo = "monado";
    rev = "f016492a81574c9d3e17cd6dcab1a6119d30182c";
    domain = "gitlab.freedesktop.org";
    sha256 = "1p58mdwgqahyhm4mj0qvc7rzkdxz3y82cm03y91r1p9rnp5khvgz";
  };

  nativeBuildInputs = [ cmake glslang pkgconfig ];
  buildInputs = [ eigen openhmd vulkan-loader vulkan-headers mesa hidapi libglvnd ] ++ (with xlibs; [libX11 libXrandr libpthreadstubs libXau libXdmcp]);
}
