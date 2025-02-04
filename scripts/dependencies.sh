#!/usr/bin/env bash

set -e

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup.sh
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
curl -fsSL https://bun.sh/install | bash
curl -fsSL https://get.pnpm.io/install.sh | sh -

chmod +x rustup.sh

bash ./rustup.sh --default-toolchain nightly --profile minimal -y

rustup component add rustfmt clippy --toolchain nightly

source "$HOME/.cargo/env"

rm ~/.cargo/bin/rustfmt ~/.cargo/bin/rust-analyzer ~/.cargo/bin/cargo-fmt

rustup update

if [ $GITHUB_JOB == "update_and_lint" ]; then
    cargo install cargo-edit
fi