use crate::{mock::*, ChallengeDetailsFor, Products};
use frame_support::{assert_ok, traits::fungible::Inspect};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Seta o bloco atual como bloco 1
		System::set_block_number(1);

		// Parametros do Challenge.
		let challenge = 77u128;
		let details =
			ChallengeDetailsFor::<Test> { owner: 1, prize: 1234, end_date: Timestamp::get() + 10 };

		// Adiciona fundos na conta.
		let new_balance =
			Balances::balance(&details.owner) + Balances::minimum_balance() + details.prize;
		Balances::force_set_balance(RuntimeOrigin::root(), details.owner, new_balance).unwrap();

		// O challenge não deve existir
		assert_eq!(Products::<Test>::get(&challenge), None);

		// Cria o Challenge
		assert_ok!(RsaChallenge::set_challenge(
			RuntimeOrigin::signed(details.owner),
			challenge,
			details.prize,
			details.end_date,
		));

		// O challenge foi criado com sucesso
		assert_eq!(Products::<Test>::get(&challenge), Some(details));
	});
}
