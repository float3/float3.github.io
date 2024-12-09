#!/usr/bin/env bash

cd "$(dirname "$0")/../content/misc/media"

cat <<EOF > index.md
---
title: media
tags:
  - list
---

EOF

files=( $(ls -p | grep -v index.md) )
count=${#files[@]}
for (( i=0; i<$count; i++ )); do
  f=${files[$i]}
  if [ $i -lt $((count-1)) ]; then
    echo "[$f](/misc/media/$f) \\" >> index.md
  else
    echo "[$f](/misc/media/$f)" >> index.md
  fi
done