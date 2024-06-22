{
  description = "A full-stack rust website using Leptos";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [
        inputs.treefmt-nix.flakeModule
        ./nix/leptos-module.nix
      ];
      perSystem =
        { config
        , self'
        , pkgs
        , lib
        , system
        , ...
        }: {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              inputs.rust-overlay.overlays.default
            ];
          };

          # Auto-formatters
          treefmt.config = {
            projectRootFile = "flake.nix";
            programs = {
              nixpkgs-fmt.enable = true;
              rustfmt.enable = true;
              leptosfmt.enable = true;
            };
          };

          packages.default = self'.packages.webbed_site;

          devShells.default = pkgs.mkShell {
            packages = with pkgs; [ sqlx-cli ];
            inputsFrom = [
              config.treefmt.build.devShell
              self'.devShells.webbed_site
            ];
            nativeBuildInputs = with pkgs; [
              just
              cargo-watch
            ];
          };
        };
    };
}
