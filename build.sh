#!/usr/bin/env bash

cd content/tools/
rm -rf tuningplayground tuningplayground_debug textprocessing
cd ../..

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

mv ../content/tools/tuningplayground/chords.* ../content/misc/media/

cd ../textprocessing
./build.sh prod
mv ./www ../content/tools/textprocessing/

cd ../ts 
pnpm install
pnpm exec tsc

cd ..
./scripts/collect_links.sh
./scripts/media.sh

pnpm install
npx quartz build