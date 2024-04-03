with (import <nixpkgs> { });

mkShell {
  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ probe-rs ];
}
