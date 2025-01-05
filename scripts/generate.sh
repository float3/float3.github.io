#!/usr/bin/env bash

./scripts/collect_links.sh
./scripts/indices.sh

CD=$(pwd)

cd ./wasm/tuningplayground/music21-rs/

python -m venv venv
source venv/bin/activate
pip install --upgrade pip
pip install -r music21/requirements.txt
pip install music21
python -m generate_chords

cd $CD/content
../scripts/dates.sh