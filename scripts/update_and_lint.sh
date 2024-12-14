#!/usr/bin/env bash

node_up() {
    if [[ -z "$GITHUB_ACTIONS" ]]; then
        pnpx npm-upgrade
    fi
    pnpm update
    pnpm audit fix
    pnpm install
    pnpx prettier $1 --write
    pnpx eslint $1 --fix
}

cargo_up() {
    cargo upgrade
    cargo update --workspace
    cargo clippy --fix --allow-dirty --allow-staged --all-targets --all-features --workspace -- -D warnings
    cargo fix --allow-dirty --allow-staged --all-targets --all-features --workspace
    cargo fmt --all
    cargo check --all-targets --all-features --workspace
    cargo test --all-targets --all-features --workspace
}

if [[ -z "$GITHUB_ACTIONS" ]]; then
    git pull
    ARGS=""
else
    ARGS="--release"
fi

root_path=$(pwd)
cd tuningplayground/music21-rs/music21
git pull origin master

# need to provide packages so that pnpm doesn't complain

cd $root_path
cd ./tuningplayground
wasm-pack build --target bundler $ARGS

cd $root_path
cd ./textprocessing
wasm-pack build --target bundler $ARGS

cd $root_path
cd ./glsl2hlsl/glsl2hlsl-wasm
wasm-pack build --target bundler $ARGS

cd $root_path
pnpx npm-upgrade
pnpm update
pnpm audit fix
pnpm install

cd ./ts
node_up src

cd $root_path

cd ./tuningplayground/
cargo_up

cd ./tuning_systems/
cargo_up

cd ../keymapping
cargo_up

cd ../music21-rs
cargo_up

cd ../ts
node_up src

cd $root_path

cd ./textprocessing/
cargo_up

cd ./ts
node_up src

cd $root_path

cd ./glsl2hlsl/
cargo_up

cd ./glsl2hlsl-wasm/
cargo_up

cd ./ts
node_up src