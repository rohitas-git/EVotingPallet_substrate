#![allow(unused_imports)]
use crate::{
	mock::*, AccountToVoterInfo, CandidateInfo, Config, ElectionConfig, ElectionInfo, Error, Event,
	VoterInfo,
};
use frame_support::{assert_noop, assert_ok};
// use crate as pallet_template;

// ! Funda: give when then
// ! Wrap it with fn set_election_time
// ! Remove code duplication
// ! Better Naming (Descriptive names also work)
// ! Split code into meaningful files
// ! Replace number with meaningful const/variable

#[test]
fn test_register_voter() {
	ExtBuilder::default().build().execute_with(|| {
		set_current_time(TIME_DURING_ELECTION);

		assert_ok!(register_voter(who(ALICE)));
		assert_eq!(TemplateModule::voter_account(ALICE).unwrap(), VoterInfo::new());

		System::assert_last_event(Event::RegisterVoter.into());
	})
}

#[test]
fn test_register_candidate() {
	ExtBuilder::default().build().execute_with(|| {
		set_current_time(TIME_DURING_ELECTION);

		assert_ok!(register_candidate(who(BOB)));
		assert_eq!(TemplateModule::candidate_account(BOB).unwrap(), CandidateInfo::new());

		System::assert_last_event(Event::RegisterCandidate.into());
	})
}

#[test]
fn test_election_configured() {
	ExtBuilder::default().build().execute_with(|| {
		set_current_time(TIME_DURING_ELECTION);

		assert_ok!(configure_election_start_and_end_time());
		assert_eq!(TemplateModule::get_election().unwrap(), ElectionInfo::<Test>::voted());

		System::assert_last_event(Event::ElectionConfigured.into());
	})
}


#[test]
fn test_voter_giving_vote_to_candidate_during_election() {
	ExtBuilder::default().build().execute_with(|| {
		setup_for_one_voter_one_candidate_and_election_time();
		set_current_time(TIME_DURING_ELECTION);

		give_vote(who(ALICE), BOB);

		assert_eq!(TemplateModule::candidate_account(BOB).unwrap(), CandidateInfo::voted());
		assert_eq!(TemplateModule::voter_account(ALICE).unwrap(), VoterInfo::voted());

		System::assert_last_event(Event::VoteSuccess.into());
	})
}

#[test]
fn test_voter_giving_vote_to_candidate_before_election() {
	new_test_ext().execute_with(|| {
		setup_for_one_voter_one_candidate_and_election_time();
		set_current_time(TIME_BEFORE_ELECTION);

		assert_noop!(give_vote(who(ALICE), BOB), Error::<Test>::ElectionNotStarted);
	})
}

#[test]
fn test_voter_giving_vote_to_candidate_after_election() {
	new_test_ext().execute_with(|| {
		setup_for_one_voter_one_candidate_and_election_time();
		set_current_time(TIME_AFTER_ELECTION);

		assert_noop!(give_vote(who(ALICE), BOB), Error::<Test>::ElectionEnded);
	})
}

#[test]
fn test_decided_winner_after_election_ended() {
	ExtBuilder::default().build().execute_with(|| {
		register_voter(who(ALICE));
		register_voter(who(BOB));
		register_voter(who(DAVE));
		register_voter(who(JOHN));
		register_voter(who(RON));

		register_candidate(who(DAVE));
		register_candidate(who(RON));
		register_candidate(who(JOHN));

		configure_election_start_and_end_time();

		// -------------------------------- Voting -------------------------------
		set_current_time(TIME_DURING_ELECTION);

		give_vote(who(ALICE), DAVE);
		give_vote(who(JOHN), DAVE);
		give_vote(who(DAVE), RON);
		give_vote(who(BOB), RON);
		give_vote(who(RON), JOHN);

		/* --------------------------------- Decide Winner --------------------------------- */
		set_current_time(TIME_AFTER_ELECTION);

		// println!("Max Votes: {}",TemplateModule::max_votes());
		// println!("Winner Vec: {:?}", TemplateModule::max_votes_candidate().clone().unwrap_or_default());

		use frame_support::pallet_prelude::ConstU32;
		use frame_support::BoundedVec;
		let win: BoundedVec<<Test as frame_system::Config>::AccountId, ConstU32<100>> =
			vec![DAVE, RON].try_into().unwrap();

		assert_ok!(who_won_elections());
		assert_eq!(TemplateModule::max_votes_candidate().unwrap(), win);

		System::assert_last_event(Event::WinnerVecStored.into());
	})
}

#[test]
fn test_raise_error_when_voter_registers_twice() {
	ExtBuilder::default().build().execute_with(|| {
		set_current_time(TIME_DURING_ELECTION);

		register_voter(who(ALICE));
		assert_noop!(register_voter(who(ALICE)), Error::<Test>::AlreadyRegistered);
	})
}

#[test]
fn test_raise_error_when_candidate_registers_twice() {
	ExtBuilder::default().build().execute_with(|| {
		set_current_time(TIME_DURING_ELECTION);

		register_candidate(who(BOB));
		assert_noop!(register_candidate(who(BOB)), Error::<Test>::AlreadyRegistered);
	})
}

#[test]
fn test_raise_error_when_voter_already_voted() {
	ExtBuilder::default().build().execute_with(|| {
		register_voter(who(ALICE));
		register_candidate(who(BOB));

		configure_election_start_and_end_time();
		set_current_time(TIME_DURING_ELECTION);

		give_vote(who(ALICE), BOB);
		assert_noop!(give_vote(who(ALICE), BOB), Error::<Test>::AlreadyVoted);
	})
}

#[test]
fn test_raise_error_for_voting_when_election_not_configured() {
	ExtBuilder::default().build().execute_with(|| {
		set_current_time(TIME_DURING_ELECTION);
		register_voter(who(ALICE));
		register_candidate(who(BOB));

		assert_noop!(give_vote(who(ALICE), BOB), Error::<Test>::ElectionNotConfigured);
	})
}

#[test]
fn test_raise_error_when_voting_by_not_registered_voter() {
	ExtBuilder::default().build().execute_with(|| {
		register_candidate(who(BOB));

		configure_election_start_and_end_time();
		set_current_time(TIME_DURING_ELECTION);

		assert_noop!(give_vote(who(ALICE), BOB), Error::<Test>::NotRegistered);
	})
}

#[test]
fn test_raise_error_when_voting_to_not_registered_candidate() {
	ExtBuilder::default().build().execute_with(|| {
		register_voter(who(ALICE));

		configure_election_start_and_end_time();
		set_current_time(TIME_DURING_ELECTION);

		assert_noop!(give_vote(who(ALICE), BOB), Error::<Test>::NotRegistered);
	})
}

#[test]
fn test_raise_error_when_calling_winner_before_election_end() {
	ExtBuilder::default().build().execute_with(|| {
		setup_for_one_voter_one_candidate_and_election_time();
		set_current_time(TIME_DURING_ELECTION);

		assert_noop!(who_won_elections(), Error::<Test>::ElectionNotEnded);
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
		let start = ELECTION_START_TIME;
		let end = ELECTION_END_TIME;
		ElectionInfo::set(start, end)
	}
}
