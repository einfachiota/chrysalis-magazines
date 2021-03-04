#!/bin/sh
# This is a comment!
rm -f ./dist
mkdir ./dist
echo "Build de..."
mdbook build ./src/de
mkdir ./dist/de
cp -r ./src/de/book/. ./dist/magazines/de
echo "... de build finshed."
echo "Build en..."
mdbook build ./src/en
mkdir ./dist/en
cp -r ./src/en/book/. ./dist/magazines/en
echo "... en build finshed."
echo "Build index..."
mdbook build
cp -r ./book/. ./dist
echo "... index build finshed."