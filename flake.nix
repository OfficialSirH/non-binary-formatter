{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix }:
    let
      systems = ["x86_64-linux" "aarch64-linux"];
      
      forAllSystems = f:
        nixpkgs.lib.genAttrs systems (system:
          f {
            pkgs =
              import nixpkgs {
                inherit system;
                overlays = [self.overlays.default];
              };
          });
  
    in {
      overlays.default = final: prev: {
			rustToolchain = with fenix.packages.${prev.stdenv.hostPlatform.system};
				combine (
					(with stable; [clippy rustc cargo rust-src rust-analyzer])
					++ [default.rustfmt]
				);
    	};
		      
      packages =
      			forAllSystems ({pkgs}: {
      					default =
      						(pkgs.makeRustPlatform {
      								cargo = pkgs.rustToolchain;
      								rustc = pkgs.rustToolchain;
      							}).buildRustPackage {
      							pname = "non-binary-formatter";
      							version = "0.0.1";
      							src = ./.;
      							cargoLock.lockFile = ./Cargo.lock;
      						};
      				});
      				      
			devShells = forAllSystems (
        { pkgs }:
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              rustToolchain
              pkg-config
              cargo-deny
              cargo-edit
              cargo-semver-checks
              cargo-watch
              cargo-show-asm
              bacon
            ];

            buildInputs = with pkgs; [
              gcc.cc.lib
            ];

            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath (with pkgs; [gcc.cc.lib])}:$LD_LIBRARY_PATH";
            RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
          };
        }
      );

    };
}
