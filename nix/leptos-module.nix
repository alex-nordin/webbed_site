{ self
, lib
, inputs
, flake-parts-lib
, ...
}:
let
  inherit
    (flake-parts-lib)
    mkPerSystemOption
    ;
in
{
  options = {
    perSystem =
      mkPerSystemOption
        ({ config
         , self'
         , inputs'
         , pkgs
         , system
         , ...
         }: {
          options = {
            webbed_site.overrideCraneArgs = lib.mkOption {
              type = lib.types.functionTo lib.types.attrs;
              default = _: { };
              description = "Override crane args for the webbed_site package";
            };

            webbed_site.rustToolchain = lib.mkOption {
              type = lib.types.package;
              description = "Rust toolchain to use for the webbed_site package";
              default = (pkgs.rust-bin.fromRustupToolchainFile (self + /rust-toolchain.toml)).override {
                extensions = [
                  "rust-src"
                  "rust-analyzer"
                  "clippy"
                ];
              };
            };

            webbed_site.craneLib = lib.mkOption {
              type = lib.types.lazyAttrsOf lib.types.raw;
              default = (inputs.crane.mkLib pkgs).overrideToolchain config.webbed_site.rustToolchain;
            };

            webbed_site.src = lib.mkOption {
              type = lib.types.path;
              description = "Source directory for the webbed_site package";
              # When filtering sources, we want to allow assets other than .rs files
              # TODO: Don't hardcode these!
              default = lib.cleanSourceWith {
                src = self; # The original, unfiltered source
                filter = path: type:
                  (lib.hasSuffix "\.html" path)
                  || (lib.hasSuffix "tailwind.config.js" path)
                  ||
                  # Example of a folder for images, icons, etc
                  (lib.hasInfix "/assets/" path)
                  || (lib.hasInfix "/css/" path)
                  ||
                  # Default filter from crane (allow .rs files)
                  (config.webbed_site.craneLib.filterCargoSources path type);
              };
            };
          };
          config =
            let
              cargoToml = builtins.fromTOML (builtins.readFile (self + /Cargo.toml));
              inherit (cargoToml.package) name version;
              inherit (config.webbed_site) rustToolchain craneLib src;

              # Crane builder for cargo-leptos projects
              craneBuild = rec {
                args = {
                  inherit src;
                  pname = name;
                  version = version;
                  buildInputs = [
                    pkgs.cargo-leptos
                    pkgs.binaryen # Provides wasm-opt
                    tailwindcss
                  ];
                };
                cargoArtifacts = craneLib.buildDepsOnly args;
                buildArgs =
                  args
                  // {
                    inherit cargoArtifacts;
                    cargoExtraArgs = "";
                    buildPhaseCargoCommand = "cargo leptos build --release";
                    cargoTestCommand = "cargo leptos test --release -v";
                    nativeBuildInputs = [
                      pkgs.makeWrapper
                    ];
                    installPhaseCommand = ''
                      mkdir -p $out/bin
                      cp target/release/${name} $out/bin/
                      cp -r target/site $out/bin/
                      wrapProgram $out/bin/${name} \
                        --set LEPTOS_SITE_ROOT $out/bin/site
                    '';
                  };
                package = craneLib.buildPackage (buildArgs // config.webbed_site.overrideCraneArgs buildArgs);

                check = craneLib.cargoClippy (args
                  // {
                  inherit cargoArtifacts;
                  cargoClippyExtraArgs = "--all-targets --all-features -- --deny warnings";
                });

                doc = craneLib.cargoDoc (args
                  // {
                  inherit cargoArtifacts;
                });
              };

              rustDevShell = pkgs.mkShell {
                shellHook = ''
                  # For rust-analyzer 'hover' tooltips to work.
                  export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library";
                '';
                buildInputs = [
                  pkgs.libiconv
                ];
                nativeBuildInputs = [
                  rustToolchain
                ];
              };

              tailwindcss =
                pkgs.nodePackages.tailwindcss.overrideAttrs
                  (oa: {
                    plugins = [
                      pkgs.nodePackages."@tailwindcss/aspect-ratio"
                      pkgs.nodePackages."@tailwindcss/forms"
                      pkgs.nodePackages."@tailwindcss/language-server"
                      pkgs.nodePackages."@tailwindcss/line-clamp"
                      pkgs.nodePackages."@tailwindcss/typography"
                    ];
                  });
            in
            {
              # Rust package
              packages.${name} = craneBuild.package;
              packages."${name}-doc" = craneBuild.doc;

              checks."${name}-clippy" = craneBuild.check;

              # Rust dev environment
              devShells.${name} = pkgs.mkShell {
                inputsFrom = [
                  rustDevShell
                ];
                nativeBuildInputs = with pkgs; [
                  tailwindcss
                  cargo-leptos
                  binaryen # Provides wasm-opt
                ];
              };
            };
        });
  };
}
