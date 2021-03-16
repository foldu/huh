{
  description = "The similar thing to the hlissner/dotfiles/bin/hey wrapper script";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, naersk, flake-utils }@inputs: {
    overlay = final: prev: {
      hey =
        let
          pkgs = nixpkgs.legacyPackages.${prev.system};
          naersk-lib = naersk.lib."${prev.system}".override {
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
            installShellCompletion target/release/build/hey-*/out/hey.{fish,bash}
            installShellCompletion --zsh target/release/build/hey-*/out/_hey
          '';
        };

    };
  } // flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        overlays = [ self.overlay ];
        inherit system;
      };
    in
    {
      defaultPackage = pkgs.hey;
    }
  );
}
