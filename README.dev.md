echokit
===

### build & flash

build

```bash
cargo build --release
# find srmodels.bin
find . -name srmodels.bin
# copy srmodels.bin
cp ./target/xtensa-esp32s3-espidf/release/build/esp-idf-sys-771afe2b71dc2f41/out/build/srmodels/srmodels.bin .
cp ./target/xtensa-esp32s3-espidf/release/echokit .
```

flash

```bat
espflash flash --partition-table partitions.csv --flash-size 16mb .\echokit
espflash write-bin 0x710000 .\srmodels.bin
espflash monitor
```

