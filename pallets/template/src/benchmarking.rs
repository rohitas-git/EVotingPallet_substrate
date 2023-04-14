// Benchmarking for `pallet-evoting`.

#![cfg(feature = "runtime-benchmarks")]

/// `use crate::*;` is a wildcard import that imports all items from the current crate's root
/// module into the current module. This allows the code to use any item defined in the crate's
/// root module without having to specify the module path. In this case, it is used in the
/// benchmarking module to import all items from the crate's root module so that they can be
/// used in the benchmarking functions without having to specify the module path for each item.
use crate::*;
use frame_benchmarking::v2::*;
// use frame_support::assert_eq;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
		frame_system::Pallet::<T>::assert_last_event(generic_event.into());
	}

	// This will measure the execution time of `register_voter`.
	#[benchmark]
	fn add_voter() {
		let voter: T::AccountId = account("Alice", 1u32, 2u32);
		let voter_origin = RawOrigin::Signed(voter.clone());

		#[extrinsic_call]
		Pallet::<T>::register_voter(voter_origin.clone());

		assert_last_event::<T>(Event::RegisterVoter.into());
	}

	#[benchmark]
	fn register_candidate() {
		let candidate: T::AccountId = account("Bob", 1u32, 2u32);
		let candidate_origin = RawOrigin::Signed(candidate.clone());

		#[extrinsic_call]
		_(candidate_origin.clone());

		assert_last_event::<T>(Event::RegisterCandidate.into());
	}

	#[benchmark]
	fn config_election() {
		let (start, end): (u32, u32) = (2, 10);
		let root_origin = RawOrigin::Root;

		#[extrinsic_call]
		_(root_origin, start.into(), end.into());

		assert_last_event::<T>(Event::ElectionConfigured.into());
	}

	#[benchmark]
	fn give_vote() {
		let voter: T::AccountId = account("Alice", 1u32, 2u32);
		let candidate: T::AccountId = account("Bob", 2u32, 3u32);
		let (start, end): (u32, u32) = (2, 10);

		let voter_origin: <T as frame_system::Config>::RuntimeOrigin =
			RawOrigin::Signed(voter.clone()).into();
		let candidate_origin: <T as frame_system::Config>::RuntimeOrigin =
			RawOrigin::Signed(candidate.clone()).into();
		let root_origin: <T as frame_system::Config>::RuntimeOrigin = RawOrigin::Root.into();


		Pallet::<T>::register_voter(voter_origin.clone());
		Pallet::<T>::register_candidate(candidate_origin.clone());
		Pallet::<T>::config_election(root_origin.clone(), start.into(), end.into());

		frame_system::Pallet::<T>::set_block_number(start.into());

		#[block]
		{	
			#[allow(unused_must_use)]
			Pallet::<T>::give_vote(voter_origin.clone(), candidate.clone());
		}

		// self::assert_eq!(Pallet::<T>::candidate_account(candidate).unwrap(), voted_candidate);
		// self::assert_eq!(Pallet::<T>::voter_account(voter).unwrap(), voted_voter);
		assert_last_event::<T>(Event::VoteSuccess.into());
	}

	#[benchmark]
	fn winner() {
		let root_origin: <T as frame_system::Config>::RuntimeOrigin = RawOrigin::Root.into();
		let (start, end, inbetween, after_end): (u32, u32, u32, u32) = (2, 10, 7, 15);

		Pallet::<T>::config_election(root_origin.clone(), start.into(), end.into());
		frame_system::Pallet::<T>::set_block_number(start.into());

		let num_candidates =100;
		let num_voter=1000;

		let mut candidates_list: Vec<T::AccountId > = vec![];
		for i in 0..num_candidates{
			let candidate: T::AccountId = account("Candidate", 1u32, i as u32);
			let candidate_origin: <T as frame_system::Config>::RuntimeOrigin= RawOrigin::Signed(candidate.clone()).into();

			candidates_list.push(candidate.clone());
			Pallet::<T>::register_candidate(candidate_origin.clone());
		}

		let mut voter_origin_list: Vec< <T as frame_system::Config>::RuntimeOrigin > = vec![];
		for i in 0..num_voter{
			let voter: T::AccountId = account("Voter", 1u32, i as u32);
			let voter_origin: <T as frame_system::Config>::RuntimeOrigin= RawOrigin::Signed(voter.clone()).into();

			voter_origin_list.push(voter_origin.clone());
			Pallet::<T>::register_voter(voter_origin.clone());
		}

		frame_system::Pallet::<T>::set_block_number(inbetween.into());

		let outer_id= voter_origin_list.len() / candidates_list.len(); // 10
		let inner_id: usize = 100;
		for i in 1..=outer_id{
			let start= inner_id*(i-1);
			let end= inner_id*(i); 

			for voter_index in start..end{
				let candidate_index= voter_index - start;
				let voter_origin = voter_origin_list.get(voter_index).unwrap().clone();
				let candidate = candidates_list.get(candidate_index).unwrap().clone();
				Pallet::<T>::give_vote(voter_origin, candidate );
			}
		}

		frame_system::Pallet::<T>::set_block_number(after_end.into());
		let caller: T::AccountId = account("Alice", 1u32, 2u32);
		let caller: <T as frame_system::Config>::RuntimeOrigin= RawOrigin::Signed(caller.clone()).into();
		
		#[block]
		{	
			Pallet::<T>::winner(caller.clone());
		}

		assert_last_event::<T>(Event::WinnerVecStored.into());
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
