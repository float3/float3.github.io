#!/usr/bin/env bash

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup.sh
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -ssf -o init.sh
chmod +x rustup.sh init.sh
./rustup.sh --default-toolchain nightly --profile minimal -y
rustup component add rustfmt clippy --toolchain nightly
source "$HOME/.cargo/env"
rm ~/.cargo/bin/rustfmt ~/.cargo/bin/rust-analyzer ~/.cargo/bin/cargo-fmt
rustup update
./init.sh

if [ "$GITHUB_JOB" == "Update"]; then
    cargo install cargo-edit
fi

npm install -g pnpm
curl -fsSL https://bun.sh/install | bash