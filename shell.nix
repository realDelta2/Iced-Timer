with import <nixpkgs> {};

stdenv.mkDerivation rec {
  name = "iced-env";
  buildInputs = [
    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr libGL freetype pkgconfig freetype.dev expat
    fontconfig alsa-lib alsa-utils alsa-oss
  ];

  LD_LIBRARY_PATH = builtins.foldl'
    (a: b: "${a}:${b}/lib") "${vulkan-loader}/lib" buildInputs;
}