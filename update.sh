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
fi

current_path=$(pwd)
cd tuningplayground/music21-rs/music21
git pull origin master

cd ..
python -m venv venv
source venv/bin/activate
pip install --upgrade pip
pip install -r music21/requirements.txt
pip install music21
python -m test
python -m generate_chords

cd ../tuningplayground
wasm-pack build --target web --dev
cd ../../textprocessing
wasm-pack build --target web --dev

cd $current_path
pnpx npm-upgrade
pnpm update
pnpm audit fix
pnpm install

cd ./ts
node_up src

cd ../tuningplayground/
cargo_up

cd ./ts
node_up src

cd ../tuningplayground
cargo_up

cd ../../textprocessing/
ls
cargo_up

cd ./ts
node_up src