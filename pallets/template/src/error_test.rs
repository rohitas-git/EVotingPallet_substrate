use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

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
