//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::{pallet_prelude::AccountIdFor, RawOrigin};

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn alterar_valor() {
		let value = TokenIdOf::<T>::from(100u32);
		let caller: AccountIdFor<T> = whitelisted_caller();
		#[extrinsic_call]
		alterar_valor(RawOrigin::Signed(caller), value);
		assert_eq!(NextToken::<T>::get(), Some(value));
	}

	#[benchmark]
	fn cause_error() {
		let hundred = TokenIdOf::<T>::from(100u32);
		let hundred_one = TokenIdOf::<T>::from(101u32);
		NextToken::<T>::put(hundred);
		let caller: AccountIdFor<T> = whitelisted_caller();
		#[extrinsic_call]
		cause_error(RawOrigin::Signed(caller));
		assert_eq!(NextToken::<T>::get(), Some(hundred_one));
	}

	#[benchmark]
	fn incrementar() {
		let hundred = TokenIdOf::<T>::from(100u32);
		let hundred_one = TokenIdOf::<T>::from(101u32);
		NextToken::<T>::put(hundred);
		let caller: AccountIdFor<T> = whitelisted_caller();
		#[extrinsic_call]
		incrementar(RawOrigin::Signed(caller));
		assert_eq!(NextToken::<T>::get(), Some(hundred_one));
	}

	#[benchmark]
	fn mint(x: Linear<7, 1_000>) {
		// setup
		let caller: AccountIdFor<T> = whitelisted_caller();
		let token_id: TokenIdOf<T> = x.into();
		assert_eq!(Tokens::<T>::get(&token_id), None);

		#[extrinsic_call]
		mint(RawOrigin::Signed(caller.clone()), token_id);

		// verification
		assert_eq!(Tokens::<T>::get(&token_id), Some(caller));
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
