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
	FixedPointOperand,
};

/// Apelido utilizado para se referir ao `Saldo` definido
/// na configuração desse pallet.
/// ```rust
/// # use pallet_template::{SaldoOf, Config};
/// // fazer isso:
/// fn foo<T: Config>(saldo: <T as pallet_template::Config>::Saldo) {};
///
/// // é equivalente a fazer isso
/// fn bar<T: Config>(saldo: SaldoOf<T>) {}
/// ```
pub type SaldoOf<T> = <T as pallet::Config>::Saldo;

// Every callable function or "dispatchable" a pallet exposes must have weight values that correctly
// estimate a dispatchable's execution time. The benchmarking module is used to calculate weights
// for each dispatchable and generates this pallet's weight.rs file. Learn more about benchmarking here: https://docs.substrate.io/test/benchmark/
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::{SubstrateWeight, WeightInfo};

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
	// Import various useful types required by all FRAME pallets.
	use super::{
		AtLeast32BitUnsigned, CheckedAdd, Codec, Debug, DecodeWithMemTracking, FixedPointOperand,
		HasCompact, MaybeSerializeDeserialize, Member, One, Parameter, SaldoOf, TypeInfo,
		WeightInfo,
	};
	use frame_support::pallet_prelude::{
		DispatchResult,
		IsType,
		MaxEncodedLen,
		StorageDoubleMap,
		StorageValue,
		Identity,        // -> CHAVE é uniformemente distribuida, e o usuário não consegue escolher ela.
		Twox64Concat,    // -> CHAVE não é uniformemente distribuida, e o usuário não consegue escolher ela.
		Twox128,
		Blake2_128,      // -> O usuário pode influenciar o valor da storage key
		Blake2_128Concat,
	};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::{ensure_signed, OriginFor};

	// The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
	// (`Call`s) in this pallet.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// The pallet's configuration trait.
	///
	/// All our types and constants a pallet depends on must be declared here.
	/// These types are defined generically and made concrete when the pallet is declared in the
	/// `runtime/src/lib.rs` file of your chain.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching runtime event type.
		#[allow(deprecated)]
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Tipo que representa um saldo na blockchain
		type Saldo: Parameter
			+ Member
			+ AtLeast32BitUnsigned
			+ Codec
			+ HasCompact<Type: DecodeWithMemTracking>
			+ Default
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ MaxEncodedLen
			+ TypeInfo
			+ One
			+ FixedPointOperand;

		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;
	}

	/// Valor é um número armazenado nesse pallet.
	///
	/// Nesse template esta sendo declarado um Item no storage chamado `Valor` que armazena um
	/// valor do tipo `Config::Saldo`.
	///
	/// Aprenda mais sobre storage aqui: <https://docs.substrate.io/build/runtime-storage/>
	#[pallet::storage]
	pub type Valor<T: Config> = StorageValue<_, T::Saldo>;

	//       FUNÇÃO(X)        |      SAIDA
	// -----------------------|-----------------------
	//   Identity(X)          |  X
	//   Twox128(X)           |  Twox64(X, 0) + Twox64(X, 1)
	//   Blake2_128(X)        |  Blake2_128(X)
	//   Twox64Concat(X)      |  Twox64(X) + X
	//   Blake2_128Concat(X)  |  Blake2_128(X) + X
	//
	//        FUNÇÃO(X)       |      QUANDO USAR
	// -----------------------|------------------------------------------------
	//                        |  A chave `X` é uniformemente distribuida
	//        Identity        |  Um adversário não consegue escolher a chave `X`.
	//                        |  Ex: A chave é igual ao blake2 do valor armazenado.
	// -----------------------|------------------------------------------------
	//         Twox128        |  A chave `X` não é uniformemente distribuida.
	//         Twox64         |  Um adversário não consegue escolher a chave `X`.
	//                        |  Ex: A entrada é uma texto, nome do pallet, etc.
	// -----------------------|------------------------------------------------
	//        Blake2_256      |  Um adversário consegue escolher a chave `X`
	//        Blake2_128      |  
	//                        |  Ex: Mais segura, na duvída sempre utiliza essas
	// ------------------------------------------------------------------------
	//
	// # Quando utilizar as varições com `*Concat` como `Blake2_128Concat`, etc.
	// DICA: Utilize variações com `*_CONCAT` quando a chave X é pequena e precisa
	// ser listada on-chain ou off-chain.

	/// Metadata of a collection.
	#[pallet::storage]
	pub type Tokens<T: Config> = StorageMap<
		_,
		Blake2_128Concat,   // Função que recebe a CHAVE e retorna o sufixo da storage key.
		u32,                // tipo da CHAVE
		T::AccountId,       // tipo do VALOR
		OptionQuery,        // Define o que é retornado ao ler uma CHAVE que não existe.
	>;

	/// Events that functions in this pallet can emit.
	///
	/// Events are a simple means of indicating to the outside world (such as dApps, chain explorers
	/// or other users) that some notable update in the runtime has occurred. In a FRAME pallet, the
	/// documentation for each event field and its parameters is added to a node's metadata so it
	/// can be used by external interfaces or tools.
	///
	///	The `generate_deposit` macro generates a function on `Pallet` called `deposit_event` which
	/// will convert the event type of your pallet into `RuntimeEvent` (declared in the pallet's
	/// [`Config`] trait) and deposit it using [`frame_system::Pallet::deposit_event`].
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A user has successfully set a new value.
		ValorArmazenado {
			/// The new value set.
			valor: SaldoOf<T>,
			/// The account who set the new value.
			conta: T::AccountId,
		},
	}

	/// Errors that can be returned by this pallet.
	///
	/// Errors tell users that something went wrong so it's important that their naming is
	/// informative. Similar to events, error documentation is added to a node's metadata so it's
	/// equally important that they have helpful documentation associated with them.
	///
	/// This type of runtime error can be up to 4 bytes in size should you want to return additional
	/// information.
	#[pallet::error]
	pub enum Error<T> {
		/// The value retrieved was `None` as no value was previously set.
		NoneValue,
		/// There was an attempt to increment the value in storage over `u32::MAX`.
		StorageOverflow,
		/// O token já existe
		TokenJaExiste,
		/// O token não existe
		TokenNotFound,
		/// O token não existe
		Unauthorized,
	}

	/// The pallet's dispatchable functions ([`Call`]s).
	///
	/// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	/// These functions materialize as "extrinsics", which are often compared to transactions.
	/// They must always return a `DispatchResult` and be annotated with a weight and call index.
	///
	/// The [`call_index`] macro is used to explicitly
	/// define an index for calls in the [`Call`] enum. This is useful for pallets that may
	/// introduce new dispatchables over time. If the order of a dispatchable changes, its index
	/// will also change which will break backwards compatibility.
	///
	/// The [`weight`] macro is used to assign a weight to each call.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a single u32 value as a parameter, writes the value
		/// to storage and emits an event.
		///
		/// It checks that the _origin_ for this call is _Signed_ and returns a dispatch
		/// error if it isn't. Learn more about origins here: <https://docs.substrate.io/build/origins/>
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::alterar_valor())]
		pub fn alterar_valor(origin: OriginFor<T>, valor: SaldoOf<T>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let conta = ensure_signed(origin)?;

			// Update storage.
			Valor::<T>::put(valor);

			// Emit an event.
			Self::deposit_event(Event::ValorArmazenado { valor, conta });

			// Return a successful `DispatchResult`
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		///
		/// It checks that the caller is a signed origin and reads the current value from the
		/// `Something` storage item. If a current value exists, it is incremented by 1 and then
		/// written back to storage.
		///
		/// ## Errors
		///
		/// The function will return an error under the following conditions:
		///
		/// - If no value has been set ([`Error::NoneValue`])
		/// - If incrementing the value in storage causes an arithmetic overflow
		///   ([`Error::StorageOverflow`])
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match Valor::<T>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage. This will cause an error in the event
					// of overflow.
					let one = <SaldoOf<T> as One>::one();
					let new = old.checked_add(&one).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					Valor::<T>::put(new);
					Ok(())
				},
			}
		}

		/// Método que incrementa o `Valor` em uma unidade.
		/// pode retornar um erro em dois casos:
		/// - O valor não foi definido
		/// - Overflow
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::incrementar())]
		pub fn incrementar(origin: OriginFor<T>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// Verifica se essa `extrinsic` foi assinada:
			// - Se foi assinada, retorna quem assinou.
			// - Se não foi assinada, retorna um erro.
			let conta = ensure_signed(origin)?;

			// Le o `Valor` que esta armazenado no storage.
			let valor = match Valor::<T>::get() {
				None => {
					// Valor não existe, retorne um erro, isso irá reverter
					// essa transação.
					return Err(Error::<T>::NoneValue.into())
				},
				Some(old) => {
					// Incrementa o `Valor` em 1 unidade.
					// Um erro será retornado em caso de overflow.
					let one = <SaldoOf<T> as One>::one();
					let new = old.checked_add(&one).ok_or(Error::<T>::StorageOverflow)?;

					// Atualiza o valor armazenado no storage.
					Valor::<T>::put(new);
					new
				},
			};

			// Emite um evento
			Self::deposit_event(Event::ValorArmazenado { valor, conta });

			// Retorna que a transação foi executa com sucesso.
			Ok(())
		}

		/// Cria um novo NFT e o transfere para a conta de quem assinou a transação.
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::incrementar())]
		pub fn mint(origin: OriginFor<T>, token_id: u32) -> DispatchResult {
			let conta = ensure_signed(origin)?;

			if !Tokens::<T>::contains_key(&token_id)  {
				Tokens::<T>::insert(token_id, conta);
			} else {
				return Err(Error::<T>::TokenJaExiste.into());
			}

			Ok(())
		}

		/// Destroi um NFT se quem assinou a transação for o dono dele.
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::incrementar())]
		pub fn burn(origin: OriginFor<T>, token_id: u32) -> DispatchResult {
			let conta = ensure_signed(origin)?;
			
			if let Some(owner) = Tokens::<T>::get(&token_id) {
				if conta == owner {
					Tokens::<T>::remove(&token_id);
				} else {
					return Err(Error::<T>::Unauthorized.into());
				} 
			} else {
				return Err(Error::<T>::TokenNotFound.into());
			}

			Ok(())
		}
	}
}
