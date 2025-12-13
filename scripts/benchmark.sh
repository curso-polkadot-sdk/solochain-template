#!/bin/sh

# check for required programs
command -v dirname > /dev/null 2>&1 || { echo >&2 "'dirname' not found"; exit 1; }
command -v cargo > /dev/null 2>&1 || { echo >&2 "'cargo' not found"; exit 1; }

# go to project root directory
cd -- "$(dirname "${0}")" || exit 1
cd .. || exit 1

# check if the binary exists, if not, build it
if test ! -x ./target/release/solochain-template-node; then
	echo 'solochain-template-node not found, building...'
	cargo build --release --features=runtime-benchmarks -p solochain-template-node
fi

# run the benchmarks
./target/release/solochain-template-node benchmark pallet \
	--chain=dev \
	--pallet=pallet_template \
	--extrinsic='*' \
	--steps=50 \
	--repeat=200 \
	--wasm-execution=compiled \
	--output='pallets/template/src/weights.rs' \
	--template='./.maintain/frame-weight-template.hbs'
