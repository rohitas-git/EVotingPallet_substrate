// macro required to build both the native Rust binary (std) and the WebAssembly (no_std) binary.
#![cfg_attr(not(feature = "std"), no_std)]

mod voters;
mod candidates;
mod admins;



pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

use crate::voters;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type VoteCount;
		type Candidate;
		type Voter;
		type Admin;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type CandidatesList<T> = StorageValue<_, Vec<u32> >;
	pub type VotersList<T> = StorageValue<_, Vec<voters> >;
	pub type VotesCount<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		
		SuccessfulVote { who: T::AccountId },

		CandidateRegistered {who: Candidate},
		VoterRegistered {who: Voter},

		GiveVote {from: Voter, whom: Candidate},
		FetchVotes {whose: Candidate},
		CandidateInfo {whose:Candidate},
		VerifyVotes {by: Admin, whom: Candidate}

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,

		AlreadyVoted,
		///
		InvalidOperation
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		
		// #[pallet::weight(0)]
		#[pallet::call_index(0)]
		pub fn register_candidate(origin: OriginFor<T>, info: Candidate)-> DispatchResult{
			let candidate = ensure_signed(origin)?;
		}
	}
}
