{
  description = "Rust + SQLx + Postgres dev environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        # Rust toolchain
        cargo
        rustc
        rustfmt
        clippy
        rust-analyzer

        # Cargo utilities
        cargo-watch
        cargo-edit
        cargo-tarpaulin
        cargo-audit

	#psql
	sqlx-cli
	postgresql

        # Native deps (common for Rust crates)
        pkg-config
        openssl

        # Optional but useful for your script
        docker
        docker-compose
      ];

      # Needed by rust-analyzer / some tooling
      RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

      shellHook = ''
        echo "Rust + SQLx dev environment loaded"
      '';
    };
  };
}
