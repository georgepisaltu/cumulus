// Copyright Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_session`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-31, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-ynta1nyy-project-238-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: ``, WASM-EXECUTION: `Compiled`, CHAIN: `Some("asset-hub-kusama-dev")`, DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot-parachain
// benchmark
// pallet
// --chain=asset-hub-kusama-dev
// --wasm-execution=compiled
// --pallet=pallet_session
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/assets/asset-hub-kusama/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_session`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_fixed::WeightInfo for WeightInfo<T> {
	fn note_author() -> Weight {
		Weight::from_parts(71_461_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	fn new_session(r: u32, c: u32) -> Weight {
		Weight::from_parts(0_u64, 0)
			// Standard Error: 1_010_000
			.saturating_add(Weight::from_parts(109_961_000_u64, 0).saturating_mul(r as u64))
			// Standard Error: 1_010_000
			.saturating_add(Weight::from_parts(151_952_000_u64, 0).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64.saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().reads(2_u64.saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().writes(2_u64.saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(2_u64.saturating_mul(c as u64)))
	}
	/// Storage: Session NextKeys (r:1 w:0)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: CollatorSelection collators (r:1 w:1)
	/// Proof: CollatorSelection collators (max_values: Some(1), max_size: Some(641), added:
	/// 1136, mode: MaxEncodedLen) Storage: CollatorSelection Candidates (r:1 w:1)
	/// Proof: CollatorSelection Candidates (max_values: Some(1), max_size: Some(4802), added: 5297,
	/// mode: MaxEncodedLen) Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode:
	/// MaxEncodedLen) The range of component `b` is `[1, 19]`.
	/// The range of component `c` is `[1, 99]`.
	fn add_collator(c: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `757 + b * (32 ±0) + c * (53 ±0)`
		//  Estimated: `6287 + b * (37 ±0) + c * (53 ±0)`
		// Minimum execution time: 52_720_000 picoseconds.
		Weight::from_parts(56_102_459, 0)
			.saturating_add(Weight::from_parts(0, 6287))
			// Standard Error: 12_957
			// Standard Error: 2_456
			.saturating_add(Weight::from_parts(128_528, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 53).saturating_mul(c.into()))
	}
	/// Storage: CollatorSelection collators (r:1 w:1)
	/// Proof: CollatorSelection collators (max_values: Some(1), max_size: Some(3202), added:
	/// 3697, mode: MaxEncodedLen) The range of component `b` is `[1, 100]`.
	fn remove_collator(b: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `119 + b * (32 ±0)`
		//  Estimated: `4687`
		// Minimum execution time: 183_054_000 picoseconds.
		Weight::from_parts(197_205_427, 0)
			.saturating_add(Weight::from_parts(0, 4687))
			// Standard Error: 13_533
			.saturating_add(Weight::from_parts(376_231, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
