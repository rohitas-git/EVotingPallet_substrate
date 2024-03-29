#![cfg_attr(not(feature = "std"), no_std)]

mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

// use frame_support::BoundedVec;
pub use self::pallet::*;

mod weights;
use frame_support::pallet_prelude::Weight;
pub use weights::TemplateWeightInfo;
pub trait WeightInfo {
	fn add_voter() -> Weight;
	fn register_candidate() -> Weight;
	fn config_election() -> Weight;
	fn give_vote() -> Weight;
	fn winner() -> Weight;
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/* --------------------------------- Config --------------------------------- */
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type WeightInfo: WeightInfo;
	}
	/* --------------------------------- Custom --------------------------------- */
	#[derive(Clone, Eq, PartialEq, Encode, Decode, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct VoterInfo<T: Config> {
		vote_status: bool,
		voted_for: Option<T::AccountId>,
	}

	impl<T: Config> VoterInfo<T> {
		pub fn new() -> Self {
			VoterInfo { vote_status: false, voted_for: None }
		}
		pub fn set(val: bool, who: T::AccountId) -> Self {
			VoterInfo { vote_status: val, voted_for: Some(who) }
		}
	}

	#[derive(Clone, Eq, PartialEq, Encode, Decode, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	pub struct CandidateInfo {
		// name: BoundedVec<u8, ConstU32<100>>,
		vote_count: u32,
	}

	impl CandidateInfo {
		pub fn new() -> Self {
			CandidateInfo { vote_count: 0 }
		}
		pub fn set(val: u32) -> Self {
			CandidateInfo { vote_count: val }
		}
	}

	#[derive(Clone, Eq, PartialEq, Encode, Decode, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct ElectionInfo<T: Config> {
		start_block: Option<T::BlockNumber>,
		end_block: Option<T::BlockNumber>,
	}

	impl<T: Config> ElectionInfo<T> {
		fn _new() -> Self {
			ElectionInfo { start_block: None, end_block: None }
		}

		pub fn set(start: T::BlockNumber, end: T::BlockNumber) -> Self {
			ElectionInfo { start_block: Some(start), end_block: Some(end) }
		}

		pub fn ensure_election_progress() -> DispatchResult {
			let block_number = <frame_system::Pallet<T>>::block_number();

			ensure!(ElectionConfig::<T>::get().is_some(), Error::<T>::ElectionNotConfigured);
			let election = ElectionConfig::<T>::get().unwrap();

			ensure!(block_number >= election.start_block.unwrap(), Error::<T>::ElectionNotStarted);
			ensure!(block_number <= election.end_block.unwrap(), Error::<T>::ElectionEnded);

			Ok(())
		}
	}

	/* --------------------------------- storage -------------------------------- */

	#[pallet::storage]
	#[pallet::getter(fn voter_account)]
	pub type AccountToVoterInfo<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, VoterInfo<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn candidate_account)]
	pub type AccountToCandidateInfo<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, CandidateInfo, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_election)]
	pub type ElectionConfig<T: Config> = StorageValue<_, ElectionInfo<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn max_votes_candidate)]
	pub type MaxVoteCandidate<T: Config> =
		StorageValue<_, BoundedVec<T::AccountId, ConstU32<100>>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn max_votes)]
	pub type MaxVote<T: Config> = StorageValue<_, u32, ValueQuery>;

	/* ---------------------------------- Event --------------------------------- */

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// A Voter has registered
		RegisterVoter,
		// A Candidate has registered
		RegisterCandidate,
		// Voter has voted successfully
		VoteSuccess,
		// Get the Current Vote Count of a Candidate
		RecieveVoteCount,
		// Election timings has been configured
		ElectionConfigured,
		// Winner for the election have been stored in a storage
		WinnerVecStored,
	}

	/* ---------------------------------- Error --------------------------------- */
	#[pallet::error]
	pub enum Error<T> {
		// AccountId already registered as Voter
		AlreadyVoted,
		// AccountId already registered as Candidate
		AlreadyRegistered,
		// Election timings are already set
		AlreadyConfiguredElection,
		// AccountId is not registered as Voter or Candidate
		NotRegistered,
		// Election timing is not set
		ElectionNotConfigured,
		// Election has not yet started
		ElectionNotStarted,
		// Election has ended
		ElectionEnded,
		// Election has not ended
		ElectionNotEnded,
		// Candidates exceed the maximum capacity
		MaxCandidatesExceed,
		// When Election start time > end time
		ElectionTimeIllogical,
	}

	/* ---------------------------------- Extrinsics ---------------------------------- */
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::add_voter())]
		// #[pallet::weight(0)]
		pub fn register_voter(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let is_voter = <AccountToVoterInfo<T>>::contains_key(sender.clone());
			ensure!(!is_voter, Error::<T>::AlreadyRegistered);

			let new_voter = VoterInfo::<T>::new();
			<AccountToVoterInfo<T>>::insert(sender, new_voter);

			Self::deposit_event(Event::RegisterVoter);
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::register_candidate())]
		// #[pallet::weight(0)]
		pub fn register_candidate(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let is_candidate = AccountToCandidateInfo::<T>::contains_key(sender.clone());
			ensure!(!is_candidate, Error::<T>::AlreadyRegistered);

			let new_candidate = CandidateInfo::new();
			AccountToCandidateInfo::<T>::insert(sender, new_candidate);

			Self::deposit_event(Event::RegisterCandidate);
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::config_election())]
		// #[pallet::weight(0)]
		pub fn config_election(
			origin: OriginFor<T>,
			start: T::BlockNumber,
			end: T::BlockNumber,
		) -> DispatchResult {
			ensure_root(origin)?;

			let is_configured_election = ElectionConfig::<T>::exists();
			ensure!(!is_configured_election, Error::<T>::AlreadyConfiguredElection);
			ensure!(start < end, Error::<T>::ElectionTimeIllogical);

			let election = ElectionInfo::<T>::set(start, end);

			ElectionConfig::<T>::put(&election);

			Self::deposit_event(Event::ElectionConfigured);
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::give_vote())]
		// #[pallet::weight(0)]
		pub fn give_vote(origin: OriginFor<T>, to_vote_for: T::AccountId) -> DispatchResult {
			let voter_account = ensure_signed(origin)?;

			ElectionInfo::<T>::ensure_election_progress()?;

			let is_voter = <AccountToVoterInfo<T>>::contains_key(voter_account.clone());
			ensure!(is_voter, Error::<T>::NotRegistered);

			let is_candidate = <AccountToCandidateInfo<T>>::contains_key(to_vote_for.clone());
			ensure!(is_candidate, Error::<T>::NotRegistered);

			let voterinfo =
				<AccountToVoterInfo<T>>::get(&voter_account).clone().expect("No VoterInfo");
			ensure!(!voterinfo.vote_status, Error::<T>::AlreadyVoted);

			AccountToCandidateInfo::<T>::mutate(to_vote_for.clone(), |val| {
				val.as_mut().unwrap().vote_count += 1
			});

			AccountToVoterInfo::<T>::mutate(voter_account.clone(), |voter| {
				voter.as_mut().unwrap().vote_status = true
			});

			AccountToVoterInfo::<T>::mutate(voter_account.clone(), |voter| {
				voter.as_mut().unwrap().voted_for = Some(to_vote_for.clone())
			});

			// Max Votes
			let max_votes = MaxVote::<T>::get();
			let candidate_votes =
				AccountToCandidateInfo::<T>::get(&to_vote_for).unwrap().vote_count;
			if max_votes < candidate_votes {
				MaxVote::<T>::put(candidate_votes);
			}

			Self::deposit_event(Event::VoteSuccess);
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::winner())]
		// #[pallet::weight(0)]
		pub fn winner(_origin: OriginFor<T>) -> DispatchResult {
			//Election was configured and has ended
			let block_number = <frame_system::Pallet<T>>::block_number();
			ensure!(ElectionConfig::<T>::get().is_some(), Error::<T>::ElectionNotConfigured);
			let election = ElectionConfig::<T>::get().unwrap();
			ensure!(block_number >= election.end_block.unwrap(), Error::<T>::ElectionNotEnded);

			// Candidates
			let mut winner_num = 0;
			let mut winner_vec: BoundedVec<T::AccountId, ConstU32<100>> = Default::default();
			for key in AccountToCandidateInfo::<T>::iter_keys() {
				let candidate_info = AccountToCandidateInfo::<T>::get(key.clone()).unwrap();
				let votes = candidate_info.clone().vote_count;
				let max_votes = MaxVote::<T>::get();

				if votes == max_votes {
					ensure!(winner_num <= 100, Error::<T>::MaxCandidatesExceed);
					winner_num += 1;
					winner_vec.try_push(key.clone()).unwrap();
				}
			}
			// println!("Winner Vec: {:?}", &winner_vec);
			MaxVoteCandidate::<T>::put(winner_vec);
			Self::deposit_event(Event::WinnerVecStored);
			Ok(())
		}
	}
}
