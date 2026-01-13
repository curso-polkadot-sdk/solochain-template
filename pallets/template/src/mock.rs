use crate as pallet_template;
use frame_support::{derive_impl, traits::ConstU64};
use sp_runtime::BuildStorage;

type Block = frame_system::mocking::MockBlock<Test>;

#[frame_support::runtime]
mod runtime {
	// The main runtime
	#[runtime::runtime]
	// Runtime Types to be generated
	#[runtime::derive(
		RuntimeCall,
		RuntimeEvent,
		RuntimeError,
		RuntimeOrigin,
		RuntimeFreezeReason,
		RuntimeHoldReason,
		RuntimeSlashReason,
		RuntimeLockId,
		RuntimeTask,
		RuntimeViewFunction
	)]
	pub struct Test;

	#[runtime::pallet_index(0)]
	pub type System = frame_system::Pallet<Test>;

	#[runtime::pallet_index(1)]
	pub type Timestamp = pallet_timestamp::Pallet<Test>;

	#[runtime::pallet_index(2)]
	pub type Balances = pallet_balances::Pallet<Test>;

	#[runtime::pallet_index(3)]
	pub type Template = pallet_template::Pallet<Test>;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
	type AccountData = pallet_balances::AccountData<u64>;
}

// Armazena balanços de contas.
#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Test {
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Self>;
}

// Armazena o horário que o bloco atual foi produzido
// O tempo é monotônico e medido em milisegundos
#[derive_impl(pallet_timestamp::config_preludes::TestDefaultConfig)]
impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type MinimumPeriod = ConstU64<0>;
	type WeightInfo = pallet_timestamp::weights::SubstrateWeight<Self>;
}

impl pallet_template::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type TokenId = u32;
	type WeightInfo = pallet_template::weights::SubstrateWeight<Self>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
