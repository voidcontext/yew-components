{
  mkCrate = {pkgs, rust, nodejs, src, name ? null}:
    let
      cargoToml = builtins.fromTOML (builtins.readFile src + "/Cargo.toml");
      rustPackages = [ rust pkgs.wasm-bindgen-cli ];
      nameAttrs =
        if name == null then {
          pname = cargoToml.package.name;
          version = cargoToml.package.version;
        }
        else {inherit name;}
      ;
    in
    pkgs.rustPlatform.buildRustPackage (nameAttrs // {

      inherit src;

      cargoLock = {
        lockFile = src + "/Cargo.lock";
      };

      nativeBuildInputs = rustPackages;

      buildPhase = ''
      cargo build --target=wasm32-unknown-unknown --release
      '';

      checkInputs = [ nodejs ];
      checkPhase = ''
      cargo test --target=wasm32-unknown-unknown
      '';

      installPhase = ''
      mkdir -p $out
      cp -r target/wasm32-unknown-unknown/* $out
      '';
    });
}
