{
  description = "A libary that contains various utils for developing web apps with yew-rs";

  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.nru.url = "github:voidcontext/nix-rust-utils/v0.4.1";

  outputs = {
    flake-utils,
    nru,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        lib = nru.lib.${system};
        pkgs = nru.env.${system}.pkgs;

        index-html = import ./index.html.nix {inherit pkgs;};

        node-packages = pkgs.callPackage ./nix/node {};

        plainViewCss = ''
          li.autocomplete-item.highlighted {
            background: gray;
          }
        '';

        yew-commons = lib.mkWasmCrate {
          pname = "yew-commons-all";
          version = "0.1.0";
          src = ./.;
          # Nodejs is need by wasm-bindgen-test
          packageAttrs.checkInputs = [pkgs.nodejs];
        };

        example = name:
        # TODO: find a better way to build examples
          lib.mkWasmCrate {
            pname = "yew-commons-${name}-demo";
            version = "0.1.0";
            src = ./.;
            packageAttrs.checkInputs = [pkgs.nodejs];
            # Build examples in the preBuild step and copy them into the release directory so that
            # the postBuild script picks them up and generates their JS bindings
            packageAttrs.preBuild = lib.snippets.wasm.buildExample name;
          };

        serve-example-demo = name: port: let
          demo-src = pkgs.symlinkJoin {
            name = "${name}-demo";
            paths = [(index-html name plainViewCss) (example name).package];
          };
        in
          pkgs.writeShellScriptBin "serve-${name}-demo"
          (lib.snippets.utils.serve demo-src port);

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

        cypress-bin = pkgs.cypress.overrideAttrs (old: let
          version = "12.3.0";
        in {
          inherit version;

          src = pkgs.fetchzip {
            url = "https://cdn.cypress.io/desktop/${version}/linux-x64/cypress.zip";
            sha256 = "sha256-RhPH/MBF8lqXeFEm2sd73Z55jgcl45VsmRWtAhckrP0=";
          };
        });

        cypress = node-packages."cypress-12.3.x".overrideAttrs (old:
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
          watches = lib.utils.watch {
            "$WORKSPACE/yew-autocomplete/src $WORKSPACE/yew-autocomplete/examples/autocomplete" = ''
              ${lib.snippets.wasm.buildExample "autocomplete"}
              ${lib.snippets.wasm.bindgen {outDir = "$WORKSPACE/dist/lib";}}
            '';
          };
        in
          pkgs.writeShellScriptBin "watch-autocomplete-demo"
          (lib.snippets.utils.cleanupWrapper ''
            out=$WORKSPACE/dist

            mkdir -p $out/lib
            cp ${index-html "autocomplete" plainViewCss}/index.html $out
            ${watches}
            ${lib.snippets.utils.serve "$out" 9001}
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
                serve-autocomplete-demo
                cypress
              ]
              ++ (pkgs.lib.optionals pkgs.stdenv.isLinux [pkgs.xvfb-run]);

            text = wrapper ''
              set -e -o pipefail
              serve-autocomplete-demo&

              # shellcheck disable=SC2016
              timeout 30 sh -c 'until nc -z $0 $1; do sleep 1; done' 0.0.0.0 9001

              ${prefix}cypress run
            '';
          };

        run-e2e-tests = mkRunE2eTests "" lib.snippets.utils.cleanupWrapper;
        run-e2e-tests-ci = mkRunE2eTests "-ci" lib.snippets.utils.cleanupWrapper;

        mkApp = derivation: name: {
          apps.${name} = {
            type = "app";
            program = "${derivation}/bin/${name}";
          };
        };

        apps = let
          derivations = [
            (mkApp serve-autocomplete-demo "serve-autocomplete-demo")
            (mkApp run-e2e-tests "run-e2e-tests")
            (mkApp run-e2e-tests-ci "run-e2e-tests-ci")
          ];
        in
          builtins.foldl' pkgs.lib.recursiveUpdate {} derivations;
      in (apps
        // {
          packages.default = yew-commons.package;

          checks.default = yew-commons.package;
          checks.run-e2e-tests = run-e2e-tests;
          checks.run-e2e-tests-ci = run-e2e-tests-ci;
          checks.serve-autocomplete-demo = serve-autocomplete-demo;
          checks.nix-formatting = check-nix-formatting;

          devShells.default = (lib.mkDevShell yew-commons).overrideAttrs (old: {
            buildInputs =
              old.buildInputs
              ++ [
                pkgs.node2nix
                gen-node-packages
                watch-autocomplete-demo
                cypress
                fmt
              ];
          });
        })
    );
}
