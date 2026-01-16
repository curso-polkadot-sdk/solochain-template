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

use codec::{Codec, DecodeWithMemTracking, HasCompact};
use frame_support::Parameter;
use scale_info::TypeInfo;
use sp_runtime::{
	sp_std::fmt::Debug,
	traits::{AtLeast32BitUnsigned, CheckedAdd, MaybeSerializeDeserialize, Member, One},
};

// Every callable function or "dispatchable" a pallet exposes must have weight values that correctly
// estimate a dispatchable's execution time. The benchmarking module is used to calculate weights
// for each dispatchable and generates this pallet's weight.rs file. Learn more about benchmarking here: https://docs.substrate.io/test/benchmark/
// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;
pub mod weights;
pub use weights::{SubstrateWeight, WeightInfo};

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
	use super::weights::WeightInfo;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;


	// The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
	// (`Call`s) in this pallet.
	#[pallet::pallet]
	pub struct Pallet<T, I = ()>(_);

	/// The pallet's configuration trait.
	///
	/// All our types and constants a pallet depends on must be declared here.
	/// These types are defined generically and made concrete when the pallet is declared in the
	/// `runtime/src/lib.rs` file of your chain.
	#[pallet::config]
	pub trait Config<I: 'static = ()>: pallet_balances::Config<I> + pallet_timestamp::Config + frame_system::Config {
		/// The overarching runtime event type.
		#[allow(deprecated)]
		type RuntimeEvent: From<Event<Self, I>>
			+ IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;
	}

	/// Valor é um número armazenado nesse pallet.
	/// P
	/// A > 1 && A < P
	/// B > 1 && B < P
	/// A * B == P
	#[pallet::storage]
	pub type Products<T: Config<I>, I: 'static = ()> = StorageMap<
		_,
		Blake2_128Concat,
		u128, // Challenge: é o resultado da multiplicação de dois numeros
		u32,  // Prize: premios que será pago para quem descobrir os dois numeros
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		Dummy,
	}

	#[pallet::error]
	pub enum Error<T, I = ()> {
		/// Account not authorized
		Unauthorized,
		/// Challenge doesn't exists
		ChallengeNotFound,
		/// Wrong solution
		WrongSolution,
	}

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config<I>>::WeightInfo::alterar_valor())]
		pub fn set_challenge(origin: OriginFor<T>, challenge: u128, prize: u32) -> DispatchResult {
			// TODO: emitir eventos

			// Check that the extrinsic was signed and get the signer.
			let conta = ensure_signed(origin)?;

			// 1 - Bloquear os fundos da conta
			// TODO: bloquear fundos aqui

			// 2 - Salva o challenge no storage.
			Products::<T, I>::insert(&challenge, prize);

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config<I>>::WeightInfo::alterar_valor())]
		pub fn submit_solution(origin: OriginFor<T>, challenge: u128, a: u128, b: u128) -> DispatchResult {
			// TODO: emitir eventos

			// Check that the extrinsic was signed and get the signer.
			let conta = ensure_signed(origin)?;

			// Verifica se o challenge existe
			let Some(prize) = Products::<T, I>::get(&challenge) else {
				return Err(Error::<T, I>::ChallengeNotFound.into());
			};

			// Multiplica A * B, falha se der overflow
			let Some(product) = a.checked_mul(b) else {
				return Err(Error::<T, I>::WrongSolution.into());
			};

			// Falha se `a*b !== challenge`
			if product != challenge {
				return Err(Error::<T, I>::WrongSolution.into());
			}

			// Desafio concluido
			// 1 - Pagar o premio para a `conta`
			// TODO: transferir para `conta` o `prize`.

			// 2 - Apagar o desafio
			Products::<T, I>::remove(&challenge);

			Ok(())
		}
	}
}
