#!/usr/bin/env bash


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
rm ./www/*.bootstrap.js.LICENSE.txt | true
mv ./www $root_path/content/tools/tuningplayground/

cd $wasm_path/textprocessing
./build.sh $ARGS
mv ./www $root_path/content/tools/textprocessing/

cd $wasm_path/glsl2hlsl
./build.sh $ARGS
mv ./glsl2hlsl-wasm/www $root_path/content/tools/glsl2hlsl/

cd $wasm_path/adventofcode
./build.sh $ARGS
mv ./www $root_path/content/tools/adventofcode/

cd $root_path/ts 
pnpm install
pnpm exec tsc

cd $root_path
pnpm install

npx quartz build $QUARTZ_ARGS
