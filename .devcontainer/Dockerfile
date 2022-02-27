# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.205.2/containers/rust/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="buster"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends netcat \ 
    bash-completion curl git libssl-dev make pkg-config vim \
    && cargo install --force cargo-make

RUN rustup component add clippy rustfmt rust-src

RUN cargo install cargo-benchcmp devserver

# to avoid "permission denied", since we are in a container...
RUN chown -R 1000:1000 /usr/local/cargo
