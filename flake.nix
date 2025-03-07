{
  inputs = {
    configuration.url = "github:ivandimitrov8080/configuration.nix";
    nixpkgs.follows = "configuration/nixpkgs-unstable";
  };
  outputs =
    inputs: inputs.configuration.inputs.parts.lib.mkFlake { inherit inputs; } { imports = [ ./. ]; };
}
