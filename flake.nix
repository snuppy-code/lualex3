# I should read this ! I didn't !
# https://fasterthanli.me/series/building-a-rust-service-with-nix/part-10
{
  description = "lualexer3 development flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-26.05";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    my = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages."${my}";
  in {
    devShells."${my}".default = pkgs.mkShell {
      buildInputs = with pkgs; [
        cargo
        rustc
        rustfmt
        clippy
        # rust-analyzer # doesn't seem necessary?
      ];
      env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    };
  };
}
