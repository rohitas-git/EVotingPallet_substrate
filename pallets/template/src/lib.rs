#![cfg_attr(not(feature = "std"), no_std)]

// use frame_support::BoundedVec;
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	// use sp_core::ConstU32;
	// type MyCandidateList<T> = BoundedVec<CandidateInfo, <T as Config>::MaxCandidates>;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		// #[pallet::constant]
		// type MaxCandidates: Get<u32>;
	}

	#[derive(Clone, Eq, PartialEq, Encode, Decode, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct VoterInfo<T: Config> {
		vote_status: bool,
		voted_for: Option<T::AccountId>,
	}

	impl<T: Config> VoterInfo<T> {
		fn new() -> Self {
			VoterInfo { vote_status: false, voted_for: None }
		}
	}

	#[derive(Clone, Eq, PartialEq, Encode, Decode, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	pub struct CandidateInfo {
		// name: BoundedVec<u8, ConstU32<100>>,
		vote_count: u32,
	}

	impl CandidateInfo {
		fn new() -> Self {
			CandidateInfo { vote_count: 0 }
		}
	}

	#[derive(Clone, Eq, PartialEq, Encode, Decode, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct ElectionInfo<T: Config> {
		start_block: Option<T::BlockNumber>,
		end_block: Option<T::BlockNumber>,
	}

	impl<T: Config> ElectionInfo<T> {
		fn new() -> Self {
			ElectionInfo { start_block: None, end_block: None }
		}

		fn set_start(&mut self, num: T::BlockNumber) {
			self.start_block = Some(num);
		}

		fn set_end(&mut self, num: T::BlockNumber) {
			self.end_block = Some(num);
		}

		fn ensure_election_progress() -> DispatchResult {
			let block_number = <frame_system::Pallet<T>>::block_number();
			let election = ElectionConfig::<T>::get().unwrap();

			ensure!(block_number >= election.start_block.unwrap(), Error::<T>::ElectionNotStarted);
			ensure!(block_number <= election.end_block.unwrap(), Error::<T>::ElectionEnded);

			Ok(())
		}
	}

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
	pub type ElectionConfig<T: Config> = StorageValue<_, ElectionInfo<T>>;

	// #[pallet::storage]
	// #[pallet::getter(fn candidate_list)]
	// pub type CandidateList<T: Config> = StorageValue<_, MyCandidateList<T>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		RegisterVoter,
		RegisterCandidate,
		VoteSuccess,
		RecieveVoteCount,
		ElectionConfigured,
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		AlreadyVoted,
		AlreadyRegistered,
		AlreadyConfiguredElection,
		NotRegistered,
		ElectionNotConfigured,
		ElectionNotStarted,
		ElectionEnded,
		ElectionNotEnded,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		#[pallet::weight(0)]
		pub fn register_voter(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let is_voter = <AccountToVoterInfo<T>>::contains_key(sender.clone());
			ensure!(!is_voter, Error::<T>::AlreadyRegistered);

			let new_voter = VoterInfo::<T>::new();
			<AccountToVoterInfo<T>>::insert(sender, new_voter);

			Self::deposit_event(Event::RegisterVoter);
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		// #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		#[pallet::weight(0)]
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
		#[pallet::weight(0)]
		pub fn give_vote(origin: OriginFor<T>, to_vote_for: T::AccountId) -> DispatchResult {
			let voter = ensure_signed(origin)?;

			ElectionInfo::<T>::ensure_election_progress()?;

			let is_voter = <AccountToVoterInfo<T>>::contains_key(voter.clone());
			ensure!(is_voter, Error::<T>::NotRegistered);

			let voterinfo = <AccountToVoterInfo<T>>::get(&voter).clone().expect("No VoterInfo");
			ensure!(!voterinfo.vote_status, Error::<T>::AlreadyVoted);

			AccountToCandidateInfo::<T>::mutate(to_vote_for.clone(), |val| {
				val.as_mut().unwrap().vote_count += 1
			});

			AccountToVoterInfo::<T>::mutate(voter.clone(), |voter| {
				voter.as_mut().unwrap().vote_status = true
			});

			AccountToVoterInfo::<T>::mutate(voter.clone(), |voter| {
				voter.as_mut().unwrap().voted_for = Some(to_vote_for)
			});

			Self::deposit_event(Event::VoteSuccess);
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn config_election(
			origin: OriginFor<T>,
			start: T::BlockNumber,
			end: T::BlockNumber,
		) -> DispatchResult {
			ensure_root(origin)?;

			let is_configured_election = ElectionConfig::<T>::exists();
			ensure!(!is_configured_election, Error::<T>::AlreadyConfiguredElection);

			let mut election = ElectionInfo::<T>::new();
			election.set_start(start);
			election.set_end(end);

			ElectionConfig::<T>::put(election);

			Self::deposit_event(Event::ElectionConfigured);
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(0)]
		pub fn winner(_origin: OriginFor<T>) -> DispatchResult {
			let block_number = <frame_system::Pallet<T>>::block_number();
			let election = ElectionConfig::<T>::get().unwrap();

			ensure!(block_number >= election.end_block.unwrap(), Error::<T>::ElectionNotEnded);
			// todo: INCOMPLETE
			Ok(())
		}

	}

	// fn push_candidate<T>(item: T::AccountId)
	// where
	// 	T: frame_system::Config,
	// {
	// 	CandidateList::get().mutate()
	// }
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate as pallet_template;

	use frame_support::{
		assert_noop, assert_ok, ord_parameter_types,
		traits::{ConstU32, ConstU64},
	};
	use frame_system::EnsureSignedBy;
	use sp_core::H256;
	use sp_runtime::{
		testing::Header,
		traits::{BadOrigin, BlakeTwo256, IdentityLookup},
	};

	type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
	type Block = frame_system::mocking::MockBlock<Test>;

	frame_support::construct_runtime!(
		pub struct Test
		where
			Block = Block,
			NodeBlock = Block,
			UncheckedExtrinsic = UncheckedExtrinsic,
		{
			System: frame_system,

			Evoting: pallet_template,
		}
	);

	impl frame_system::Config for Test {
		type BaseCallFilter = frame_support::traits::Everything;
		type BlockWeights = ();
		type BlockLength = ();
		type DbWeight = ();
		type RuntimeOrigin = RuntimeOrigin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type RuntimeCall = RuntimeCall;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type RuntimeEvent = RuntimeEvent;
		type BlockHashCount = ConstU64<250>;
		type Version = ();
		type PalletInfo = PalletInfo;
		type AccountData = pallet_balances::AccountData<u64>;
		type OnNewAccount = ();
		type OnKilledAccount = ();
		type SystemWeightInfo = ();
		type SS58Prefix = ();
		type OnSetCode = ();
		type MaxConsumers = ConstU32<16>;
	}

	impl pallet_balances::Config for Test {
		type MaxLocks = ();
		type MaxReserves = ();
		type ReserveIdentifier = [u8; 8];
		type Balance = u64;
		type RuntimeEvent = RuntimeEvent;
		type DustRemoval = ();
		type ExistentialDeposit = ConstU64<1>;
		type AccountStore = System;
		type WeightInfo = ();
		type FreezeIdentifier = ();
		type MaxFreezes = ();
		type HoldIdentifier = ();
		type MaxHolds = ();
	}

	ord_parameter_types! {
		pub const One: u64 = 1;
	}
	impl Config for Test {
		type RuntimeEvent = RuntimeEvent;
		type Currency = Balances;
		type ReservationFee = ConstU64<2>;
		type Slashed = ();
		type ForceOrigin = EnsureSignedBy<One, u64>;
		type MinLength = ConstU32<3>;
		type MaxLength = ConstU32<16>;
	}

	fn new_test_ext() -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		pallet_balances::GenesisConfig::<Test> { balances: vec![(1, 10), (2, 10)] }
			.assimilate_storage(&mut t)
			.unwrap();
		t.into()
	}

	#[test]
	fn kill_name_should_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Nicks::set_name(RuntimeOrigin::signed(2), b"Dave".to_vec()));
			assert_eq!(Balances::total_balance(&2), 10);
			assert_ok!(Nicks::kill_name(RuntimeOrigin::signed(1), 2));
			assert_eq!(Balances::total_balance(&2), 8);
			assert_eq!(<NameOf<Test>>::get(2), None);
		});
	}

	#[test]
	fn force_name_should_work() {
		new_test_ext().execute_with(|| {
			assert_noop!(
				Nicks::set_name(RuntimeOrigin::signed(2), b"Dr. David Brubeck, III".to_vec()),
				Error::<Test>::TooLong,
			);

			assert_ok!(Nicks::set_name(RuntimeOrigin::signed(2), b"Dave".to_vec()));
			assert_eq!(Balances::reserved_balance(2), 2);
			assert_noop!(
				Nicks::force_name(RuntimeOrigin::signed(1), 2, b"Dr. David Brubeck, III".to_vec()),
				Error::<Test>::TooLong,
			);
			assert_ok!(Nicks::force_name(
				RuntimeOrigin::signed(1),
				2,
				b"Dr. Brubeck, III".to_vec()
			));
			assert_eq!(Balances::reserved_balance(2), 2);
			let (name, amount) = <NameOf<Test>>::get(2).unwrap();
			assert_eq!(name, b"Dr. Brubeck, III".to_vec());
			assert_eq!(amount, 2);
		});
	}

	#[test]
	fn normal_operation_should_work() {
		new_test_ext().execute_with(|| {
			assert_ok!(Nicks::set_name(RuntimeOrigin::signed(1), b"Gav".to_vec()));
			assert_eq!(Balances::reserved_balance(1), 2);
			assert_eq!(Balances::free_balance(1), 8);
			assert_eq!(<NameOf<Test>>::get(1).unwrap().0, b"Gav".to_vec());

			assert_ok!(Nicks::set_name(RuntimeOrigin::signed(1), b"Gavin".to_vec()));
			assert_eq!(Balances::reserved_balance(1), 2);
			assert_eq!(Balances::free_balance(1), 8);
			assert_eq!(<NameOf<Test>>::get(1).unwrap().0, b"Gavin".to_vec());

			assert_ok!(Nicks::clear_name(RuntimeOrigin::signed(1)));
			assert_eq!(Balances::reserved_balance(1), 0);
			assert_eq!(Balances::free_balance(1), 10);
		});
	}

	#[test]
	fn error_catching_should_work() {
		new_test_ext().execute_with(|| {
			assert_noop!(Nicks::clear_name(RuntimeOrigin::signed(1)), Error::<Test>::Unnamed);

			assert_noop!(
				Nicks::set_name(RuntimeOrigin::signed(3), b"Dave".to_vec()),
				pallet_balances::Error::<Test, _>::InsufficientBalance
			);

			assert_noop!(
				Nicks::set_name(RuntimeOrigin::signed(1), b"Ga".to_vec()),
				Error::<Test>::TooShort
			);
			assert_noop!(
				Nicks::set_name(RuntimeOrigin::signed(1), b"Gavin James Wood, Esquire".to_vec()),
				Error::<Test>::TooLong
			);
			assert_ok!(Nicks::set_name(RuntimeOrigin::signed(1), b"Dave".to_vec()));
			assert_noop!(Nicks::kill_name(RuntimeOrigin::signed(2), 1), BadOrigin);
			assert_noop!(
				Nicks::force_name(RuntimeOrigin::signed(2), 1, b"Whatever".to_vec()),
				BadOrigin
			);
		});
	}
}
