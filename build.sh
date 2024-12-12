#!/usr/bin/env bash

current_path=$(pwd)

cd content/tools/
rm -rf tuningplayground tuningplayground_debug textprocessing glsl2hlsl
cd $current_path

cd tuningplayground

echo "building master"
./build.sh prod
rm ./www/*.bootstrap.js.LICENSE.txt | true
mv ./www ./stable
mv ./stable ../content/tools/tuningplayground/

echo "building dev"
./build.sh dev
rm ./www/chords.* | true
mv ./www ../content/tools/tuningplayground_debug/

mv ../content/tools/tuningplayground/chords.* ../content/misc/plaintext/

cd ../textprocessing
./build.sh prod
mv ./www ../content/tools/textprocessing/

cd ../glsl2hlsl
./build.sh prod
mv ./glsl2hlsl-wasm/www ../content/tools/glsl2hlsl/


cd ../ts 
pnpm install
pnpm exec tsc

cd ..
./scripts/collect_links.sh
./scripts/indices.sh
./scripts/debug_version.sh

pnpm install

if [[ -z "$GITHUB_ACTIONS" ]]; then
    ARGS="--serve"
else
    ARGS=""
fi

npx quartz build $ARGS