use crate::{
	mock::*, CandidateInfo, Config, ElectionConfig, ElectionInfo, Error, Event, VoterInfo,
};
use frame_support::{assert_noop, assert_ok};
// use crate as pallet_template;

#[test]
fn test_during_election() {
	new_test_ext().execute_with(|| {
		test_voting_process();
	})
}

#[test]
fn test_before_election() {
	new_test_ext().execute_with(|| {
		setup();

		type T = Test;
		assert_noop!(
			TemplateModule::give_vote(RuntimeOrigin::signed(ALICE), BOB),
			Error::<T>::ElectionNotStarted
		);
	})
}

#[test]
fn test_after_election() {
	new_test_ext().execute_with(|| {
		setup();

		type T = Test;
		assert_noop!(
			TemplateModule::give_vote(RuntimeOrigin::signed(ALICE), BOB),
			Error::<T>::ElectionEnded
		);
	})
}

#[test]
fn test_register_voter() {
	ExtBuilder::default().build().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Register Voter
		assert_ok!(TemplateModule::register_voter(RuntimeOrigin::signed(ALICE)));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::voter_account(ALICE).unwrap(), VoterInfo::new());
		// Assert that the correct event was deposited
		System::assert_last_event(Event::RegisterVoter.into());
	})
}

#[test]
fn test_register_candidate() {
	ExtBuilder::default().build().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// Register Voter
		assert_ok!(TemplateModule::register_candidate(RuntimeOrigin::signed(BOB)));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::candidate_account(BOB).unwrap(), CandidateInfo::new());
		// Assert that the correct event was deposited
		System::assert_last_event(Event::RegisterCandidate.into());
	})
}

#[test]
fn test_voting_process() {
	ExtBuilder::default().build().execute_with(|| {
		setup();

		// Assert that ALICE gives vote to BOB is OK
		assert_ok!(TemplateModule::give_vote(RuntimeOrigin::signed(ALICE), BOB));
		// Assert that BOB's CandidateInfo has changed
		assert_eq!(TemplateModule::candidate_account(BOB).unwrap(), CandidateInfo::voted());
		// Assert that ALICE's VoterInfo has changed
		assert_eq!(TemplateModule::voter_account(ALICE).unwrap(), VoterInfo::voted());
		// Assert that correct event was deposited
		System::assert_last_event(Event::VoteSuccess.into());
	})
}

#[test]
fn test_election_configured() {
	System::set_block_number(5);
	// Assert that Election was configured
	assert_ok!(TemplateModule::config_election(RuntimeOrigin::signed(ALICE), 1, 20));
	// Assert that election storage changed
	assert_eq!(TemplateModule::get_election().unwrap(), ElectionInfo::voted());
	// Assert that correct event was deposited
	System::assert_last_event(Event::ElectionConfigured.into());
}

fn setup() {
	test_register_voter();
	test_register_candidate();
	test_election_configured();
}

trait Voted {
	fn voted() -> Self;

	fn ongoing() {}
}

impl<T: Config> Voted for VoterInfo<T> {
	fn voted() -> Self {
		let status = true;
		let who = Some(BOB);
		VoterInfo { vote_status: status, voted_for: who }
	}
}

impl Voted for CandidateInfo {
	fn voted() -> Self {
		CandidateInfo { vote_count: 1 }
	}
}

impl<T: Config> Voted for ElectionInfo<T> {
	fn voted() -> Self {
		let start = 1u32.into();
		let end = 20.into();
		ElectionInfo::<T> { start_block: Some(start), end_block: Some(end) }
	}
}
