#!/usr/bin/env bash

if [[ -z "$GITHUB_ACTIONS" ]]; then
    ARGS="dev"
    QUARTZ_ARGS="--serve"
else
    ARGS="prod"
fi

root_path=$(pwd)
cd wasm
wasm_path=$(pwd)

cd $root_path/content/tools/
rm -rf tuningplayground tuningplayground_debug textprocessing glsl2hlsl

cd $wasm_path/tuningplayground

echo "building master"
./build.sh prod
rm ./www/*.bootstrap.js.LICENSE.txt | true
mv ./www ./stable
mv ./stable $root_path/content/tools/tuningplayground/

echo "building dev"
./build.sh dev
mv ./www $root_path/content/tools/tuningplayground_debug/

cd $wasm_path/textprocessing
./build.sh $ARGS
mv ./www $root_path/content/tools/textprocessing/

cd $wasm_path/glsl2hlsl
./build.sh $ARGS
mv ./glsl2hlsl-wasm/www $root_path/content/tools/glsl2hlsl/

cd $root_path/ts 
pnpm install
pnpm exec tsc

cd $root_path
pnpm install

npx quartz build $QUARTZ_ARGS