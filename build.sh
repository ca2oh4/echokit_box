#!/bin/bash

cargo build --release
# find srmodels.bin
find . -name srmodels.bin
# copy srmodels.bin
cp ./target/xtensa-esp32s3-espidf/release/build/esp-idf-sys-771afe2b71dc2f41/out/build/srmodels/srmodels.bin .
cp ./target/xtensa-esp32s3-espidf/release/echokit .
