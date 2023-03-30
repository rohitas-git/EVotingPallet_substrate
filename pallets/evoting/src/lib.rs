// macro required to build both the native Rust binary (std) and the WebAssembly (no_std) binary.
#![cfg_attr(not(feature = "std"), no_std)]

mod voters;
mod candidates;
// mod admins;

pub use crate::pallet::*;
use scale_info::TypeInfo;


pub struct ElectionConfig<BlockNumber>{
	/// Starting block of the election
	start: BlockNumber,
	/// Length of the election (start + length = end).
	length: BlockNumber,
}

#[frame_support::pallet(dev_mode)]
pub mod pallet {

	// use frame_benchmarking::runtime_decl_for_benchmark::ValueQuery;
	use frame_support::pallet_prelude::*;
	// #[warn(unused_imports)]
	// use frame_support::traits::VoteTally; 
	use frame_system::pallet_prelude::*;

	use crate::voters::{Voter, VoterInfo};
	use crate::candidates::{CandidateInfo,Candidate};

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {

		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type CandidateOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		type VoterOrigin: EnsureOrigin<Self::RuntimeOrigin>; //?  EnsureOrigin... how to do it

		//? Do write types alias for VoterInfo and CandidateInfo
		
	}

	// The pallet's runtime storage items.
	#[pallet::storage]
	#[pallet::getter(fn candidates_list)]
	pub type CandidatesList<T:Config> = StorageMap<_, Blake2_128Concat, u64, CandidateInfo::<T>, ValueQuery >;

	#[pallet::storage]
	#[pallet::getter(fn voters_list)]
	pub type VotersList<T:Config> = StorageMap<_, Blake2_128Concat, u64, VoterInfo::<T>, ValueQuery >; //?



	// Pallets use events to inform users when important changes are made.

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		
		SuccessfulVote { who: T::AccountId },

		CandidateRegistered {who: CandidateInfo::<T>},
		VoterRegistered {who: VoterInfo::<T>},

		GiveVote {from: VoterInfo::<T>, whom: CandidateInfo::<T>},
		FetchVotes {whose: CandidateInfo::<T>, count: u32},
		CandidateInfo {whose:CandidateInfo::<T>},

		ElectionStarted,
		ElectionClosed,

		Winner{who:CandidateInfo::<T>, vote_count: u32}

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		
		NoneValue,
		/// Election has not been configured
		NotConfigured,
		// Election is already in progress
		InProgress,
		// Voter has given his 1 vote 
		AlreadyVoted,
		// The call is not valid for ongoing election
		InvalidCall,
		// Voter or Candidate already registered
		AlreadyRegistered,
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
		pub fn register_candidate(origin: OriginFor<T>, name:String)-> DispatchResult{
			let sender = ensure_signed(origin)?;

			ensure!(sender.registered(), Error::<T>::AlreadyRegistered); //? Registered is a fn on VoterList and CandidateList

			let candidate = Candidate::new(name, sender);
			// CandidateList::<T>::put(candidate);
			
			Self::deposit_event(Event::<T>::CandidateRegistered { who: &candidate });
			Ok(())
		}

		#[pallet::call_index(1)]
		pub fn register_voter(origin: OriginFor<T>)-> DispatchResult{
			let sender = ensure_signed(origin)?;

			ensure!(sender.registered(), Error::<T>::AlreadyRegistered);

			let voter = Voter::new(sender); //? store this voter

			Self::deposit_event(Event::<T>::VoterRegistered { who: &voter });
			Ok(())
		}

		#[pallet::call_index(2)]
		pub fn give_vote(origin: OriginFor<T>, by: VoterInfo::<T>, whose: CandidateInfo::<T>)-> DispatchResult{
			let voter = ensure_signed(origin)?;

			ensure!(by.check_voted(), Error::<T>::AlreadyVoted);

			whose.increase_vote();

			Self::deposit_event(Event::<T>::GiveVote { from: &by, whom: &whose });
			Ok(())
		}

		#[pallet::call_index(3)]
		pub fn vote_count(origin: OriginFor<T>, whose: CandidateInfo::<T>)-> DispatchResult{
			let candidate = ensure_signed(origin)?;
			
			ensure!();

			Self::deposit_event(Event::<T>::FetchVotes { whose: &whose });
			Ok(())
		}

		#[pallet::call_index(4)]
		pub fn candidate_info(origin: OriginFor<T>, whose: CandidateInfo::<T>)-> DispatchResult{
			let candidate = ensure_signed(origin)?;

			Self::deposit_event(Event::<T>::CandidateInfo { whose: &whose });
			Ok(())
		}

		#[pallet::call_index(5)]
		pub fn start_election(origin: OriginFor<T>)-> DispatchResult{
			ensure_root(origin)?;

			Self::deposit_event(Event::<T>::ElectionStarted);
			Ok(())
		}

		#[pallet::call_index(6)]
		pub fn close_election(origin: OriginFor<T>)-> DispatchResult{
			let admin = ensure_root(origin)?;

			Self::deposit_event(Event::<T>::ElectionClosed);
			Ok(())
		}
	}
}
