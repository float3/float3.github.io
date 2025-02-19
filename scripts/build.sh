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
rm -rf wasm | true  

cd $wasm_path/wasm
./build.sh $ARGS

cd $root_path
rm ./content/tools/wasm/*LICENSE.txt | true

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