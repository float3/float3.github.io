#!/usr/bin/env bash

set -e

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup.sh
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -ssf -o wasmpack.sh
curl -fsSL https://bun.sh/install -o bun.sh
curl -fsSL https://get.pnpm.io/install.sh -o pnpm.sh

chmod +x *.sh
./rustup.sh --default-toolchain nightly --profile minimal -y
rustup component add rustfmt clippy --toolchain nightly
source "$HOME/.cargo/env"
rm ~/.cargo/bin/rustfmt ~/.cargo/bin/rust-analyzer ~/.cargo/bin/cargo-fmt
rustup update
./wasmpack.sh

if [ $GITHUB_JOB == "update_and_lint" ]; then
    cargo install cargo-edit
fi

./pnpm.sh
./bun.sh