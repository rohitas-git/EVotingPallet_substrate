use crate::{
	mock::*, AccountToVoterInfo, CandidateInfo, Config, ElectionConfig, ElectionInfo, Error, Event,
	VoterInfo,
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
fn test_election_configured() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(5);

		// Assert that Election was configured
		assert_ok!(TemplateModule::config_election(RuntimeOrigin::root(), 1, 20));
		// Assert that election storage changed
		assert_eq!(TemplateModule::get_election().unwrap(), ElectionInfo::<Test>::voted());
		// Assert that correct event was deposited
		System::assert_last_event(Event::ElectionConfigured.into());
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

use frame_support::ensure;
#[test]
fn s() {
	ExtBuilder::default().build().execute_with(|| {
		setup();
		let is_voter = AccountToVoterInfo::contains_key(ALICE.clone());
		ensure!(is_voter, Error::<Test>::NotRegistered);
		()
	})
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

impl Voted for VoterInfo<Test> {
	fn voted() -> Self {
		let status = true;
		let who = BOB;
		VoterInfo::set(status, who)
	}
}

impl Voted for CandidateInfo {
	fn voted() -> Self {
		CandidateInfo::set(1)
	}
}

impl Voted for ElectionInfo<Test> {
	fn voted() -> Self {
		let start = 1;
		let end = 20;
		ElectionInfo::set(start, end)
	}
}
