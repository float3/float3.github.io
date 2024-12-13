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

current_path=$(pwd)
cd tuningplayground/music21-rs/music21
git pull origin master

# need to provide packages so that pnpm doesn't complain

cd $current_path
cd ./tuningplayground
wasm-pack build --target web $ARGS

cd $current_path
cd ../textprocessing
wasm-pack build --target web $ARGS

cd $current_path
cd ./glsl2hlsl/glsl2hlsl-wasm
wasm-pack build --target web $ARGS

cd $current_path
pnpx npm-upgrade
pnpm update
pnpm audit fix
pnpm install

cd ./ts
node_up src

cd $current_path

cd ./tuningplayground/
cargo_up

cd ./ts
node_up src

cd $current_path

cd ./textprocessing/
cargo_up

cd ./ts
node_up src

cd $current_path

cd ./glsl2hlsl/
cargo_up

cd ./glsl2hlsl-wasm/
cargo_up

cd ./ts
node_up src