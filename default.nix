with import <nixpkgs> {};
stdenv.mkDerivation rec {
  name = "env";

  # Mandatory boilerplate for buildable env
  env = buildEnv { name = name; paths = buildInputs; };

  # Customizable development requirements
  builder = builtins.toFile "builder.sh" ''
    source $stdenv/setup; ln -s $env $out
  '';
  buildInputs = [
    # Add packages from nix-env -qaP | grep -i needle queries
    rustup
    cmake
    git
    gcc
  ];

}
