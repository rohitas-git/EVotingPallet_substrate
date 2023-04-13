use crate::{self as pallet_template, weights::TemplateWeightInfo};
use frame_support::{
	pallet_prelude::DispatchResult,
	traits::{ConstU16, ConstU64},
};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

pub type AccountId = u64;
pub type BlockNumber = u64;
type Origin = <Test as frame_system::Config>::RuntimeOrigin;

// type Origin = <Test as frame_system::trait>::Origin;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const DAVE: AccountId = 3;
pub const RON: AccountId = 4;
pub const JOHN: AccountId = 5;

pub fn root_user() -> Origin {
	RuntimeOrigin::root()
}
pub fn who(who: AccountId) -> Origin {
	RuntimeOrigin::signed(who)
}

pub const ELECTION_START_TIME: u64 = 5;
pub const ELECTION_END_TIME: u64 = 25;
pub const TIME_BEFORE_ELECTION: u64 = 2;
pub const TIME_DURING_ELECTION: u64 = 10;
pub const TIME_AFTER_ELECTION: u64 = 40;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		TemplateModule: pallet_template,
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_template::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = TemplateWeightInfo<Test>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

pub struct ExtBuilder;

impl Default for ExtBuilder {
	fn default() -> Self {
		ExtBuilder
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

		t.into()
	}
}

pub fn setup_for_one_voter_one_candidate_and_election_time() {
	register_voter(who(ALICE));
	register_candidate(who(BOB));
	configure_election_start_and_end_time();
}

pub fn set_current_time(time: u64) {
	System::set_block_number(time);
}

pub fn register_voter(who: Origin) -> DispatchResult {
	TemplateModule::register_voter(who)
}

pub fn register_voters(arr: &[AccountId]) {
	for id in 0..arr.len() {
		register_voter(who(arr.get(id).unwrap().clone()));
	}
}

pub fn register_candidate(who: Origin) -> DispatchResult {
	TemplateModule::register_candidate(who)
}

pub fn register_candidates(arr: &[AccountId]) {
	for id in 0..arr.len() {
		register_candidate(who(arr.get(id).unwrap().clone()));
	}
}

pub fn give_vote(from: Origin, to: AccountId) -> DispatchResult {
	TemplateModule::give_vote(from, to)
}

pub fn configure_election_start_and_end_time() -> DispatchResult {
	TemplateModule::config_election(root_user(), ELECTION_START_TIME, ELECTION_END_TIME)
}

pub fn who_won_elections() -> DispatchResult {
	TemplateModule::winner(who(ALICE))
}

// pub fn account_info_of_voter(whose: AccountId)-> Option<>{

// }

// Both are Equivalent:
// -ExtBuilder::default().build()
// -new_test_ext()
