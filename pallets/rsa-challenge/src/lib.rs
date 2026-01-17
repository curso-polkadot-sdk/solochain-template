//! # Template Pallet
//!
//! A pallet with minimal functionality to help developers understand the essential components of
//! writing a FRAME pallet. It is typically used in beginner tutorials or in Substrate template
//! nodes as a starting point for creating a new pallet and **not meant to be used in production**.
//!
//! ## Overview
//!
//! This template pallet contains basic examples of:
//! - declaring a storage item that stores a single `u32` value
//! - declaring and using events
//! - declaring and using errors
//! - a dispatchable function that allows a user to set a new value to storage and emits an event
//!   upon success
//! - another dispatchable function that causes a custom error to be thrown
//!
//! Each pallet section is annotated with an attribute using the `#[pallet::...]` procedural macro.
//! This macro generates the necessary code for a pallet to be aggregated into a FRAME runtime.
//!
//! Learn more about FRAME macros [here](https://docs.substrate.io/reference/frame-macros/).
//!
//! ### Pallet Sections
//!
//! The pallet sections in this template are:
//!
//! - A **configuration trait** that defines the types and parameters which the pallet depends on
//!   (denoted by the `#[pallet::config]` attribute). See: [`Config`].
//! - A **means to store pallet-specific data** (denoted by the `#[pallet::storage]` attribute).
//!   See: [`storage_types`].
//! - A **declaration of the events** this pallet emits (denoted by the `#[pallet::event]`
//!   attribute). See: [`Event`].
//! - A **declaration of the errors** that this pallet can throw (denoted by the `#[pallet::error]`
//!   attribute). See: [`Error`].
//! - A **set of dispatchable functions** that define the pallet's functionality (denoted by the
//!   `#[pallet::call]` attribute). See: [`dispatchables`].
//!
//! Run `cargo doc --package pallet-template --open` to view this pallet's documentation.

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
// Learn about pallet unit testing here: https://docs.substrate.io/test/unit-testing/
#[cfg(test)]
mod tests;

// Tipos utilizados internamente por esse pallet.
pub mod types;

// Every callable function or "dispatchable" a pallet exposes must have weight values that correctly
// estimate a dispatchable's execution time. The benchmarking module is used to calculate weights
// for each dispatchable and generates this pallet's weight.rs file. Learn more about benchmarking here: https://docs.substrate.io/test/benchmark/
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::{SubstrateWeight, WeightInfo};

use frame_support::traits::Currency;
use frame_system::pallet_prelude::AccountIdFor;

// Apelidos para tipois comuns.
pub type CurrencyFor<T, I = ()> = pallet_balances::Pallet<T, I>;
pub type BalanceFor<T, I = ()> = <CurrencyFor<T, I> as Currency<AccountIdFor<T>>>::Balance;
pub type MomentFor<T> = <T as pallet_timestamp::Config>::Moment;
pub type ChallengeDetailsFor<T, I = ()> = types::ChallengeDetails<
	<T as frame_system::Config>::AccountId,
	<T as pallet_balances::Config<I>>::Balance,
	<T as pallet_timestamp::Config>::Moment,
>;

#[frame_support::pallet]
pub mod pallet {
	use super::{
		types::ChallengeDetails, weights::WeightInfo, BalanceFor, ChallengeDetailsFor, CurrencyFor,
		MomentFor,
	};
	use frame_support::traits::{Currency, ExistenceRequirement, ReservableCurrency};
	#[allow(unused_imports)]
	use frame_support::{
		dispatch::DispatchResult,
		sp_runtime::{
			codec::{Codec, Decode, DecodeWithMemTracking, Encode, HasCompact, MaxEncodedLen},
			scale_info::TypeInfo,
			sp_std::fmt::Debug,
			traits::{
				AtLeast32BitUnsigned, CheckedAdd, Lookup, MaybeSerializeDeserialize, Member, One,
			},
		},
		storage::types::{
			OptionQuery, ResultQuery, StorageDoubleMap, StorageMap, StorageNMap, StorageValue,
			ValueQuery,
		},
		traits::{Hooks, IsType},
		weights::Weight,
		Blake2_128, Blake2_128Concat, Blake2_256, Identity, Parameter, Twox128, Twox256,
		Twox64Concat,
	};
	#[allow(unused_imports)]
	use frame_system::{
		ensure_authorized, ensure_none, ensure_root, ensure_signed, ensure_signed_or_root,
		pallet_prelude::{
			AccountIdFor, BlockNumberFor, ExtrinsicFor, HeaderFor, OriginFor, RuntimeCallFor,
		},
	};

	// The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
	// (`Call`s) in this pallet.
	#[pallet::pallet]
	pub struct Pallet<T, I = ()>(_);

	/// Configuração do pallet
	#[pallet::config]
	pub trait Config<I: 'static = ()>:
		pallet_balances::Config<I> + pallet_timestamp::Config + frame_system::Config
	{
		/// The overarching runtime event type.
		#[allow(deprecated)]
		type RuntimeEvent: From<Event<Self, I>>
			+ IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;
	}

	/// Armazena os detalhes de um RSA Challenge.
	#[pallet::storage]
	pub type Products<T: Config<I>, I: 'static = ()> = StorageMap<
		_,
		Blake2_128Concat,
		u128,                      // RSA Challenge
		ChallengeDetailsFor<T, I>, // Detalhes do Challenge
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		ChallangeCreated { challenge: u128, prize: <T as pallet_balances::Config<I>>::Balance },
	}

	#[pallet::error]
	pub enum Error<T, I = ()> {
		/// Account not authorized
		Unauthorized,
		/// Challenge doesn't exists
		ChallengeNotFound,
		/// Attempt to create a challenge that already exists
		ChallengeAlreadyExists,
		/// Attempt to create a challenge that already exists
		InvalidChallenge,
		/// Wrong solution
		WrongSolution,
	}

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config<I>>::WeightInfo::set_challenge())]
		pub fn set_challenge(
			origin: OriginFor<T>,
			#[pallet::compact] challenge: u128,
			prize: BalanceFor<T, I>,
			end_date: MomentFor<T>,
		) -> DispatchResult {
			// 1. Garante que a origem é uma transação assinada por uma conta.
			let account = ensure_signed(origin)?;

			// 2. Verifica se a conta `account` possui fundos suficientes.
			if !<CurrencyFor<T, I> as ReservableCurrency<AccountIdFor<T>>>::can_reserve(
				&account, prize,
			) {
				return Err(Error::<T, I>::InvalidChallenge.into());
			}

			// 3. Verifica se esse challenge já existe.
			if Products::<T, I>::get(&challenge).is_some() {
				return Err(Error::<T, I>::ChallengeAlreadyExists.into());
			};

			// 4. Bloquear os fundos da conta, ou falhar se a conta não tiver fundos.
			<CurrencyFor<T, I> as ReservableCurrency<AccountIdFor<T>>>::reserve(&account, prize)?;

			// 5. Salva o challenge no storage.
			let now = pallet_timestamp::Pallet::<T>::get();
			if now >= end_date {
				return Err(Error::<T, I>::InvalidChallenge.into());
			}

			// 6. Salva o challenge no storage.
			let challenge_details = ChallengeDetails { owner: account, prize, end_date };
			Products::<T, I>::insert(&challenge, challenge_details);

			// 7. Emite um evento informando que o challenge foi criado.
			Self::deposit_event(Event::<T, I>::ChallangeCreated { challenge, prize });

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config<I>>::WeightInfo::set_challenge())]
		pub fn submit_solution(
			origin: OriginFor<T>,
			challenge: u128,
			a: u128,
			b: u128,
		) -> DispatchResult {
			// 1. Garante que a origem é uma transação assinada por uma conta.
			let account = ensure_signed(origin)?;

			// 2. Verifica se o challenge existe, se não retorne um erro.
			let Some(details) = Products::<T, I>::get(&challenge) else {
				return Err(Error::<T, I>::ChallengeNotFound.into());
			};

			// 3. Garante que a origem é uma transação assinada por uma conta.
			if &details.owner == &account {
				return Err(Error::<T, I>::Unauthorized.into());
			}

			// 4. Calcula `product = a * b`, falha se der overflow
			let Some(product) = a.checked_mul(b) else {
				return Err(Error::<T, I>::WrongSolution.into());
			};

			// 5. Falha se `a == 1` ou `b == 1` ou `a * b != challenge`
			if a == 1 || b == 1 || product != challenge {
				return Err(Error::<T, I>::WrongSolution.into());
			}

			// 6. Libera os fundos bloqueados na conta do criador do challenge.
			<CurrencyFor<T, I> as ReservableCurrency<AccountIdFor<T>>>::unreserve(
				&details.owner,
				details.prize,
			);

			// 7. Transfere os fundos do criador para a conta que resolveu o challenge
			<CurrencyFor<T, I> as Currency<AccountIdFor<T>>>::transfer(
				&details.owner,
				&account,
				details.prize,
				ExistenceRequirement::AllowDeath,
			)?;

			// 8 - Remove o desafio do storage
			Products::<T, I>::remove(&challenge);

			// 9. Emite um evento informando que o desafio foi concluído.
			// TODO

			Ok(())
		}
	}
}
