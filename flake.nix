{
  description = "A libary that contains various utils for developing web apps with yew-rs";

  inputs.nixpkgs.url = "nixpkgs/release-23.05";
  inputs.nixpkgs-unstable.url = "nixpkgs/nixpkgs-unstable";
  inputs.nix-rust-utils.url = "git+https://git.vdx.hu/voidcontext/nix-rust-utils.git?ref=refs/tags/v0.10.0";
  inputs.nix-rust-utils.inputs.nixpkgs.follows = "nixpkgs";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  outputs = {
    nixpkgs,
    nixpkgs-unstable,
    flake-utils,
    nix-rust-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
            (final: prev: {unstable = nixpkgs-unstable.legacyPackages.${system};})
          ];
        };
        rustWithWasm32 = pkgs.rust-bin.stable."1.69.0".default.override {
          targets = ["wasm32-unknown-unknown"];
        };

        nru = nix-rust-utils.mkLib {
          inherit pkgs;
          toolchain = rustWithWasm32;
        };
        src = ./.;

        index-html = import ./index.html.nix {inherit pkgs;};

        node-packages = pkgs.callPackage ./nix/node {};

        plainViewCss = ''
          .autocomplete-item {
            padding: 0.3rem 0.5rem;
            display: block;
          }
          .autocomplete-item.highlighted {
            background: #ebfffc;;
          }
        '';

        commonArgs = {
          inherit src;
          buildInputs = [pkgs.unstable.wasm-bindgen-cli];
          # Nodejs is need by wasm-bindgen-test
          checkInputs = [pkgs.nodejs];
        };

        yew-components = nru.mkWasmCrate commonArgs;

        example = name:
        # TODO: find a better way to build examples
          nru.mkWasmCrate (commonArgs
            // {
              # Build examples in the preBuild step and copy them into the release directory so that
              # the postBuild script picks them up and generates their JS bindings
              preBuild = nru.snippets.wasm.buildExample name;
            });

        serve-example-demo = name: port: let
          demo-src = pkgs.symlinkJoin {
            name = "${name}-demo";
            paths = [(index-html name plainViewCss) (example name)];
          };
        in
          pkgs.writeShellScriptBin "serve-${name}-demo"
          (nru.snippets.utils.serve demo-src port);

        serve-autocomplete-demo = serve-example-demo "autocomplete" 9001;

        check-nix-formatting = pkgs.stdenv.mkDerivation {
          name = "nix-formatting-check";
          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            # prevent re-checking the formatting when only non nix files changed
            filter = path: _type: builtins.match ".*\.nix$" path != null;
          };
          checkInputs = [pkgs.alejandra];
          checkPhase = ''
            alejandra . --check \
              -e nix/node  # this directory contains generated code
          '';
          doCheck = true;
          installPhase = ''mkdir -p $out'';
        };

        gen-node-packages = pkgs.writeShellScriptBin "gen-node-packages" ''
          cd $WORKSPACE/nix/node
          ${pkgs.node2nix}/bin/node2nix -i node-packages.json -o node-packages.nix
        '';

        fmt = pkgs.writeShellScriptBin "fmt-nix" ''
          ${pkgs.alejandra}/bin/alejandra -e $WORKSPACE/nix/node $WORKSPACE
        '';

        cypress-bin = pkgs.cypress;

        cypress = node-packages."cypress-${pkgs.cypress.version}".overrideAttrs (old:
          {
            buildInputs = old.buildInputs ++ (pkgs.lib.optionals (system == flake-utils.lib.system.x86_64-linux) [cypress-bin]);
          }
          // (
            if system == flake-utils.lib.system.x86_64-linux
            then {
              CYPRESS_INSTALL_BINARY = 0;
            }
            else {}
          ));

        watch-autocomplete-demo = let
          watches = nru.utils.watch {
            "$WORKSPACE/yew-autocomplete/src $WORKSPACE/yew-autocomplete/examples/autocomplete" = ''
              ${nru.snippets.wasm.buildExample "autocomplete"}
              ${nru.snippets.wasm.bindgen {outDir = "$WORKSPACE/dist/lib";}}
            '';
          };
        in
          pkgs.writeShellScriptBin "watch-autocomplete-demo"
          (nru.snippets.utils.cleanupWrapper ''
            out=$WORKSPACE/dist

            mkdir -p $out/lib
            cp ${index-html "autocomplete" plainViewCss}/index.html $out
            ${watches}
            ${nru.snippets.utils.serve "$out" 9001}
          '');

        mkRunE2eTests = suffix: wrapper: let
          prefix =
            if system == flake-utils.lib.system.x86_64-linux
            then "CYPRESS_RUN_BINARY=${cypress-bin}/bin/Cypress xvfb-run "
            else "";
        in
          pkgs.writeShellApplication {
            name = "run-e2e-tests${suffix}";

            runtimeInputs =
              [
                pkgs.coreutils # timeout
                pkgs.netcat
                pkgs.procps #pkill
                serve-autocomplete-demo
                cypress
              ]
              ++ (pkgs.lib.optionals pkgs.stdenv.isLinux [pkgs.xvfb-run]);

            text = wrapper ''
              set -x -e -o pipefail
              serve-autocomplete-demo&
              echo $! >> server.pid

              # shellcheck disable=SC2016
              timeout 30 sh -c 'until nc -z $0 $1; do sleep 1; done' 0.0.0.0 9001

              ${prefix}cypress run

              # shellcheck disable=SC2046
              pkill -TERM -P $(cat server.pid)
            '';
          };

        run-e2e-tests = mkRunE2eTests "" nru.snippets.utils.cleanupWrapper;
        run-e2e-tests-ci = mkRunE2eTests "-ci" (text: text);

        mkApp = derivation: name: {
          ${name} = {
            type = "app";
            program = "${derivation}/bin/${name}";
          };
        };

        checks =
          (nru.mkWasmChecks {
            inherit src;
            crate = yew-components;
          })
          // {
            inherit
              run-e2e-tests
              run-e2e-tests-ci
              serve-autocomplete-demo
              ;
            nix-formatting = check-nix-formatting;
          };

        apps = let
          derivations = [
            (mkApp serve-autocomplete-demo "serve-autocomplete-demo")
            (mkApp run-e2e-tests "run-e2e-tests")
            (mkApp run-e2e-tests-ci "run-e2e-tests-ci")
          ];
        in
          builtins.foldl' pkgs.lib.recursiveUpdate {} derivations;
      in {
        inherit apps checks;
        packages = {
          default = yew-components;
          inherit
            run-e2e-tests
            run-e2e-tests-ci
            serve-autocomplete-demo
            check-nix-formatting
            ;
        };

        devShells.default = nru.mkDevShell {
          inputsFrom = [yew-components];
          inherit checks;
          buildInputs = [
            pkgs.node2nix
            pkgs.gnused
            pkgs.cargo-edit
            gen-node-packages
            watch-autocomplete-demo
            cypress
            fmt
            rustWithWasm32
          ];
        };
      }
    );
}
