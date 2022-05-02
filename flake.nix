{
  description = "The similar thing to the hlissner/dotfiles/bin/huh wrapper script";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        huh =
          let
            craneLib = crane.lib."${system}";
          in
          craneLib.buildPackage {
            src = self;
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
