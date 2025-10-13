# default.nix
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
    nativeBuildInputs = with pkgs; [
        #llvmPackages.bintools #Not sure this is needed
        #clang #Not sure this is needed
        #cargo #Needed for rust
        #rustc #Needed for rust
        #rustup #Not sure this is needed
        #lldb #Needed for debugging

        #From nixos.wiki/wiki/Rust
        rustc
        cargo
        gcc
        rustfmt
        clippy 
    ];

    # Certain Rust tools won't work without this
    # This can also be fixed by using oxalica/rust-overlay and specifying the rust-src extension
    # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela. for more details.
    # RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}