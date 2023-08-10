#![cfg_attr(not(feature = "std"), no_std)]

use codec::FullCodec;
use frame_election_provider_support::{ScoreProvider, SortedListProvider};
use sp_runtime::traits::{AtLeast32BitUnsigned, Bounded};
use sp_std::prelude::*;

pub use pallet::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Basic information about a collation candidate.
	#[derive(
		PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, scale_info::TypeInfo, MaxEncodedLen,
	)]
	pub struct CandidateInfo<AccountId, Score> {
		/// Account identifier.
		pub who: AccountId,
		/// Reserved deposit.
		pub deposit: Score,
	}

	#[pallet::pallet]
	pub struct Pallet<T, I = ()>(_);

	#[pallet::storage]
	#[pallet::getter(fn candidate_count)]
	pub type List<T: Config<I>, I: 'static = ()> = StorageValue<
		_,
		BoundedVec<CandidateInfo<T::AccountId, T::Score>, T::MaxEntries>,
		ValueQuery,
	>;

	#[pallet::config]
	pub trait Config<I: 'static = ()>: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self, I>>
			+ IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Weight information for extrinsics in this pallet.
		// type WeightInfo: weights::WeightInfo;

		/// Something that provides the scores of ids.
		type ScoreProvider: ScoreProvider<Self::AccountId, Score = Self::Score>;

		/// Maximum number of entries.
		type MaxEntries: Get<u32>;

		/// The type used to dictate a node position relative to other nodes.
		type Score: Clone
			+ Default
			+ PartialEq
			+ Eq
			+ Ord
			+ PartialOrd
			+ sp_std::fmt::Debug
			+ Copy
			+ AtLeast32BitUnsigned
			+ Bounded
			+ TypeInfo
			+ FullCodec
			+ MaxEncodedLen;
	}

	#[pallet::event]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		/// Updated the score of some account to the given amount.
		ScoreUpdated { who: T::AccountId, new_score: T::Score },
	}

	#[pallet::error]
	#[cfg_attr(test, derive(PartialEq))]
	pub enum Error<T, I = ()> {
		/// A error in the list interface implementation.
		List,
	}

	#[pallet::hooks]
	impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {
		#[cfg(feature = "try-runtime")]
		fn try_state(_: BlockNumberFor<T>) -> Result<(), TryRuntimeError> {
			<Self as SortedListProvider<T::AccountId>>::try_state()
		}
	}
}

impl<T: Config<I>, I: 'static> SortedListProvider<T::AccountId> for Pallet<T, I> {
	type Error = Error<T, I>;
	type Score = T::Score;

	fn iter() -> Box<dyn Iterator<Item = T::AccountId>> {
		let binding = <List<T, I>>::get();
		Box::new(
			binding
				.iter()
				.map(|candidate_info| candidate_info.who.clone())
				.collect::<Vec<_>>()
				.into_iter(),
		)
	}

	fn iter_from(
		start: &T::AccountId,
	) -> Result<Box<dyn Iterator<Item = T::AccountId>>, Self::Error> {
		let binding = <List<T, I>>::get();
		let iter = Box::new(
			binding
				.iter()
				.map(|candidate_info| candidate_info.who.clone())
				.skip_while(|who| who != start)
				.skip(1),
		);
		Ok(Box::new(iter.collect::<Vec<_>>().into_iter()))
	}

	fn count() -> u32 {
		<List<T, I>>::decode_len().unwrap_or_default().try_into().unwrap_or_default()
	}

	fn contains(id: &T::AccountId) -> bool {
		<List<T, I>>::get().iter().any(|candidate_info| &candidate_info.who == id)
	}

	fn on_insert(id: T::AccountId, score: T::Score) -> Result<(), Error<T, I>> {
		<List<T, I>>::try_mutate(|list| {
			let idx = list.partition_point(|candidate_info| candidate_info.deposit >= score);
			list.try_insert(idx, CandidateInfo { who: id, deposit: score })
				.map_err(|_| Error::List)?;
			Ok(())
		})
	}

	fn get_score(id: &T::AccountId) -> Result<T::Score, Error<T, I>> {
		<List<T, I>>::get()
			.iter()
			.find(|candidate_info| candidate_info.who == *id)
			.map(|candidate_info| candidate_info.deposit)
			.ok_or_else(|| Error::List)
	}

	fn on_update(id: &T::AccountId, new_score: T::Score) -> Result<(), Error<T, I>> {
		<List<T, I>>::try_mutate(|list| {
			let mut idx = list
				.iter()
				.position(|candiadte_info| candiadte_info.who == *id)
				.ok_or_else(|| Error::List)?;
			let increase = list[idx].deposit < new_score;
			list[idx].deposit = new_score;

			if increase && idx < list.len() {
				idx += 1;
				while idx < list.len() && list[idx].deposit < new_score {
					list.as_mut().swap(idx - 1, idx);
					idx += 1;
				}
			} else {
				while idx > 0 && list[idx].deposit >= new_score {
					list.as_mut().swap(idx - 1, idx);
					idx -= 1;
				}
			}
			Ok(())
		})
	}

	fn on_remove(id: &T::AccountId) -> Result<(), Error<T, I>> {
		<List<T, I>>::try_mutate(|list| {
			let idx = list
				.iter()
				.position(|candiadte_info| candiadte_info.who == *id)
				.ok_or_else(|| Error::List)?;
			list.remove(idx);
			Ok(())
		})
	}

	fn unsafe_regenerate(
		all: impl IntoIterator<Item = T::AccountId>,
		score_of: Box<dyn Fn(&T::AccountId) -> T::Score>,
	) -> u32 {
		// NOTE: This call is unsafe for the same reason as SortedListProvider::unsafe_regenerate.
		// I.e. because it can lead to many storage accesses.
		// So it is ok to call it as caller must ensure the conditions.
		Self::unsafe_clear();
		let mut new_list = vec![];
		for id in all.into_iter() {
			let candidate_info = CandidateInfo { who: id.clone(), deposit: score_of(&id) };
			new_list.push(candidate_info);
		}
		new_list.sort_by_key(|candidate_info| candidate_info.deposit);
		<List<T, I>>::try_mutate(|list| {
			*list = new_list.try_into().map_err(|_| Error::List)?;
			Ok::<(), Error<T, I>>(())
		})
		.unwrap();
		<List<T, I>>::decode_len().unwrap_or_default().try_into().unwrap_or_default()
	}

	#[cfg(feature = "try-runtime")]
	fn try_state() -> Result<(), TryRuntimeError> {
		Self::do_try_state()
	}

	fn unsafe_clear() {
		// NOTE: This call is unsafe for the same reason as SortedListProvider::unsafe_clear.
		// I.e. because it can lead to many storage accesses.
		// So it is ok to call it as caller must ensure the conditions.
		<List<T, I>>::mutate(|list| {
			list.clear();
		});
	}

	frame_election_provider_support::runtime_benchmarks_enabled! {
		fn score_update_worst_case(who: &T::AccountId, is_increase: bool) -> Self::Score {
			let score = Self::get_score(who).unwrap();
			if is_increase {
				<List<T, I>>::get().iter().last().map(|candidate_info| candidate_info.deposit).unwrap_or_default() - score
			} else {
				score - <List<T, I>>::get().iter().next().map(|candidate_info| candidate_info.deposit).unwrap_or_default()
			}
		}
	}
}

impl<T: Config<I>, I: 'static> ScoreProvider<T::AccountId> for Pallet<T, I> {
	type Score = T::Score;

	fn score(who: &T::AccountId) -> Self::Score {
		<List<T, I>>::get()
			.iter()
			.find(|candidate_info| candidate_info.who == *who)
			.map(|candidate_info| candidate_info.deposit)
			.unwrap_or_default()
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn set_score_of(who: &T::AccountId, weight: Self::Score) {
		Self::on_update(who, weight).unwrap()
	}
}
