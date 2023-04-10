#![allow(unused_imports)]
use crate::{
	mock::*, AccountToVoterInfo, CandidateInfo, Config, ElectionConfig, ElectionInfo, Error, Event,
	VoterInfo,
};
use frame_support::{assert_noop, assert_ok};
// use crate as pallet_template;

// !Funda: give when then
// ! Wrap it with fn set_election_time
// ! Remove code duplication
// ! Better Naming (Descriptive names also work)
// ! Split code into meaningful files
// ! Replace number with meaningful const/variable

#[test]
fn test_during_election() {
	new_test_ext().execute_with(|| {
		test_voting_process();
	})
}

#[test]
fn test_voting_process() {
	ExtBuilder::default().build().execute_with(|| {
		// Voter
		assert_ok!(TemplateModule::register_voter(RuntimeOrigin::signed(ALICE)));
		assert_eq!(TemplateModule::voter_account(ALICE).unwrap(), VoterInfo::new());

		// Candidate
		assert_ok!(TemplateModule::register_candidate(RuntimeOrigin::signed(BOB)));
		assert_eq!(TemplateModule::candidate_account(BOB).unwrap(), CandidateInfo::new());

		// Election
		assert_ok!(TemplateModule::config_election(RuntimeOrigin::root(), 1, 20));
		assert_eq!(TemplateModule::get_election().unwrap(), ElectionInfo::<Test>::voted());

		// BlockNumber
		System::set_block_number(5);

		// -------------------------------- Voting -------------------------------

		// Assert that ALICE gives vote to BOB is OK
		assert_ok!(TemplateModule::give_vote(RuntimeOrigin::signed(ALICE), BOB));
		// Assert that BOB's CandidateInfo has changed
		assert_eq!(TemplateModule::candidate_account(BOB).unwrap(), CandidateInfo::voted());
		// Assert that ALICE's VoterInfo
		assert_eq!(TemplateModule::voter_account(ALICE).unwrap(), VoterInfo::voted());
		// Assert that correct event was deposited
		System::assert_last_event(Event::VoteSuccess.into());
	})
}

#[test]
fn err_before_election() {
	new_test_ext().execute_with(|| {
		// Voter
		assert_ok!(TemplateModule::register_voter(RuntimeOrigin::signed(ALICE)));
		assert_eq!(TemplateModule::voter_account(ALICE).unwrap(), VoterInfo::new());

		// Candidate
		assert_ok!(TemplateModule::register_candidate(RuntimeOrigin::signed(BOB)));
		assert_eq!(TemplateModule::candidate_account(BOB).unwrap(), CandidateInfo::new());

		// Election
		assert_ok!(TemplateModule::config_election(RuntimeOrigin::root(), 1, 20));
		assert_eq!(TemplateModule::get_election().unwrap(), ElectionInfo::<Test>::voted());

		// BlockNumber
		System::set_block_number(0);

		// Voting
		assert_noop!(
			TemplateModule::give_vote(RuntimeOrigin::signed(ALICE), BOB),
			Error::<Test>::ElectionNotStarted
		);
	})
}

#[test]
fn err_after_election() {
	new_test_ext().execute_with(|| {
		// Voter
		assert_ok!(TemplateModule::register_voter(RuntimeOrigin::signed(ALICE)));
		assert_eq!(TemplateModule::voter_account(ALICE).unwrap(), VoterInfo::new());

		// Candidate
		assert_ok!(TemplateModule::register_candidate(RuntimeOrigin::signed(BOB)));
		assert_eq!(TemplateModule::candidate_account(BOB).unwrap(), CandidateInfo::new());

		// Election
		assert_ok!(TemplateModule::config_election(RuntimeOrigin::root(), 1, 20));
		assert_eq!(TemplateModule::get_election().unwrap(), ElectionInfo::<Test>::voted());

		// BlockNumber
		System::set_block_number(25);

		// Voting
		assert_noop!(
			TemplateModule::give_vote(RuntimeOrigin::signed(ALICE), BOB),
			Error::<Test>::ElectionEnded
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
fn test_winner() {
	ExtBuilder::default().build().execute_with(|| {
		// Voter
		assert_ok!(TemplateModule::register_voter(RuntimeOrigin::signed(ALICE)));
		assert_ok!(TemplateModule::register_voter(RuntimeOrigin::signed(BOB)));
		assert_ok!(TemplateModule::register_voter(RuntimeOrigin::signed(DAVE)));
		assert_ok!(TemplateModule::register_voter(RuntimeOrigin::signed(JOHN)));
		assert_ok!(TemplateModule::register_voter(RuntimeOrigin::signed(RON)));
		
		// Candidate
		assert_ok!(TemplateModule::register_candidate(RuntimeOrigin::signed(DAVE)));
		assert_ok!(TemplateModule::register_candidate(RuntimeOrigin::signed(RON)));
		assert_ok!(TemplateModule::register_candidate(RuntimeOrigin::signed(JOHN)));
		

		// Election
		assert_ok!(TemplateModule::config_election(RuntimeOrigin::root(), 1, 20));

		// BlockNumber
		System::set_block_number(5);

		// -------------------------------- Voting -------------------------------

		// Assert that votes are given correctly
		assert_ok!(TemplateModule::give_vote(RuntimeOrigin::signed(ALICE), DAVE));
		assert_ok!(TemplateModule::give_vote(RuntimeOrigin::signed(JOHN), DAVE));
		assert_ok!(TemplateModule::give_vote(RuntimeOrigin::signed(DAVE), RON));
		assert_ok!(TemplateModule::give_vote(RuntimeOrigin::signed(BOB), RON));
		assert_ok!(TemplateModule::give_vote(RuntimeOrigin::signed(RON), JOHN));
		
		/* --------------------------------- Winner --------------------------------- */
		// BlockNumber
		System::set_block_number(25);
		
		// println!("Max Votes: {}",TemplateModule::max_votes());
		// println!("Winner Vec: {:?}", TemplateModule::max_votes_candidate().clone().unwrap_or_default());

		use frame_support::BoundedVec;
		use frame_support::pallet_prelude::ConstU32;
		let win: BoundedVec<<Test as frame_system::Config>::AccountId, ConstU32<100>> = vec![DAVE,RON].try_into().unwrap();

		// Assert that winner was called
		assert_ok!(TemplateModule::winner(RuntimeOrigin::signed(ALICE)));
		// Assert that storage was correctly modified
		assert_eq!(TemplateModule::max_votes_candidate().unwrap(), win);
		// Assert that correct event was deposited
		System::assert_last_event(Event::WinnerVecStored.into());
	})
}

#[test]
fn err_twice_register_voter() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);
		// Register Voter
		assert_ok!(TemplateModule::register_voter(RuntimeOrigin::signed(ALICE)));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::voter_account(ALICE).unwrap(), VoterInfo::new());
		// Assert that the correct event was deposited
		System::assert_last_event(Event::RegisterVoter.into());

		// Register Voter Again
		assert_noop!(
			TemplateModule::register_voter(RuntimeOrigin::signed(ALICE)),
			Error::<Test>::AlreadyRegistered
		);
	})
}

#[test]
fn err_twice_register_candidate() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);
		// Register Candidate
		assert_ok!(TemplateModule::register_candidate(RuntimeOrigin::signed(BOB)));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::candidate_account(BOB).unwrap(), CandidateInfo::new());
		// Assert that the correct event was deposited
		System::assert_last_event(Event::RegisterCandidate.into());

		// Register Candidate Again
		assert_noop!(
			TemplateModule::register_candidate(RuntimeOrigin::signed(BOB)),
			Error::<Test>::AlreadyRegistered
		);
	})
}

#[test]
fn err_already_voted() {
	ExtBuilder::default().build().execute_with(|| {
		// Voter
		assert_ok!(TemplateModule::register_voter(RuntimeOrigin::signed(ALICE)));
		assert_eq!(TemplateModule::voter_account(ALICE).unwrap(), VoterInfo::new());

		// Candidate
		assert_ok!(TemplateModule::register_candidate(RuntimeOrigin::signed(BOB)));
		assert_eq!(TemplateModule::candidate_account(BOB).unwrap(), CandidateInfo::new());

		// Election
		assert_ok!(TemplateModule::config_election(RuntimeOrigin::root(), 1, 20));
		assert_eq!(TemplateModule::get_election().unwrap(), ElectionInfo::<Test>::voted());

		// BlockNumber
		System::set_block_number(5);

		// -------------------------------- Voting -------------------------------

		// Assert that ALICE gives vote to BOB is OK
		assert_ok!(TemplateModule::give_vote(RuntimeOrigin::signed(ALICE), BOB));
		// Assert that BOB's CandidateInfo has changed
		assert_eq!(TemplateModule::candidate_account(BOB).unwrap(), CandidateInfo::voted());
		// Assert that ALICE's VoterInfo
		assert_eq!(TemplateModule::voter_account(ALICE).unwrap(), VoterInfo::voted());
		// Assert that correct event was deposited
		System::assert_last_event(Event::VoteSuccess.into());

		assert_noop!(
			TemplateModule::give_vote(RuntimeOrigin::signed(ALICE), BOB),
			Error::<Test>::AlreadyVoted
		);
	})
}

#[test]
fn err_voting_election_not_configured() {
	ExtBuilder::default().build().execute_with(|| {
		// Voter
		assert_ok!(TemplateModule::register_voter(RuntimeOrigin::signed(ALICE)));
		assert_eq!(TemplateModule::voter_account(ALICE).unwrap(), VoterInfo::new());

		// Candidate
		assert_ok!(TemplateModule::register_candidate(RuntimeOrigin::signed(BOB)));
		assert_eq!(TemplateModule::candidate_account(BOB).unwrap(), CandidateInfo::new());

		// Election
		// assert_ok!(TemplateModule::config_election(RuntimeOrigin::root(), 1, 20));
		// assert_eq!(TemplateModule::get_election().unwrap(), ElectionInfo::<Test>::voted());

		// BlockNumber
		System::set_block_number(5);

		// -------------------------------- Voting -------------------------------

		assert_noop!(
			TemplateModule::give_vote(RuntimeOrigin::signed(ALICE), BOB),
			Error::<Test>::ElectionNotConfigured
		);
	})
}

#[test]
fn err_voting_not_registered_voter() {
	ExtBuilder::default().build().execute_with(|| {
		// Voter
		// assert_ok!(TemplateModule::register_voter(RuntimeOrigin::signed(ALICE)));
		// assert_eq!(TemplateModule::voter_account(ALICE).unwrap(), VoterInfo::new());

		// Candidate
		assert_ok!(TemplateModule::register_candidate(RuntimeOrigin::signed(BOB)));
		assert_eq!(TemplateModule::candidate_account(BOB).unwrap(), CandidateInfo::new());

		// Election
		assert_ok!(TemplateModule::config_election(RuntimeOrigin::root(), 1, 20));
		assert_eq!(TemplateModule::get_election().unwrap(), ElectionInfo::<Test>::voted());

		// BlockNumber
		System::set_block_number(5);

		// -------------------------------- Voting -------------------------------

		assert_noop!(
			TemplateModule::give_vote(RuntimeOrigin::signed(ALICE), BOB),
			Error::<Test>::NotRegistered
		);
	})
}

#[test]
fn err_voting_not_registered_candidate() {
	ExtBuilder::default().build().execute_with(|| {
		// Voter
		assert_ok!(TemplateModule::register_voter(RuntimeOrigin::signed(ALICE)));
		assert_eq!(TemplateModule::voter_account(ALICE).unwrap(), VoterInfo::new());

		// Candidate
		// assert_ok!(TemplateModule::register_candidate(RuntimeOrigin::signed(BOB)));
		// assert_eq!(TemplateModule::candidate_account(BOB).unwrap(), CandidateInfo::new());

		// Election
		assert_ok!(TemplateModule::config_election(RuntimeOrigin::root(), 1, 20));
		assert_eq!(TemplateModule::get_election().unwrap(), ElectionInfo::<Test>::voted());

		// BlockNumber
		System::set_block_number(5);

		// -------------------------------- Voting -------------------------------

		assert_noop!(
			TemplateModule::give_vote(RuntimeOrigin::signed(ALICE), BOB),
			Error::<Test>::NotRegistered
		);
	})
}

#[test]
fn err_winner() {
	ExtBuilder::default().build().execute_with(|| {
		// Election
		assert_ok!(TemplateModule::config_election(RuntimeOrigin::root(), 1, 20));
		assert_eq!(TemplateModule::get_election().unwrap(), ElectionInfo::<Test>::voted());

		// BlockNumber
		System::set_block_number(5);

		// -------------------------------- Voting -------------------------------

		assert_noop!(
			TemplateModule::winner(RuntimeOrigin::signed(ALICE)),
			Error::<Test>::ElectionNotEnded
		);
	})
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
