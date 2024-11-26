#!/usr/bin/env bash

rm -rf content/piano/debug content/piano/tuningplayground/
rm -rf tuningplayground/www tuningplayground/www-dev
cd tuningplayground
echo "building master"
./build.sh prod
rm ./www/chords.json | true
rm ./www/163.bootstrap.js.LICENSE.txt | true
mv ./www ./stable
echo "building dev"
./build.sh dev
rm ./www/chords.json | true
mv ./www ./debug
mv ./debug ../content/piano/
mv ./stable ../content/piano/tuningplayground/
cd ..
sh build.sh