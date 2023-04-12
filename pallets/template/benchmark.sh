
# ---------------------------------------------------------------------------- #
#                                Add benchmarks                                #
# ---------------------------------------------------------------------------- #

# ----------------------------- Add benchmarking to the pallet ---------------------------- #

# Import the crate from cargo.toml
# Add the frame-benchmarking crate to the [dependencies] for the pallet using the same version and branch as the other dependencies in the pallet.
# Add runtime-benchmarks to the list of [features] for the pallet.
# Add frame-benchmarking/std to the list of std features for the pallet.

# ------------------------- Add a benchmarking module ------------------------ #
# Create benchmarking.rs file and add the requried code

# ---------------------------- Test the benchmarks --------------------------- #
# Add the impl_benchmark_test_suite! macro to the bottom of your benchmarking module:
# impl_benchmark_test_suite!(
#  MyPallet,
#  crate::mock::new_test_ext(),
#  crate::mock::Test,
# );

# Execute the benchmark unit tests generated for your pallet in a mock runtime by running a command similar to the following for a pallet named pallet-mycustom:
# 
# cargo test --package pallet-mycustom --features runtime-benchmarks

# ---------------------- Add benchmarking to the runtime --------------------- #
# Add your pallet to the list of [dependencies] for the runtime:
# Update the [features] for the runtime to include the runtime-benchmarks for your pallet:
# Update the std features for the runtime to include your pallet:
# Add the configuration trait for your pallet to the runtime.
# Add the pallet the the construct_runtime! macro.
# Add your pallet to the define_benchmark! macro in the runtime-benchmarks feature.

# ---------------------------------- Run your Benchmarks ---------------------------------- #

# Then compile it:
cargo build --release --features runtime-benchmarks

# If you used the production profile to compile the node, you can list the available benchmarks by
./target/production/node-template benchmark pallet --list

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

# output file implements the WeightInfo trait for your runtime.
# output file implements the WeightInfo trait for the pallet_balances pallet.

# ----------------------------------- Link ----------------------------------- #
# https://docs.substrate.io/reference/how-to-guides/weights/add-benchmarks/

# ---------------------------- Benchmark Template ---------------------------- #

# ------------------------------------ V1 ------------------------------------ #
# #![cfg(feature = "runtime-benchmarks")]
# mod benchmarking;

use crate::*;
use frame_benchmarking::v1::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
  // Add individual benchmarks here
  benchmark_name {
     /* code to set the initial state */
  }: {
     /* code to test the function benchmarked */
  }
  verify {
     /* optional verification */
  }
}