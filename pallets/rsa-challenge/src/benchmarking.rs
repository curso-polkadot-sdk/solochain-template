//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::{BalanceFor, ChallengeDetailsFor, CurrencyFor, pallet::Pallet as RsaChallenge, pallet::Products};
use frame_benchmarking::v2::*;
use frame_support::traits::Currency;
use frame_system::{pallet_prelude::AccountIdFor, RawOrigin};

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn set_challenge() {
		// Configuração do RSA Challenge.
		let caller: AccountIdFor<T> = whitelisted_caller();
		let prize = BalanceFor::<T>::from(1234u32);
		let end_date = pallet_timestamp::Pallet::<T>::get() + MomentFor::<T>::from(10u32);
		let details = ChallengeDetailsFor::<T> { owner: caller.clone(), prize, end_date };

		// Encontre um challenge que não existe
		let mut challenge = 77u128;
		while Products::<T>::get(&challenge).is_some() {
			challenge += 2;
		}

		// Adiciona fundos na conta.
		let mut new_balance =
			<CurrencyFor<T> as Currency<AccountIdFor<T>>>::total_balance(&details.owner);
		new_balance += <CurrencyFor<T> as Currency<AccountIdFor<T>>>::minimum_balance();
		new_balance += prize;
		let _ = <CurrencyFor<T> as Currency<AccountIdFor<T>>>::make_free_balance_be(
			&caller,
			new_balance,
		);

		// Desconsidera o custo de ler o `pallet_timestamp::Now`, pois esse
		// valor fica em cache.
		frame_benchmarking::benchmarking::add_to_whitelist(
			pallet_timestamp::Now::<T>::hashed_key().to_vec().into(),
		);

		// Chama a extrinsic
		#[extrinsic_call]
		set_challenge(RawOrigin::Signed(caller), challenge, details.prize, details.end_date);
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
