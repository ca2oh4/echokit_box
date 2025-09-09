#!/bin/bash

cargo build --release
# find srmodels.bin
srmodels_bin_path=`find . -name srmodels.bin | grep release`
echo $srmodels_bin_path
# copy srmodels.bin
cp -f $srmodels_bin_path .
cp -f ./target/xtensa-esp32s3-espidf/release/echokit .

ls -lah
