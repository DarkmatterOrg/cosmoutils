let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  
  packages = with pkgs; [
    pkg-config
    rustc
    cargo
    rust-analyzer
    just
    rustfmt
    linuxHeaders
  ];

  LD_LIBRARY_PATH =
    with pkgs;
    pkgs.lib.makeLibraryPath [
      linuxHeaders
    ];
}
