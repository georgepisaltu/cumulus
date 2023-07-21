#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use frame_support::{pallet_prelude::*, traits::Currency};
	use frame_system::pallet_prelude::*;

	use sp_std::prelude::*;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_bags_list::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type Currency: Currency<Self::AccountId>;

		#[pallet::constant]
		type MaximumSize: Get<u32>;
	}

	#[pallet::storage]
	pub(super) type CandidatesCount<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	pub(super) type CandidatesMap<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, BalanceOf<T>>;

	#[pallet::error]
	pub enum Error<T> {
		/// Each collectible must have a unique identifier
		DuplicateCollectible,
		/// An account can't exceed the `MaximumOwned` constant
		MaximumCollectiblesOwned,
		/// The total supply of collectibles can't exceed the u64 limit
		BoundsOverflow,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new collectible was successfully created
		CollectibleCreated { collectible: u8, owner: T::AccountId },
	}

	// Pallet internal functions
	impl<T: Config> Pallet<T> {
		// Generates and returns the unique_id and color
		fn gen_unique_id() -> u8 {
			0u8
		}

		// Function to mint a collectible
		pub fn mint(owner: &T::AccountId, id: u8) -> Result<bool, DispatchError> {
			Self::deposit_event(Event::CollectibleCreated {
				collectible: id,
				owner: owner.clone(),
			});
			Ok(id < 10)
		}
	}

	// Pallet callable functions
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new unique collectible.
		///
		/// The actual collectible creation is done in the `mint()` function.
		#[pallet::weight(0)]
		pub fn create_collectible(origin: OriginFor<T>) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let sender = ensure_signed(origin)?;

			// Generate the unique_id and color using a helper function
			let generated_id = Self::gen_unique_id();

			// Write new collectible to storage by calling helper function
			Self::mint(&sender, generated_id)?;

			Ok(())
		}
	}
}
