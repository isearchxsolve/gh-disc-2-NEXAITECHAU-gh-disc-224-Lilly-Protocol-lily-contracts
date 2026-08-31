// File: pallets/nex-agent/src/lib.rs
//! NEX Agent pallet with single-key config view functions

use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::Get};
use sp_runtime::SaturatedConversion;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        #[pallet::constant]
        type MaxKeys: Get<u32>;
        
        /// Default config value for all keys
        type DefaultConfig: Parameter + Default;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn config)]
    pub type Configs<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::DefaultConfig, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    /// Single-key config view functions for efficient off-chain queries
    #[pallet::extra_constants]
    impl<T: Config> Pallet<T> {
        /// Check if a key exists in storage
        pub fn contains_key(account: &T::AccountId) -> bool {
            <Configs<T>>::contains_key(account)
        }

        /// Get config value for a key, returning default if not present
        pub fn get(account: &T::AccountId) -> T::DefaultConfig {
            <Configs<T>>::get(account)
        }

        /// Check if stored value equals the default value
        pub fn is_default(account: &T::AccountId) -> bool {
            <Configs<T>>::get(account) == T::DefaultConfig::default()
        }

        /// Get count of configured keys (non-default entries)
        pub fn configured_count() -> u32 {
            <Configs<T>>::iter()
                .filter(|(_, v)| *v != T::DefaultConfig::default())
                .count() as u32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{assert_ok, construct_runtime, parameter_types};
    use sp_core::H256;
    use sp_runtime::BuildStorage;

    parameter_types! {
        pub const MaxKeys: u32 = 100;
        pub const DefaultConfig: u32 = 0;
    }

    construct_runtime!(
        pub enum Test where
            Block = Block,
            NodeBlock = Block,
            UncheckedExtrinsic = UncheckedExtrinsic,
        {
            System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
            NexAgent: pallet::{Pallet, Storage},
        }
    );

    impl frame_system::Config for Test {
        type BaseCallFilter = frame_support::traits::Everything;
        type BlockWeights = ();
        type BlockLength = ();
        type DbWeight = ();
        type RuntimeOrigin = RuntimeOrigin;
        type RuntimeCall = RuntimeCall;
        type Index = u64;
        type BlockNumber = u64;
        type Hash = H256;
        type Hashing = sp_runtime::traits::BlakeTwo256;
        type AccountId = u64;
        type Lookup = frame_support::traits::IdentityLookup<Self::AccountId>;
        type Header = sp_runtime::generic::Header<u64, sp_runtime::traits::BlakeTwo256>;
        type RuntimeEvent = RuntimeEvent;
        type BlockHashCount = frame_support::traits::ConstU64<256>;
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = ();
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
        type SS58Prefix = ();
        type OnSetCode = ();
        type MaxConsumers = frame_support::traits::ConstU32<16>;
    }

    impl Config for Test {
        type MaxKeys = MaxKeys;
        type DefaultConfig = DefaultConfig;
    }

    #[test]
    fn single_key_view_functions_work() {
        let account: u64 = 123;
        
        // Initially empty
        assert!(!Pallet::<Test>::contains_key(&account));
        assert_eq!(Pallet::<Test>::get(&account), 0);
        assert!(Pallet::<Test>::is_default(&account));

        // Set value
        <Configs<Test>>::insert(account, 42);
        
        assert!(Pallet::<Test>::contains_key(&account));
        assert_eq!(Pallet::<Test>::get(&account), 42);
        assert!(!Pallet::<Test>::is_default(&account));

        // Verify count
        assert_eq!(Pallet::<Test>::configured_count(), 1);
    }
}