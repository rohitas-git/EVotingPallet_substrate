#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use crate::pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {

	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[derive(Encode, Decode, Clone, PartialEq, Default, TypeInfo)]
	pub struct UserInfo {
		/// Username stored as an array of bytes
		pub username: Vec<u8>,
		/// Number id of the user
		pub id: i64,
		/// The "About Me" section of the user
		pub about_me: Vec<u8>,
	}

	/// Mapping of account ids to UserInfo.
	#[pallet::storage]
	#[pallet::getter(fn info)]
	pub type AccountIdToUserInfo<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, UserInfo, ValueQuery>;

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Indicates a user has been registered.
		UserCreated { user: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		AboutMeTooLong,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// Dispatchable calls go here!
		// Register a new user and change the state of the chain.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		#[pallet::call_index(0)]
		pub fn register_user(
			origin: OriginFor<T>,
			username: Vec<u8>,
			id: i64,
			about_me: Vec<u8>,
		) -> DispatchResult {
			// Gets the caller or signer of the function.
			let sender = ensure_signed(origin)?;
			// Define a new user in accordance to UserInfo.
			let new_user = UserInfo { username, id, about_me };
			// Change the state of our storage mapping by adding user info to our sender AccountId.
			<AccountIdToUserInfo<T>>::insert(&sender, new_user);
			// Emit an event indicating the user is now created and registered.
			Self::deposit_event(Event::<T>::UserCreated { user: sender });
			Ok(())
		}
	}
}
