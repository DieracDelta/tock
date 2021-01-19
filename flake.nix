{
  description = "a tock nix flake";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
    (system:
     let pkgs = import nixpkgs {
       inherit system;
       overlays = [rust-overlay.overlay];
     }; in
     {
       devShell = import ./shell.nix { inherit pkgs; };
     }
    );

}
