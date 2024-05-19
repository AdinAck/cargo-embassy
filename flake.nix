{
  description = "Description for the project";

  inputs = {
    devenv-root = {
      url = "file+file:///dev/null";
      flake = false;
    };
    nixpkgs.url = "github:cachix/devenv-nixpkgs/rolling";
    devenv.url = "github:cachix/devenv";
    nix2container.url = "github:nlewo/nix2container";
    nix2container.inputs.nixpkgs.follows = "nixpkgs";
    mk-shell-bin.url = "github:rrbutani/nix-mk-shell-bin";
    crate2nix.url = "github:nix-community/crate2nix";
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = inputs@{ flake-parts, devenv-root, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.flake-parts.flakeModules.easyOverlay
        inputs.devenv.flakeModule
      ];
      systems = [ "x86_64-linux" "i686-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ];

      perSystem = { config, self', inputs', pkgs, system, ... }: let
        customBuildRustCrateForPkgs = pkgs: pkgs.buildRustCrate.override {
        defaultCrateOverrides = pkgs.defaultCrateOverrides // {
          hidapi = attrs: {
            buildInputs = [ pkgs.udev pkgs.systemd ];
            nativeBuildInputs = [ pkgs.pkg-config ];
          };
        };
      };
      cargoNix = pkgs.callPackage ./Cargo.nix {
        buildRustCrateForPkgs = customBuildRustCrateForPkgs;
      };
        rustPkgs = pkgs.rustBuilder.makePackageSet {
          # rustVersion = "1.75.0";
          packageFun = import ./Cargo.nix;
        };

        # cargoNix = inputs.crate2nix.tools.${system}.appliedCargoNix {
        #   name = "cargo-embassy";
        #   src = ./.;
        # };
      in {
        # Per-system attributes can be defined here. The self' and inputs'
        # module parameters provide easy access to attributes of the same
        # system.

        # Equivalent to  inputs'.nixpkgs.legacyPackages.hello;
        # packages.default = pkgs.callPackage ./package.nix {
        packages = rec {
          default = cargo-embassy;
          cargo-embassy = cargoNix.rootCrate.build;
        };

        overlayAttrs = {
          inherit (config.packages) cargo-embassy;
        };

        devenv.shells.default = {
          devenv.root =
            let
              devenvRootFileContent = builtins.readFile devenv-root.outPath;
            in
            pkgs.lib.mkIf (devenvRootFileContent != "") devenvRootFileContent;

          name = "my-project";

          imports = [
            # This is just like the imports in devenv.nix.
            # See https://devenv.sh/guides/using-with-flake-parts/#import-a-devenv-module
            # ./devenv-foo.nix
          ];

          # https://devenv.sh/reference/options/
          packages = [ config.packages.default ];

          enterShell = ''
            cargo embassy --help
          '';
        };

      };
      flake = {
        # The usual flake attributes can be defined here, including system-
        # agnostic ones like nixosModule and system-enumerating ones, although
        # those are more easily expressed in perSystem.

      };
    };
}
