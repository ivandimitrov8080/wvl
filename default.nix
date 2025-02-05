{ inputs, ... }:
{
  systems = [ "x86_64-linux" ];
  perSystem =
    { system, pkgs, ... }:
    {
      config = {
        _module.args = {
          pkgs = import inputs.configuration.inputs.nixpkgs {
            inherit system;
            overlays = [ inputs.configuration.overlays.default ];
          };
        };
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            rust-analyzer
            rustfmt
            cargo
            (nvim.extend {
              plugins = {
                lsp.servers = {
                  rust_analyzer = {
                    installCargo = false;
                    installRustc = false;
                  };
                };
                rustaceanvim = {
                  enable = true;
                };
              };
            })
          ];
        };
        packages.default = pkgs.stdenv.mkDerivation { };
      };
    };
}
