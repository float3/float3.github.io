#!/usr/bin/env bash

if [[ -z "$GITHUB_ACTIONS" ]]; then
    ARGS="dev"
    QUARTZ_ARGS="--serve"
else
    ARGS="prod"
fi

current_path=$(pwd)

cd ./content/tools/
rm -rf tuningplayground tuningplayground_debug textprocessing glsl2hlsl

cd $current_path
cd ./tuningplayground

echo "building master"
./build.sh prod
rm ./www/*.bootstrap.js.LICENSE.txt | true
mv ./www ./stable
mv ./stable ../content/tools/tuningplayground/

echo "building dev"
./build.sh dev
mv ./www ../content/tools/tuningplayground_debug/

cd $current_path
cd ./textprocessing
./build.sh $ARGS
mv ./www ../content/tools/textprocessing/

cd $current_path
cd ./glsl2hlsl
./build.sh $ARGS
mv ./glsl2hlsl-wasm/www ../content/tools/glsl2hlsl/

cd $current_path
cd ./ts 
pnpm install
pnpm exec tsc

cd $current_path
pnpm install

npx quartz build $ARGS