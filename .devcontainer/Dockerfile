FROM mcr.microsoft.com/vscode/devcontainers/rust:latest

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN rustup component add rustfmt clippy
