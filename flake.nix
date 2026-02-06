{
  description = "Sync Bitwarden Secrets Manager secrets to local .env files";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "vault-sync";
          version = self.shortRev or self.dirtyShortRev or "dev";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          meta = with pkgs.lib; {
            description = "Sync Bitwarden Secrets Manager secrets to local .env files";
            homepage = "https://github.com/kyeotic/vault-sync";
            license = licenses.mit;
            mainProgram = "vault-sync";
          };
        };
      }
    );
}
