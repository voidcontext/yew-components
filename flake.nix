{
  description = "A libary that contains various utils for developing web apps with yew-rs";

  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.nru.url = "github:voidcontext/nix-rust-utils";

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
            packageAttrs.preBuild = ''
              cargo build --release --example ${name} --target=wasm32-unknown-unknown

              cp target/wasm32-unknown-unknown/release/examples/${name}.wasm target/wasm32-unknown-unknown/release
            '';
          };

        serve-example-demo = name: let
          demo-src = pkgs.symlinkJoin {
            name = "${name}-demo";
            paths = [(index-html name) (example name).package];
          };
        in
          pkgs.writeShellScriptBin "serve-${name}-demo"
          ''
            ${pkgs.simple-http-server}/bin/simple-http-server \
              -p 9001                                         \
              --nocache                                       \
              -i --try-file ${demo-src}/index.html            \
              -- ${demo-src}
          '';

        serve-autocomplete-demo = serve-example-demo "autocomplete";

        check-nix-formatting = pkgs.stdenv.mkDerivation {
          name = "nix-formatting-check";
          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            # prevent re-checking the formatting when only non nix files changed
            filter = path: _type: builtins.match ".*\.nix$" path != null;
          };
          checkInputs = [pkgs.alejandra];
          checkPhase = ''alejandra . --check'';
          doCheck = true;
          installPhase = ''mkdir -p $out'';
        };
      in {
        packages.default = yew-commons.package;
        checks.default = yew-commons.package;
        checks.serve-autocomplete-demo = serve-autocomplete-demo;
        checks.nix-formatting = check-nix-formatting;

        apps.autocomplete-demo = {
          type = "app";
          program = "${serve-autocomplete-demo}/bin/serve-autocomplete-demo";
        };

        devShells.default = lib.mkDevShell yew-commons;
      }
    );
}
