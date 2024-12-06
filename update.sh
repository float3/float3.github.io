#!/usr/bin/env bash

node() {
    pnpx npm-upgrade
    pnpm update
    pnpm audit fix --force
    pnpx prettier . --write
    pnpx eslint . --fix
}

cargo() {
    cargo upgrade -i allow
    cargo update --workspace
    cargo clippy --fix --allow-dirty --allow-staged --all-targets --all-features --workspace -- -D warnings
    cargo fix --allow-dirty --allow-staged --all-targets --all-features --workspace
    cargo fmt --all
    cargo check --all-targets --all-features --workspace
    cargo test --all-targets --all-features --workspace
}

if [[ -z "$GITHUB_ACTIONS" ]]; then
    git pull
fi

current_path=$(pwd)
cd tuningplayground/music21-rs/music21
git pull origin master

cd $current_path
node()

cd ./ts
node()

cd ../tuningplayground/ts
node()

cd ../../textprocessing/ts
node()

cd $current_path

cd ./tuningplayground
cd ./music21-rs
python3 -m venv venv
. venv/bin/activate
pip3 install -r music21/requirements.txt
pip3 install music21
python3 -m test
python3 -m generate_chords

cargo()

cd ../tuningplayground
cargo()
wasm-pack build --target web --dev


