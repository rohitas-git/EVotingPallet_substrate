// Benchmarking for `pallet-evoting`.

#![cfg(feature = "runtime-benchmarks")]

use crate::*;
use frame_benchmarking::v2::*;
// use frame_support::{assert_eq, assert_last_event, assert_ok};
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	// fn assert_last_event<T: Config<I>, I: 'static>(generic_event: <T as Config<I>>::RuntimeEvent)
	// { 	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
	// }

	// This will measure the execution time of `register_voter`.
	#[benchmark]
	fn add_voter() {
		// This is the benchmark setup phase.

		// The caller account is whitelisted for DB reads/write by the benchmarking macro.
		let caller: T::AccountId = whitelisted_caller();
		let new_voter: pallet::VoterInfo<T> = VoterInfo::new();

		// Execution phase
		#[extrinsic_call]
		Pallet::<T>::register_voter(RawOrigin::Signed(caller));

		// Optional Verification Phase
		// self::assert_ok!(register_voter(RawOrigin::Signed(caller)));
		// self::assert_eq!(Pallet::<T>::voter_account(caller).unwrap(), new_voter);
		// assert_last_event(Event::RegisterVoter.into());
	}

	#[benchmark]
	fn register_candidate() {
		let caller: T::AccountId = whitelisted_caller();
		let new_candidate = CandidateInfo::new();

		// You can use `_` if the name of the Call matches the benchmark name.
		#[extrinsic_call]
		_(RawOrigin::Signed(caller));

		// self::assert_ok!(register_candidate(RawOrigin::Signed(caller)));
		// self::assert_eq!(Pallet::<T>::candidate_account(caller).unwrap(), new_candidate);
		// assert_last_event(Event::RegisterCandidate.into());
	}

	// This line generates test cases for benchmarking, and could be run by:
	//   `cargo test -p pallet-example-basic --all-features`, you will see one line per case:
	//
	// The line generates three steps per benchmark, with repeat=1 and the three steps are
	//   [low, mid, high] of the range.
	impl_benchmark_test_suite!(Pallet, crate::tests::new_test_ext(), crate::tests::Test);
}
