// Benchmarking for `pallet-evoting`.

#![cfg(feature = "runtime-benchmarks")]

/// `use crate::*;` is a wildcard import that imports all items from the current crate's root
/// module into the current module. This allows the code to use any item defined in the crate's
/// root module without having to specify the module path. In this case, it is used in the
/// benchmarking module to import all items from the crate's root module so that they can be
/// used in the benchmarking functions without having to specify the module path for each item.
use crate::*;
use frame_benchmarking::v2::*;
use frame_support::assert_eq;
use frame_system::RawOrigin;

#[benchmarks(where <T as frame_system::Config>::RuntimeOrigin: From<T>,  T: core::convert::From<frame_system::RawOrigin<<T as frame_system::Config>::AccountId>>)]
mod benchmarks {
	use super::*;

	fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
		frame_system::Pallet::<T>::assert_last_event(generic_event.into());
	}

	// This will measure the execution time of `register_voter`.
	#[benchmark]
	fn add_voter() {
		// This is the benchmark setup phase.

		// The caller account is whitelisted for DB reads/write by the benchmarking macro.
		// let caller: T::AccountId = whitelisted_caller();

		let voter: T::AccountId = account("Alice", 1u32, 2u32);
		let new_voter: pallet::VoterInfo<T> = VoterInfo::new();
		let voter_origin =
			<frame_system::RawOrigin<<T as frame_system::Config>::AccountId> as Into<T>>::into(
				RawOrigin::Signed(voter),
			);

		// Execution phase
		#[extrinsic_call]
		Pallet::<T>::register_voter(voter_origin);

		// Optional Verification Phase
		self::assert_eq!(Pallet::<T>::voter_account(voter).unwrap(), new_voter);
		assert_last_event(Event::RegisterVoter.into());
	}

	#[benchmark]
	fn register_candidate() {
		let candidate: T::AccountId = account("Bob", 1u32, 2u32);
		let new_candidate = CandidateInfo::new();
		let candidate_origin =
			<frame_system::RawOrigin<<T as frame_system::Config>::AccountId> as Into<T>>::into(
				RawOrigin::Signed(candidate),
			);

		#[extrinsic_call]
		_(RawOrigin::Signed(candidate_origin));

		self::assert_eq!(Pallet::<T>::candidate_account(candidate).unwrap(), new_candidate);
		assert_last_event(Event::RegisterCandidate.into());
	}

	#[benchmark]
	fn config_election() {
		let (start, end): (u32, u32) = (2, 10);
		let new_election_info = ElectionInfo::set(start.into(), end.into());
		let root_origin= <frame_system::RawOrigin<AccountId> as Into<T>>::into(RawOrigin::Root);

		#[extrinsic_call]
		_(root_origin, start.into(), end.into());

		self::assert_eq!(Pallet::<T>::get_election().unwrap(), new_election_info);
		assert_last_event(Event::ElectionConfigured.into());
	}

	#[benchmark]
	fn give_vote() {
		let voter: T::AccountId = account("Alice", 1u32, 2u32);
		let candidate: T::AccountId = account("Bob", 1u32, 2u32);
		let (start, end): (u32, u32) = (2, 10);
		let new_election_info = ElectionInfo::set(start.into(), end.into());
		let voted_voter = VoterInfo::set(true, candidate);
		let voted_candidate = CandidateInfo::set(1);

		Pallet::<T>::register_voter(RawOrigin::Signed(voter).into());
		Pallet::<T>::register_candidate(RawOrigin::Signed(candidate).into());
		Pallet::<T>::config_election(RawOrigin::Root.into(), start.into(), end.into());
		frame_system::set_block_number(5);
		// <T as frame_system::Config>::BlockNumber
		#[block]
		{
			Pallet::<T>::give_vote(RawOrigin::Signed(voter).into(), candidate);
		}

		self::assert_eq!(Pallet::<T>::candidate_account(candidate).unwrap(), voted_candidate);
		self::assert_eq!(Pallet::<T>::voter_account(voter).unwrap(), voted_voter);
		assert_last_event(Event::VoteSuccess.into());
	}
}

#[cfg(test)]
mod tests {
	use crate::mock::Test;
	use sp_io::TestExternalities;

	pub fn new_test_ext() -> TestExternalities {
		let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		TestExternalities::new(t)
	}
}

// This line generates test cases for benchmarking, and could be run by:
//   `cargo test -p pallet-example-basic --all-features`, you will see one line per case:
//
// The line generates three steps per benchmark, with repeat=1 and the three steps are
//   [low, mid, high] of the range.
impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
