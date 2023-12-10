#!/bin/bash
EXAMPLES=(fixed_length_bridge fixed_length_pendulum leq_length_pendulum pseudo_string pulley springs_double_pendulum)
cargo build --examples --release --target=wasm32-unknown-unknown

mkdir -p out/examples
for example in ${EXAMPLES[*]}
do
  echo "Running bindgen for $example"
  ~/.cargo/bin/wasm-bindgen --no-typescript --target web \
    --out-dir ./out/examples/$example \
    --out-name "$example" \
    ./target/wasm32-unknown-unknown/release/examples/$example.wasm
done
