#!/bin/bash

cd logo_examples

for file in *.lg; do
	prefix = $(echo $file | sed 's/\(.*\)\..*/\1/')
	6991 rslogo $file ${prefix}_actual.svg 500 500
done

mkdir correct_images
mv *.svg correct_images
cd -
