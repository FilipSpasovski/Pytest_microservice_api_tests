{ pkgs ? import (builtins.fetchTarball {
  # Descriptive name to make the store path easier to identify
  name = "nixpkgs-unstable-2021-05-15";
  # Commit hash for nixpkgs-unstable as of 2021-05-15 from https://status.nixos.org/
  url = "https://github.com/nixos/nixpkgs/archive/a2c3ea5bf825.tar.gz";
  # Hash obtained using `nix-prefetch-url --unpack <url>`
  sha256 = "0rxn9wg73gvgb7zwzrdhranlj3jpkkcnsqmrzw5m0znwv6apj6k4";
}) {}}:

pkgs.mkShell {                  # mkShell is a helper function
  name="dev-environment";       # that requires a name

  nativeBuildInputs = with pkgs; [    # for a list of packages (search https://search.nixos.org/packages)
    cargo
    pkgconfig
    rustfmt
    clippy
    rustup
    curl
    jq
  ]
  ;
  buildInputs = with pkgs; [
    libiconv
    openssl
  ]
    # dependencies of the code (on macosx)
  ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
    pkgs.darwin.apple_sdk.frameworks.Security
  ]
  ;
  shellHook = ''             # bash to run when you enter the shell
    # rustup default 1.52.1
    # echo "Start developing..."
    # echo "run 'rustup toolchain install 1.52.1-x86_64-apple-darwin'" to provide components for editors/rust-analyzer/...
  '';
}
