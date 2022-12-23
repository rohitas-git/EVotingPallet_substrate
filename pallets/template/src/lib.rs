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
	//To generate a Store trait associating all storages
	//More precisely, the Store trait contains an associated type for each storage. It is implemented for Pallet allowing access to the storage from pallet struct.
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::event]   // <-- Step 3. code block will replace this.
	#[pallet::error]   // <-- Step 4. code block will replace this.
	#[pallet::storage] // <-- Step 5. code block will replace this.
	#[pallet::call]    // <-- Step 6. code block will replace this.
	}



