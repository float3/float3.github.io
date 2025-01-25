#!/usr/bin/env sh

set -e

cat ./content/notes/talks.md | grep -Eo "(http|https)://[a-zA-Z0-9./?=_%:-]*" >  ./content/misc/plaintext/talks.txt
cat ./content/notes/blogs.md | grep -Eo "(http|https)://[a-zA-Z0-9./?=_%:-]*" >  ./content/misc/plaintext/blogs.txt
cat ./content/notes/graphics-resources.md | grep -Eo "(http|https)://[a-zA-Z0-9./?=_%:-]*" | sort -u >  ./content/misc/plaintext/graphics-resources.txt
