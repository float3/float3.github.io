#!/usr/bin/env bash

rm -rf content/piano/debug content/piano/tuningplayground/
rm -rf tuningplayground/www tuningplayground/www-dev
          cd tuningplayground
          sh lint.sh
          echo "building master"
          ./build.sh prod
          rm ./www/163.bootstrap.js.LICENSE.txt | true
          mv ./www ./stable
          echo "building dev"
          ./build.sh dev
          rm ./www/chords.json | true
          rm ./www/chords.txt | true
          mv ./www ./debug
          mv ./debug ../content/piano/
          mv ./stable ../content/piano/tuningplayground/
          mv ../content/piano/tuningplayground/chords.* ../content/piano/
cd ..
sh build.sh