{ stdenv, cmake, fetchFromGitLab, glslang, eigen, pkgconfig, openhmd, vulkan-loader, vulkan-headers, mesa, xlibs, hidapi, libglvnd }:
stdenv.mkDerivation {
  name = "monado";

  src = fetchFromGitLab {
    owner = "monado";
    repo = "monado";
    rev = "bc770524937032e8d6fb1f8eb4daa37289ae1f69";
    domain = "gitlab.freedesktop.org";
    sha256 = "0s8v86gg0j9775ivng5syb2i39wra9hidig5awqiy14l1dlhp5cg";
  };

  nativeBuildInputs = [ cmake glslang pkgconfig ];
  buildInputs = [ eigen openhmd vulkan-loader vulkan-headers mesa hidapi libglvnd ] ++ (with xlibs; [libX11 libXrandr libpthreadstubs libXau libXdmcp]);
}
