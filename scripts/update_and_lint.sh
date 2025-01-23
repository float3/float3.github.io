#!/usr/bin/env bash

node_up() {
    if [[ -z "$GITHUB_ACTIONS" ]]; then
        pnpx npm-upgrade
        echo "::warning::upgrading npm packages"
    fi
    pnpm update
    pnpm audit fix
    pnpm install
    pnpx prettier $1 --write
    pnpx eslint $1 --fix
}

cargo_up() {
    cargo clean
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

cd $wasm_path/tuningplayground
wasm-pack build --target bundler $ARGS

cd $wasm_path/textprocessing
wasm-pack build --target bundler $ARGS

cd $wasm_path/glsl2hlsl/glsl2hlsl-wasm
wasm-pack build --target bundler $ARGS

cd $wasm_path/adventofcode
wasm-pack build --target bundler $ARGS


cd $root_path
if [[ -z "$GITHUB_ACTIONS" ]]; then
    pnpx npm-upgrade
    echo "::warning::upgrading npm packages"
fi
pnpm update
pnpm audit fix
pnpm install

cd ./ts
node_up src

cd $wasm_path/tuningplayground/
cargo_up

cd ./tuning_systems/
cargo_up

cd ../keymapping
cargo_up

cd ../ts
node_up src

cd $wasm_path/textprocessing/
cargo_up

cd ./hangeul_conversion
cargo_up

cd ../ts
node_up src

cd $wasm_path/glsl2hlsl/
cargo_up

cd ./glsl2hlsl-wasm/
cargo_up

cd ./ts
node_up src

cd $wasm_path/adventofcode/
cargo_up

cd ./ts
node_up src
