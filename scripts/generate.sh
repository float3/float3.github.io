#!/usr/bin/env bash

set -e

./scripts/collect_links.sh
./scripts/indices.sh

CD=$(pwd)

cd ./wasm/tuningplayground/

cargo run -p chord_generator

cd $CD/content
../scripts/dates.sh
