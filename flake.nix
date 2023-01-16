{
  description = "A libary that contains various utils for developing web apps with yew-rs";

  inputs.nru.url = "github:voidcontext/nix-rust-utils";

  outputs = {nru, ...}:
    nru.lib.mkWasmOutputs ({pkgs, ...}: {
      pname = "yew-commons-rs";
      version = "0.1.0";
      src = ./.;
      packageAttrs.checkInputs = [pkgs.nodejs];
    });
}
