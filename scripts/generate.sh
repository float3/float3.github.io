#!/usr/bin/env bash

set -e

./scripts/collect_links.sh
./scripts/indices.sh

CD=$(pwd)

cd ./wasm/tuningplayground/

python -m venv venv
source venv/bin/activate
pip install --upgrade pip
pip install -r music21/requirements.txt
python -m generate_chords

cd $CD/content
../scripts/dates.sh