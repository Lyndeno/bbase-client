{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";

    pre-commit-hooks-nix = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
    pre-commit-hooks-nix,
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec {
      packages.bbase = naersk-lib.buildPackage {
        nativeBuildInputs = [pkgs.installShellFiles pkgs.pkg-config pkgs.libadwaita pkgs.gtk4 pkgs.wrapGAppsHook4];
        postInstall = ''
          installShellCompletion --cmd bbase \
            --bash ./target/release/build/bbase-*/out/bbase.bash \
            --fish ./target/release/build/bbase-*/out/bbase.fish \
            --zsh ./target/release/build/bbase-*/out/_bbase
          installManPage ./target/release/build/bbase-*/out/bbase.1
        '';
        pname = "bbase";
        root = ./.;
      };
      packages.default = packages.bbase;

      apps.bbase = utils.lib.mkApp {
        drv = packages.bbase;
      };
      apps.default = apps.bbase;

      formatter = pkgs.alejandra;

      devShells.default = let
        pre-commit-format = pre-commit-hooks-nix.lib.${system}.run {
          src = ./.;

          hooks = {
            alejandra.enable = true;
            rustfmt.enable = true;
          };
        };
      in
        pkgs.mkShell {
          nativeBuildInputs = with pkgs; [rustc cargo rustfmt clippy graphql-client pkg-config libadwaita gtk4];
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          shellHook = ''
            ${pre-commit-format.shellHook}
          '';
        };
    });
}
