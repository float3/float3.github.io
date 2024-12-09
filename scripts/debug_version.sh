#!/usr/bin/env bash

cd ./content/tools

FILE="tuningplayground_debug.md"
cp tuningplayground.md "$FILE"

sed -i 's/^title: tuningplayground/title: tuningplayground debug/' "$FILE"
sed -i 's|debug version of this page: <a href="/tools/tuningplayground_debug.md">here</a>|release version of this page: <a href="/tools/tuningplayground.md">here</a>|' "$FILE"
sed -i 's|<script src="./tuningplayground/bootstrap.js"></script>|<script src="./tuningplayground_debug/bootstrap.js"></script>|' "$FILE"

