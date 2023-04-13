// Use empirical measurements of the runtime to determine the time it takes
// to execute the extrinsics and other runtime logic.

// Run benchmarks using worst case scenario conditions
// Primary goal is to keep the runtime safe
// Secondary goal is to be accurate as possible to maxime the throughput

use super::*;

#[allow(unused)]
use frame_benchmarking::v1::{benchmarks, whitelisted_caller};
use frame_support::{assert_eq, assert_last_event, assert_ok};
use frame_system::RawOrigin;

benchmarks! {
	register_voter{
		let c in 1..100;
		let caller: T::AccountId = whitelisted_caller();

	}: _(RawOrigin::Signed(caller))
	verify{
		assert_ok!(register_voter(who(ALICE)));
		assert_eq!(TemplateModule::voter_account(ALICE).unwrap(), VoterInfo { vote_status: false, voted_for: None });
		System::assert_last_event(Event::RegisterVoter.into());
	}

	register_candidate{
		let c in 1..100;
		let caller: T::AccountId = whitelisted_caller();

	}: _(RawOrigin::Signed(caller))
	verify{
		assert_ok!(register_candidate(who(BOB)));
		assert_eq!(TemplateModule::candidate_account(BOB).unwrap(), CandidateInfo { vote_count: 0 });
		System::assert_last_event(Event::RegisterCandidate.into());
	}

	config_election {
		let start in 0..10;
		let end in start..100;

		let caller: T::AccountId = whitelisted_caller();

	}: _(RawOrigin::Signed(caller), start, end)
	verify {

	}

	giving_vote{
		let c in 1..100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))
	verify{
		
	}

	winner{
		let
	}

	// impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}

pub trait WeightInfo {
    fn add_voter() -> Weight;
    fn register_candidate() -> Weight;
	fn config_election()-> Weight;
	fn give_vote()-> Weight;
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tests::*;
	// use crate::mock::*;
	use frame_support::assert_ok;

	#[test]
	fn config_election() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_configuring_election);
		});
	}
}
