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
            packageAttrs.checkInputs = [pkgs.nodejs];
          };
        
        example = name:
           lib.mkWasmCrate {
            pname = "yew-commons-${name}-demo";
            version = "0.1.0";
            src = ./.;
            packageAttrs.checkInputs = [pkgs.nodejs];
            packageAttrs.preBuild = '' 
              cargo build --release --example ${name} --target=wasm32-unknown-unknown

              cp target/wasm32-unknown-unknown/release/examples/${name}.wasm target/wasm32-unknown-unknown/release
            '';
          };

        serve-example-demo = name:
          let 
          demo-src = pkgs.symlinkJoin {
            name = "${name}-demo";
            paths = [(index-html name) (example  name).package];
          };
          in pkgs.writeShellScriptBin "serve-${name}-demo"
          ''
            set -x
            ${pkgs.simple-http-server}/bin/simple-http-server -p 9001 -i --try-file ${demo-src}/index.html --nocache -- ${demo-src}
          '';

        serve-autocomplete-demo = serve-example-demo "autocomplete";
      in {
        packages.default = yew-commons.package;
        checks.defailt = yew-commons.package;
        checks.serve-autocomplete-demo = serve-autocomplete-demo;

        apps.autocomplete-demo = {
          type = "app";
          program = "${serve-autocomplete-demo}/bin/serve-autocomplete-demo";
        };

        devShells.default = lib.mkDevShell yew-commons;
        
      }
      );


}
