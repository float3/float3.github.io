#!/usr/bin/env bash

set -e

node_up() {
    if [[ -z "$GITHUB_ACTIONS" ]]; then
        pnpx npm-upgrade
        echo "::warning::upgrading npm packages"
    fi
    pnpm update
    pnpm audit fix
    pnpm install --no-frozen-lockfile
    pnpx prettier $1 --write
    pnpx eslint $1 --fix
}

cargo_up() {
    cargo clean
    cargo upgrade
    cargo update --workspace
    cargo hack clippy --feature-powerset --fix --allow-dirty --allow-staged --all-targets --workspace -- -D warnings
    cargo hack fix --feature-powerset --allow-dirty --allow-staged --all-targets --workspace
    cargo hack check --feature-powerset --all-targets --workspace
    cargo hack test --feature-powerset --release --verbose --all-targets --workspace --no-fail-fast --lib --bins --examples --tests --benches
    cargo fmt --all
}

if [[ -z "$GITHUB_ACTIONS" ]]; then
    git pull
    ARGS=""
    echo "::warning::Building in development mode."
else
    ARGS="--release"
fi

root_path=$(pwd)
cd wasm
wasm_path=$(pwd)

cd $wasm_path/tuningplayground/music21
git pull origin master

# need to provide packages so that pnpm doesn't complain

cd $wasm_path/wasm
wasm-pack build --target bundler $ARGS


cd $root_path
if [[ -z "$GITHUB_ACTIONS" ]]; then
    pnpx npm-upgrade
    echo "::warning::upgrading npm packages"
fi
pnpm update
pnpm audit fix
pnpm install --no-frozen-lockfile

cd ./ts
node_up src

cd $wasm_path/tuningplayground/
cargo_up

cd ./tuning_systems/
cargo_up

cd ../keymapping
cargo_up

cd $wasm_path/textprocessing/
cargo_up

cd ./hangeul_conversion
cargo_up

cd $wasm_path/glsl2hlsl/
cargo_up

cd $wasm_path/adventofcode/
cargo_up

cd $wasm_path/wasm/
cargo_up

cd ./ts
node_up src