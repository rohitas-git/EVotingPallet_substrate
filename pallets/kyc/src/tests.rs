use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn test_kyc_verification_of_customer() {
	new_test_ext().execute_with(|| {

		// Submit a Customer KYC

		// Verify that KYC given Customer Permission
		assert_eq!(KYC::verify_customer(RuntimeOrigin::signed(1), CustomerId), true);
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			TemplateModule::cause_error(RuntimeOrigin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}
