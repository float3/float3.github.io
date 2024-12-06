#!/usr/bin/env bash

cargo update --verbose
cargo clippy --fix --allow-dirty --allow-staged --all-targets --all-features --verbose -- -D warnings
cargo fix --allow-dirty --allow-staged --all-targets --all-features --verbose
cargo fmt --all --verbose
cargo check --all-targets --all-features --verbose
cargo test --all-targets --all-features --verbose

wasm-pack build --target web --dev

cd ts
pnpm update
pnpm audit fix --force
pnpx prettier . --write
pnpx eslint . --fix
