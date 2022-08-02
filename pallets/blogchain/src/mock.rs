#![cfg(test)]

use crate as pallet_blogchain;
use frame_support::parameter_types;
use frame_support::traits::{ConstU16,ConstU32, ConstU64};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};


type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage},
		BlogModule: pallet_blogchain::{Pallet, Call, Storage, Event<T>},
		SubstrateKitties: pallet_template::{Pallet, Call, Storage, Config<T>, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}
impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
	type AccountStore = System;
	type Balance = u64;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
}
impl pallet_randomness_collective_flip::Config for Test {}
parameter_types! {
	// One can owned at most 9,999 Kitties
	pub const MaxKittiesOwned: u32 = 9999;
	pub const MaxDogsOwned: u32 = 9999;
}
impl pallet_template::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type KittyRandomness = RandomnessCollectiveFlip;
	type MaxKittiesOwned = MaxKittiesOwned;
	type MaxDogsOwned = MaxDogsOwned;
	type DogRandomness = RandomnessCollectiveFlip;
}

parameter_types! {
	// One can owned at most 9,999 Kitties
	pub const MaxKittiesVotesOwned: u32 = 9999;
	pub const MaxDogsVotesOwned: u32 = 9999;
	pub const BlogPostMinBytes: u32 = 64;
	pub const BlogPostMaxBytes: u32 = 4096;
	pub const BlogPostCommentMinBytes: u32 = 64;
	pub const BlogPostCommentMaxBytes: u32 = 1024;
	pub const AssetId: u8 = 0;
	pub const MaxTotalVotes: u32 = 20;
}
impl pallet_blogchain::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type AssetId = AssetId;
	type MaxKittiesVotesOwned = MaxKittiesVotesOwned;
	type MaxDogsVotesOwned = MaxDogsVotesOwned;
	type MaxTotalVotes = MaxTotalVotes;
	type BlogPostMinBytes = BlogPostMinBytes;
	type BlogPostMaxBytes = BlogPostMaxBytes;
	type BlogPostCommentMinBytes = BlogPostCommentMinBytes;
	type BlogPostCommentMaxBytes = BlogPostCommentMaxBytes;
}
// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}