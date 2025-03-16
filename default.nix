{ inputs, ... }:
{
  systems = [ "x86_64-linux" ];
  perSystem =
    { system, pkgs, ... }:
    {
      config = {
        _module.args = {
          pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ inputs.configuration.overlays.default ];
          };
        };
        devShells = {
          default = pkgs.mkShell {
            env = {
              LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (
                with pkgs;
                [
                  libudev-zero
                  alsa-lib
                  vulkan-loader
                  libxkbcommon
                  wayland
                  xorg.libX11
                  xorg.libXcursor
                  xorg.libXi
                  xorg.libXrandr
                ]
              );
              AMD_VULKAN_ICD = "RADV";
            };
            buildInputs = with pkgs; [
              rustc
              rust-analyzer
              rustfmt
              cargo
              pkg-config
              libudev-zero
              alsa-lib
              vulkan-loader
              libxkbcommon
              wayland
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr
              (nvim.extend {
                plugins = {
                  avante = {
                    enable = true;
                    # settings = {
                    #   provider = "ollama";
                    #   vendors = {
                    #     ollama = {
                    #       model = "qwen14b:latest";
                    #     };
                    #   };
                    # };
                    settings = {
                      provider = "openai";
                    };
                  };
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
        };
        packages.default = pkgs.rustPlatform.buildRustPackage {
          name = "wvl";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };
      };
    };
}
