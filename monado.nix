{ stdenv, cmake, fetchFromGitLab, glslang, eigen, pkgconfig, openhmd, vulkan-loader, vulkan-headers, mesa, xlibs, hidapi, libglvnd }:
stdenv.mkDerivation {
  name = "monado";

  src = fetchFromGitLab {
    owner = "monado";
    repo = "monado";
    rev = "0b7a620c020beed4c62b0354bae0b570e35f3028";
    domain = "gitlab.freedesktop.org";
    sha256 = "1q8n04g2i9af3h3v3mxzflhq4cd2xkz79b25sd028przvdmd0sz9";
  };

  nativeBuildInputs = [ cmake glslang pkgconfig ];
  buildInputs = [ eigen openhmd vulkan-loader vulkan-headers mesa hidapi libglvnd ] ++ (with xlibs; [libX11 libXrandr libpthreadstubs libXau libXdmcp]);
}
