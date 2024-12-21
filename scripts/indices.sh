#!/usr/bin/env bash

generate_index() {
    local dir="$1"
    local title="$2"

    local pwd=$(pwd)

    cd "./content/misc/$dir"
    cat <<EOF > index.md
---
title: $title
tags:
  - list
---

EOF

    files=( $(ls -p | grep -v index.md) )
    count=${#files[@]}
    for (( i=0; i<$count; i++ )); do
      f=${files[$i]}
      if [ $i -lt $((count-1)) ]; then
        echo "[$f](/misc/$dir/$f) \\" >> index.md
      else
        echo "[$f](/misc/$dir/$f)" >> index.md
      fi
    done

    cd $pwd
}

generate_index "media" "media"
generate_index "blobs" "blobs"
generate_index "plaintext" "plaintext"
generate_index "trolley" "trolley"

sed -i "1s/.*/const NUM = $(($(ls content/misc/trolley -1 | wc -l) - 2));/" ts/src/trolley.ts