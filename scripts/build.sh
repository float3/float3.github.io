#!/usr/bin/env bash

set -e

START_TIME=$(date +%s)

if [ "$GITHUB_ACTIONS" == "true" ]; then
    ARGS="prod"
    QUARTZ_ARGS=""
else
    ARGS="dev"
    QUARTZ_ARGS="--serve"
    echo "::warning::Building in development mode."
fi

root_path=$(pwd)
cd wasm
wasm_path=$(pwd)

cd $root_path/content/tools/
rm -rf tuningplayground tuningplayground_debug textprocessing glsl2hlsl adventofcode

cd $wasm_path/tuningplayground
./build.sh $ARGS

cd $wasm_path/textprocessing
./build.sh $ARGS

cd $wasm_path/glsl2hlsl
./build.sh $ARGS

cd $wasm_path/adventofcode
./build.sh $ARGS

cd $root_path
rm ./content/tools/**/*LICENSE.txt | true

cd $root_path/ts 
pnpm install
pnpm exec tsc

cd $root_path
pnpm install

npx quartz build $QUARTZ_ARGS

END_TIME=$(date +%s)
BUILD_TIME=$((END_TIME - START_TIME))
cd ./public
../scripts/report.py "$BUILD_TIME"