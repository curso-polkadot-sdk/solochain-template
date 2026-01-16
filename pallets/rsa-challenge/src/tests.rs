use crate::{mock::*, Products};
use frame_support::assert_ok;

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		let challenge = 77u128;
		let prize = 10u32;
		assert_eq!(Products::<Test>::get(&challenge), None);
		assert_ok!(RsaChallenge::set_challenge(RuntimeOrigin::signed(1), challenge, prize));
	});
}
