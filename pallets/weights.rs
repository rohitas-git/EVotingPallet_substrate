
//! Autogenerated weights for `pallet_template`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-14, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Rohitas-ATR898`, CPU: `12th Gen Intel(R) Core(TM) i5-1235U`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_template
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// pallets/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_template`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_template::WeightInfo for WeightInfo<T> {
	/// Storage: TemplateModule AccountToVoterInfo (r:1 w:1)
	/// Proof: TemplateModule AccountToVoterInfo (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	fn add_voter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3547`
		// Minimum execution time: 10_151_000 picoseconds.
		Weight::from_parts(10_872_000, 0)
			.saturating_add(Weight::from_parts(0, 3547))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TemplateModule AccountToCandidateInfo (r:1 w:1)
	/// Proof: TemplateModule AccountToCandidateInfo (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn register_candidate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3517`
		// Minimum execution time: 10_158_000 picoseconds.
		Weight::from_parts(10_684_000, 0)
			.saturating_add(Weight::from_parts(0, 3517))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TemplateModule ElectionConfig (r:1 w:1)
	/// Proof: TemplateModule ElectionConfig (max_values: Some(1), max_size: Some(10), added: 505, mode: MaxEncodedLen)
	fn config_election() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `1495`
		// Minimum execution time: 9_196_000 picoseconds.
		Weight::from_parts(12_809_000, 0)
			.saturating_add(Weight::from_parts(0, 1495))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: TemplateModule ElectionConfig (r:1 w:0)
	/// Proof: TemplateModule ElectionConfig (max_values: Some(1), max_size: Some(10), added: 505, mode: MaxEncodedLen)
	/// Storage: TemplateModule AccountToVoterInfo (r:1 w:1)
	/// Proof: TemplateModule AccountToVoterInfo (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	/// Storage: TemplateModule AccountToCandidateInfo (r:1 w:1)
	/// Proof: TemplateModule AccountToCandidateInfo (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: TemplateModule MaxVote (r:1 w:1)
	/// Proof: TemplateModule MaxVote (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn give_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `185`
		//  Estimated: `10048`
		// Minimum execution time: 26_976_000 picoseconds.
		Weight::from_parts(27_948_000, 0)
			.saturating_add(Weight::from_parts(0, 10048))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: TemplateModule ElectionConfig (r:1 w:0)
	/// Proof: TemplateModule ElectionConfig (max_values: Some(1), max_size: Some(10), added: 505, mode: MaxEncodedLen)
	/// Storage: TemplateModule AccountToCandidateInfo (r:101 w:0)
	/// Proof: TemplateModule AccountToCandidateInfo (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: TemplateModule MaxVote (r:1 w:0)
	/// Proof: TemplateModule MaxVote (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: TemplateModule MaxVoteCandidate (r:0 w:1)
	/// Proof: TemplateModule MaxVoteCandidate (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	fn winner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5887`
		//  Estimated: `259201`
		// Minimum execution time: 498_291_000 picoseconds.
		Weight::from_parts(512_186_000, 0)
			.saturating_add(Weight::from_parts(0, 259201))
			.saturating_add(T::DbWeight::get().reads(103))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}