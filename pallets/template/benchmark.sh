### Benchmarking

# Testing benchmarks
# fn test_benchmark_[benchmark_name]<T>::() -> Result<(), &'static str>

# Run the unit tests with benchmarks
cargo test --package pallet-balances --features runtime-benchmarks

## Adding benchmarks
# define_benchmarks! macro
# Compile the node
cargo build --release --features runtime-benchmarks
cargo build --profile=production --features runtime-benchmarks

## Running them
# If you used the production profile to compile the node, you can list the available benchmarks by
./target/production/node-template benchmark pallet --list

## Benchmark all functions in all pallets
# To execute all benchmarks for the runtime
./target/production/node-template benchmark pallet \
    --chain dev \
    --execution=wasm \
    --wasm-execution=compiled \
    --pallet "*" \
    --extrinsic "*" \
    --steps 50 \
    --repeat 20 \
    --output pallets/all-weight.rs 
# output file implements the WeightInfo trait for your runtime.

## Benchmark a specific functions in a pallet
./target/production/node-template benchmark pallet \
    --chain dev \
    --execution=wasm \
    --wasm-execution=compiled \
    --pallet pallet_balances \
    --extrinsic transfer \
    --steps 50 \
    --repeat 20 \
    --output pallets/transfer-weight.rs
# output file implements the WeightInfo trait for the pallet_balances pallet.

## Use a template to format benchmarks
# To get a full list of benchmark subcommands, run:
./target/production/node-template benchmark --help

# To get a full list of available options for the benchmark pallet subcommand, run:
./target/production/node-template benchmark pallet --help