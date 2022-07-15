{
  description = "A libary that contains various utils for developing web apps with yew-rs";

  inputs.nixpkgs.url = "nixpkgs/release-22.05";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.rust-overlay.inputs.nixpkgs.follows = "nixpkgs";


  outputs = { self, ... }@inputs:
    inputs.flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ inputs.rust-overlay.overlays.default ];

        pkgs = import inputs.nixpkgs { inherit system overlays; };

        rust = pkgs.rust-bin.stable."1.61.0".default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };

        rustPackages = [
          rust
          pkgs.wasm-bindgen-cli
        ];

        nodejs = pkgs.nodejs-14_x;

        ycLib = import ./nix/lib.nix;

        testCommand = "test --target=wasm32-unknown-unknown";

        cargo-with-deps = pkgs.writeShellScriptBin "yew-commons-cargo-with-deps" ''
          # deps:
          # - wasm-bindgen-cli ${pkgs.wasm-bindgen-cli}
          # - nodejs ${nodejs}
          ${pkgs.cargo}/bin/cargo $@
        '';

        wasm-test = pkgs.writeShellScriptBin "$yew-commons-wasm-test" ''
          ${cargo-with-deps}/bin/yew-commons-cargo-with-deps ${testCommand}
        '';
      in
      {
        apps.cargo = {
          type = "app";
          program = "${cargo-with-deps}/bin/yew-commons-cargo-with-deps";
        };

        apps.wasm-test = {
          type = "app";
          program = "${wasm-test}/bin/yew-commons-wasm-test";
        };

        packages.default = ycLib.mkCrate {
          inherit pkgs rust nodejs;
          src = ./.;
          name = "yew-commons-rs";
        };

        packages.yew-commons = ycLib.mkCrate {
          inherit pkgs rust nodejs;
          src = ./yew-commons;
          name = "yew-commons";
        };

        packages.yew-autocomplete = ycLib.mkCrate {
          inherit pkgs rust nodejs;
          src = ./yew-autocomplete;
          name = "yew-autocomplete";
        };

        devShells.default = pkgs.mkShell {
          packages = rustPackages ++ [
            nodejs
            pkgs.rust-analyzer
          ];
        };
      }
    );
}
