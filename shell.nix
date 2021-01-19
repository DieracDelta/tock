# Shell expression for the Nix package manager
#
# This nix expression creates an environment with necessary packages installed:
#
#  * `tockloader`
#  * rust
#
# To use:
#
#  $ nix-shell
#

{ pkgs,  ... }:

with builtins;
let
  inherit (pkgs) stdenv;
  pythonPackages = stdenv.lib.fix' (self: with self; pkgs.python3Packages //
  {

    tockloader = buildPythonPackage rec {
      pname = "tockloader";
      version = "1.3.1";
      name = "${pname}-${version}";

      propagatedBuildInputs = [ argcomplete colorama crcmod pyserial pytoml ];

      src = fetchPypi {
        inherit pname version;
        sha256 = "1gralnhvl82xr7rkrmxj0c1rxn1y9dlbmkkrklcdjahragbknivn";
      };
    };
  });

  rust_date = "2021-01-07";
  rust_channel = "nightly";
  rust_targets = [
    "thumbv7em-none-eabi" "thumbv7em-none-eabihf" "thumbv6m-none-eabi"
    "riscv32imac-unknown-none-elf" "riscv32imc-unknown-none-elf" "riscv32i-unknown-none-elf"
  ];
  rust_build = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain).override {
    targets = rust_targets;
  };
in
  with pkgs;
  stdenv.mkDerivation {
    name = "tock-dev";

    buildInputs = [
      python3Full
      pythonPackages.tockloader
      llvm
      rust_build
      qemu
    ];

    LD_LIBRARY_PATH="${stdenv.cc.cc.lib}/lib64:$LD_LIBRARY_PATH";

    # The defaults "objcopy" and "objdump" are wrong (for x86), use
    # "llvm-obj{copy,dump}" as defined in the makefile
    shellHook = ''
      unset OBJCOPY
      unset OBJDUMP
    '';
  }
