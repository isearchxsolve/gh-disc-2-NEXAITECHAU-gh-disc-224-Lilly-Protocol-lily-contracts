// In pallet/src/lib.rs

use frame_support::{pallet_prelude::*, traits::Get};
use frame_system::pallet_prelude::*;

#[pallet::config]
pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    /// Config storage type
    type ConfigData: Parameter + Member + Default;
}

#[pallet::pallet]
pub struct Pallet<T>(_);

#[pallet::storage]
#[pallet::getter(fn config)]
pub type Config<T: Config> = StorageValue<_, T::ConfigData, ValueQuery>;

#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Get the current config value (returns None if not set)
    #[pallet::weight(10_000)]
    pub fn get_config_value(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
        ensure_signed(origin)?;
        let config = <Config<T>>::get();
        frame_support::debug::RuntimeDebug::println!("Config value: {:?}", config);
        Ok(().into())
    }

    /// Check if config exists (returns true if set, false otherwise)
    #[pallet::weight(10_000)]
    pub fn get_config_exists(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
        ensure_signed(origin)?;
        let exists = <Config<T>>::exists();
        frame_support::debug::RuntimeDebug::println!("Config exists: {}", exists);
        Ok(().into())
    }
}