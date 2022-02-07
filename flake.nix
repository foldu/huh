{
  description = "The similar thing to the hlissner/dotfiles/bin/huh wrapper script";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, naersk, flake-utils }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        huh =
          let
            naersk-lib = naersk.lib."${system}".override {
              cargo = pkgs.cargo;
              rustc = pkgs.rustc;
            };
          in
          naersk-lib.buildPackage {
            src = ./.;
            singleStep = true;
            nativeBuildInputs = [
              pkgs.installShellFiles
            ];
            postInstall = ''
              installShellCompletion target/release/build/huh-*/out/huh.{fish,bash}
              installShellCompletion --zsh target/release/build/huh-*/out/_huh
            '';
          };
      in
      {
        defaultPackage = huh;
        packages = {
          inherit huh;
        };
      }
    );
}
