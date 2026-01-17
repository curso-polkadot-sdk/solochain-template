use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

/// Struct que representa as informações de um RSA Challenge.
#[derive(Debug, Decode, Encode, TypeInfo, MaxEncodedLen, PartialEq, Eq)]
pub struct ChallengeDetails<ACCOUNT, BALANCE, MOMENT> {
	/// Quem cria e paga o premio do challenge.
	pub owner: ACCOUNT,

	/// Valor do prêmio.
	pub prize: BALANCE,

	/// Data de término.
	pub end_date: MOMENT,
}
