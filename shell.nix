{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
      pkgs.alsa-lib
      pkgs.udev
      pkgs.vulkan-loader
      pkgs.libxkbcommon
    ]}"
  '';

  buildInputs = with pkgs; [
    lld
    clang

    # bevy-specific deps (from https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)
    pkg-config
    udev
    alsa-lib
    lutris
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    libxkbcommon
    # vulkan-tools
    # vulkan-headers
    vulkan-loader
    # vulkan-validation-layers
  ];
}
