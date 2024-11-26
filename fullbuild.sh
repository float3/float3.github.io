#!/usr/bin/env bash

rm -rf content/piano/dev content/piano/stable
rm -rf tuningplayground/ts/dist tuningplayground/www tuningplayground/www-dev
cd tuningplayground
./lint.sh
echo "building master"
./build.sh prod
rm ./www/chords.json | true
rm ./www/163.bootstrap.js.LICENSE.txt | true
mv ./www ./stable
echo "building dev"
./build.sh dev
rm ./www/chords.json | true
mv ./www ./dev
mv ./stable ./dev ../content/piano/
cd ..
sh build.sh