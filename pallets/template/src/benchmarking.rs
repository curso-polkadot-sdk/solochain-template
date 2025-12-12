//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn alterar_valor() {
		let value = SaldoOf::<T>::from(100u32);
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		alterar_valor(RawOrigin::Signed(caller), value);
		assert_eq!(Valor::<T>::get(), Some(value));
	}

	#[benchmark]
	fn cause_error() {
		let hundred = SaldoOf::<T>::from(100u32);
		let hundred_one = SaldoOf::<T>::from(101u32);
		Valor::<T>::put(hundred);
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		cause_error(RawOrigin::Signed(caller));
		assert_eq!(Valor::<T>::get(), Some(hundred_one));
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
