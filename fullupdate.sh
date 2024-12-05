#!/usr/bin/env bash

git pull
npm update
cd ts
npm update
cd ../tuningplayground
sh lint.sh
