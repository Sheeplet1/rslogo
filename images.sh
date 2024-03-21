#!/bin/bash

cd logo_examples || exit

rm -rf images

for file in *.lg; do
	prefix="$(echo "$file" | sed 's/\(.*\)\..*/\1/')"
	cargo run "$file" "${prefix}_mine.svg" 500 500
done

mkdir images
mv *.svg images
cd - || exit
