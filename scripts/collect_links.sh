#!/bin/sh
cat ./content/thoughts/talks.md | grep -Eo "(http|https)://[a-zA-Z0-9./?=_%:-]*" >  ./content/misc/plaintext/talks.txt
cat ./content/thoughts/blogs.md | grep -Eo "(http|https)://[a-zA-Z0-9./?=_%:-]*" >  ./content/misc/plaintext/blogs.txt
cat ./content/thoughts/graphics-resources.md | grep -Eo "(http|https)://[a-zA-Z0-9./?=_%:-]*" | sort -u >  ./content/misc/plaintext/graphics-resources.txt
