{ stdenv, cmake, fetchFromGitLab, glslang, eigen, pkgconfig, openhmd, vulkan-loader, vulkan-headers, mesa, xlibs, hidapi, libglvnd }:
stdenv.mkDerivation {
  name = "monado";

  src = fetchFromGitLab {
    owner = "monado";
    repo = "monado";
    rev = "52e11ac4c589df267e6574718a1347ad827c0452";
    domain = "gitlab.freedesktop.org";
    sha256 = "0y5c230lj8nr6nqd7gh87cmlfmjrnfxw1gl5a1ybmzgw6y6h101h";
  };

  nativeBuildInputs = [ cmake glslang pkgconfig ];
  buildInputs = [ eigen openhmd vulkan-loader vulkan-headers mesa hidapi libglvnd ] ++ (with xlibs; [libX11 libXrandr libpthreadstubs libXau libXdmcp]);
}
