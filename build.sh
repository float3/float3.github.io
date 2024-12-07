#!/usr/bin/env bash

rm -rf content/piano/debug content/piano/tuningplayground/
rm -rf tuningplayground/www tuningplayground/www-dev

cd tuningplayground

echo "building master"
./build.sh prod
rm ./www/163.bootstrap.js.LICENSE.txt | true
mv ./www ./stable

echo "building dev"
./build.sh dev
rm ./www/chords.json | true
rm ./www/chords.txt | true
mv ./www ../content/piano/debug
mv ./stable ../content/piano/tuningplayground/
mv ../content/piano/tuningplayground/chords.* ../content/piano/

cd ../ts 
pnpm install
npx tsc
cd ..
sh ./scripts/collect_links.sh

pnpm install
npx quartz build