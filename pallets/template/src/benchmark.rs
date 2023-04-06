// Use empirical measurements of the runtime to determine the time it takes
// to execute the extrinsics and other runtime logic.

// Run benchmarks using worst case scenario conditions
// Primary goal is to keep the runtime safe
// Secondary goal is to be accurate as possible to maxime the throughput

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::v1::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	do_something {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), s)
	verify {
		assert_eq!(Something::<T>::get(), Some(s));
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}