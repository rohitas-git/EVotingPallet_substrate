//All of the pallets used in a runtime must be set to compile with the no_std features.
//Because they will be compiled to WASM and WASM doesn't work with std's memory management operations
//WASM needs to be defined its own memory management operations

// conditional attribute to configure compiler to run code if std feature is disable, i.e no_std
#![cfg_attr(not(feature = "std"), no_std)]

/* -------------------------------------------------------------------------- */
/*                   Custom Pallet: Proof-of-existence                        */
/* -------------------------------------------------------------------------- */ 
/*
Proof-of-existence is an approach to validating the authenticity and ownership of a digital object by storing information about the object on the blockchain. 
Because the blockchain associates a timestamp and account with the object, 
the blockchain record can be used to "prove" that a particular object existed at a specific date and time. 
It can also verify who the owner of a record was at that date and time.
*/
/* --------------------------- Design of Pallet : --------------------------- */
/*  The proof-of-existence application exposes the following callable functions:
 	create_claim() allows a user to claim the existence of a file by uploading a hash.
  	revoke_claim() allows the current owner of a claim to revoke ownership.
*/
/* ----------------------------------- End ---------------------------------- */

/* -------------------------------------------------------------------------- */
/*                          Skeleton of Custom Pallet                         */
/* -------------------------------------------------------------------------- */

/*
	pub use pallet::*;

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
*/ 
/* ----------------------------------- End ---------------------------------- */


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

	// Pallets use events to inform users when important changes are made.
	// Event documentation should end with an array that provides descriptive names for parameters.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when a claim has been created.
		ClaimCreated { who: T::AccountId, claim: T::Hash },
		/// Event emitted when a claim is revoked by the owner.
		ClaimRevoked { who: T::AccountId, claim: T::Hash },
	}
	#[pallet::error]
	pub enum Error<T> {
		/// The claim already exists.
		AlreadyClaimed,
		/// The claim does not exist, so it cannot be revoked.
		NoSuchClaim,
		/// The claim is owned by another account, so caller can't revoke it.
		NotClaimOwner,
	}

	#[pallet::storage] // <-- Step 5. code block will replace this.
	#[pallet::call]    // <-- Step 6. code block will replace this.
	}



