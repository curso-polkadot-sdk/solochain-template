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

/// Apelido utilizado para se referir ao `TokenId` definido
/// na configuração desse pallet.
/// ```rust
/// # use pallet_template::{TokenIdOf, Config};
/// // fazer isso:
/// fn foo<T: Config<I>, I: 'static>(saldo: <T as pallet_template::Config<I>>::TokenId) {};
///
/// // é equivalente a fazer isso
/// fn bar<T: Config<I>, I: 'static>(saldo: TokenIdOf<T, I>) {}
/// ```
pub type TokenIdOf<T, I = ()> = <T as pallet::Config<I>>::TokenId;

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
	use super::{TokenIdOf, WeightInfo};

	#[allow(unused_imports)]
	use frame_support::{
		dispatch::DispatchResult,
		sp_runtime::{
			codec::{Codec, DecodeWithMemTracking, HasCompact, MaxEncodedLen},
			scale_info::TypeInfo,
			sp_std::fmt::Debug,
			traits::{AtLeast32BitUnsigned, CheckedAdd, MaybeSerializeDeserialize, Member, One},
		},
		storage::types::{
			// ## QUERIES ##
			// Define o que será retornado pelo STORAGE MAP quando a chave
			// não existir no storage.
			// IMPORTANTE: Isso é incluído no metadata do runtime, logo tbm
			// afeta o que será retornado por um cliente interagindo com a
			// blockchain, ex: cliente Web utilizando o polkadot-api.
			//
			// Option: Se a chave não existir, retorne Option::None.
			OptionQuery,

			// Result: Se a chave não existir, retorne Result::Err(error).
			ResultQuery,

			// Mapeia duas chaves para valor: [x,y] -> value
			StorageDoubleMap,

			// Mapeia uma chave para valor:     key -> value
			StorageMap,

			// Mapeia N chaves para valor:    [..n] -> value
			StorageNMap,

			// Mapeia duas chaves para valor: [x,y] -> value
			StorageValue,

			//  Value: Se a chave não existir, retorne Default::default().
			ValueQuery,
		},

		// outros tipos
		traits::{Hooks, IsType},
		weights::Weight,
		// f(x) = blake2b(x, 128)
		Blake2_128,

		// f(x) = blake2b(x, 128) | x
		Blake2_128Concat,

		// f(x) = blake2b(x, 256)
		Blake2_256,

		// ## STORAGE HASHER ##
		// É uma função que mapeia uma chave `X` para bytes que serão
		// concatenados na chave final do storage, lembre-se que é um
		// banco de dados chave-valor.
		// Código: https://github.com/paritytech/polkadot-sdk/blob/polkadot-stable2512/substrate/primitives/crypto/hashing/src/lib.rs#L63-L123
		// NOTA: `a | b` significa concatenar a e b.
		//
		// f(x) = x
		Identity,

		Parameter,
		// f(x) = xxhash64(x, 0) | xxhash64(x, 1)
		Twox128,

		// f(x) = xxhash64(x, 0) | xxhash64(x, 1) | xxhash64(x, 2) | xxhash64(x, 3)
		Twox256,

		// f(x) = xxhash64(x, 0) | x
		Twox64Concat,
	};
	#[allow(unused_imports)]
	use frame_system::{
		ensure_none,   // Verifica que a origem é uma transação não assinada.
		ensure_root,   // Verifica que a origem é o super usuário.
		ensure_signed, // Verifica que a origem representa uma transação assinada.
		pallet_prelude::{
			// `AccountIdFor<T>` é equivalente a `<T as frame_system::Config>::AccountId`
			AccountIdFor,

			// `BlockNumberFor<T>` é equivalente a `<T as frame_system::Config>::BlockNumber`
			BlockNumberFor,

			// `OriginFor<T>` é equivalente a `<T as frame_system::Config>::Origin`
			OriginFor,
		},
	};

	// The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
	// (`Call`s) in this pallet.
	#[pallet::pallet]
	pub struct Pallet<T, I = ()>(_);

	/// Configuração do pallet-template
	#[pallet::config]
	pub trait Config<I: 'static = ()>: pallet_timestamp::Config + frame_system::Config {
		/// The overarching runtime event type.
		#[allow(deprecated)]
		type RuntimeEvent: From<Event<Self, I>>
			+ IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Tipo que identifica unicamente um NFT.
		type TokenId: Parameter
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
			+ One;

		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;
	}

	/// Valor é um número armazenado nesse pallet.
	///
	/// Nesse template esta sendo declarado um Item no storage chamado `NextToken`
	/// que armazena um valor do tipo `Config::TokenId`.
	///
	/// Aprenda mais sobre storage aqui: <https://docs.substrate.io/build/runtime-storage/>
	#[pallet::storage]
	pub type NextToken<T: Config<I>, I: 'static = ()> = StorageValue<_, T::TokenId>;

	// Os tipos `StorageMap`, `StorageDoubleMap` e `StorageNMap` precisam de uma
	// função para mapear uma chave para uma storage-key chamada Storage Hasher,
	// a tabela a seguir descreve quando utilizar cada storage hasher.
	//
	//        FUNÇÃO(X)       |                  QUANDO USAR
	// -----------------------|------------------------------------------------
	//                        |  A chave `X` é uniformemente distribuida
	//        Identity        |  Um adversário não consegue escolher a chave `X`.
	//                        |  Ex: A chave é igual ao blake2 do valor armazenado.
	// -----------------------|------------------------------------------------
	//         Twox128        |  A chave `X` não é uniformemente distribuida.
	//         Twox64         |  Um adversário não consegue escolher a chave `X`.
	//                        |  Ex: A entrada é um numero sequencial, um texto, etc.
	// -----------------------|------------------------------------------------
	//        Blake2_256      |  Um adversário consegue escolher a chave `X`
	//        Blake2_128      |  Ex: Mais segura, na duvída sempre utilize essas.
	// ------------------------------------------------------------------------
	//
	// # Quando utilizar as varições com `*Concat` como `Blake2_128Concat`, etc.
	// DICA: Utilize variações com `*_CONCAT` quando a chave é pequena e precisa
	// ser listada on-chain ou off-chain, imagine ela como um campo indexado no
	// banco de dados.
	//
	/// Metadata of a collection.
	#[pallet::storage]
	pub type Tokens<T: Config<I>, I: 'static = ()> = StorageMap<
		_,
		Blake2_128Concat, // Função que recebe a CHAVE e retorna o sufixo da storage key.
		TokenIdOf<T, I>,  // tipo da CHAVE
		AccountIdFor<T>,  // tipo do VALOR
		OptionQuery,      // Define o que é retornado ao ler uma CHAVE que não existe.
	>;

	#[pallet::storage]
	pub type BlockCounter<T: Config<I>, I: 'static = ()> = StorageValue<_, u32>;

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
	pub enum Event<T: Config<I>, I: 'static = ()> {
		/// A user has successfully set a new value.
		ValorArmazenado {
			/// The new value set.
			valor: TokenIdOf<T, I>,
			/// The account who set the new value.
			conta: AccountIdFor<T>,
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
	pub enum Error<T, I = ()> {
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

	#[pallet::hooks]
	impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {
		/// A dummy `on_initialize` to return the amount of weight that `on_finalize` requires to
		/// execute.
		fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
			// Como acessar a configuração de outro pallet
			let _horario_default = <T as pallet_timestamp::Config>::Moment::default();

			// Como acessar métodos de outro pallet
			let _horario_atual = pallet_timestamp::Pallet::<T>::get();

			// Incrementa o bloco atual.
			let block_counter = BlockCounter::<T, I>::get().unwrap_or(0).saturating_add(1);
			BlockCounter::<T, I>::set(Some(block_counter));

			// Retorna o quanto de `weight` foi consumido.
			<<T as Config<I>>::WeightInfo as WeightInfo>::on_initialize()
		}
	}

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		/// An example dispatchable that takes a single u32 value as a parameter, writes the value
		/// to storage and emits an event.
		///
		/// It checks that the _origin_ for this call is _Signed_ and returns a dispatch
		/// error if it isn't. Learn more about origins here: <https://docs.substrate.io/build/origins/>
		#[pallet::call_index(0)]
		#[pallet::weight(<<T as Config<I>>::WeightInfo as WeightInfo>::alterar_valor())]
		pub fn alterar_valor(origin: OriginFor<T>, valor: TokenIdOf<T, I>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let conta = ensure_signed(origin)?;

			// Update storage.
			NextToken::<T, I>::put(valor);

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
		#[pallet::weight(<<T as Config<I>>::WeightInfo as WeightInfo>::cause_error())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match NextToken::<T, I>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T, I>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage. This will cause an error in the event
					// of overflow.
					let one = <TokenIdOf<T, I> as One>::one();
					let new = old.checked_add(&one).ok_or(Error::<T, I>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					NextToken::<T, I>::put(new);
					Ok(())
				},
			}
		}

		/// Método que incrementa o `Valor` em uma unidade.
		/// pode retornar um erro em dois casos:
		/// - O valor não foi definido
		/// - Overflow
		#[pallet::call_index(2)]
		#[pallet::weight(<<T as Config<I>>::WeightInfo as WeightInfo>::incrementar())]
		pub fn incrementar(origin: OriginFor<T>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// Verifica se essa `extrinsic` foi assinada:
			// - Se foi assinada, retorna quem assinou.
			// - Se não foi assinada, retorna um erro.
			let conta = ensure_signed(origin)?;

			// Le o `Valor` que esta armazenado no storage.
			let valor = match NextToken::<T, I>::get() {
				None => {
					// Valor não existe, retorne um erro, isso irá reverter
					// essa transação.
					return Err(Error::<T, I>::NoneValue.into())
				},
				Some(old) => {
					// Incrementa o `Valor` em 1 unidade.
					// Um erro será retornado em caso de overflow.
					let one = <TokenIdOf<T, I> as One>::one();
					let new = old.checked_add(&one).ok_or(Error::<T, I>::StorageOverflow)?;

					// Atualiza o valor armazenado no storage.
					NextToken::<T, I>::put(new);
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
		#[pallet::weight(<<T as Config<I>>::WeightInfo as WeightInfo>::mint(1))]
		pub fn mint(origin: OriginFor<T>, token_id: TokenIdOf<T, I>) -> DispatchResult {
			let conta = ensure_signed(origin)?;

			if !Tokens::<T, I>::contains_key(&token_id) {
				Tokens::<T, I>::insert(token_id, conta);
			} else {
				return Err(Error::<T, I>::TokenJaExiste.into());
			}

			Ok(())
		}
	}
}
