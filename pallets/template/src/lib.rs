//All of the pallets used in a runtime must be set to compile with the no_std features.
//Because they will be compiled to WASM and WASM doesn't work with std's memory management operations
//WASM needs to be defined its own memory management operations

// conditional attribute to configure compiler to run code if std feature is disable, i.e no_std
#![cfg_attr(not(feature = "std"), no_std)]


// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// The pallet attribute macro defines a pallet that can be used with construct_runtime!. It must be attached to a module named pallet as follows:
//The pallet macro will parse any items within your pallet module that are annotated with #[pallet::*] attributes.
#[frame_support::pallet]
pub mod pallet {
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  #[pallet::config]  // <-- Step 2. code block will replace this.
  #[pallet::event]   // <-- Step 3. code block will replace this.
  #[pallet::error]   // <-- Step 4. code block will replace this.
  #[pallet::storage] // <-- Step 5. code block will replace this.
  #[pallet::call]    // <-- Step 6. code block will replace this.
}



