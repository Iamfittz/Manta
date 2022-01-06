// Copyright 2020-2022 Manta Network.
// This file is part of Manta.

// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_timestamp
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
<<<<<<< HEAD
//! DATE: 2022-01-05, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
=======
//! DATE: 2021-12-27, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
>>>>>>> manta
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("calamari-dev"), DB CACHE: 128

// Executed Command:
// manta
// benchmark
// --chain=calamari-dev
// --pallet=pallet_timestamp
// --extrinsic=*
// --execution=Wasm
// --wasm-execution=Compiled
// --heap-pages=4096
// --repeat=20
// --steps=50
// --template=.github/resources/frame-weight-template.hbs
// --output=pallet_timestamp.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_timestamp.
pub trait WeightInfo {
	fn set() -> Weight;
	fn on_finalize() -> Weight;
}

/// Weights for pallet_timestamp using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_timestamp::WeightInfo for SubstrateWeight<T> {
	// Storage: Timestamp Now (r:1 w:1)
	fn set() -> Weight {
<<<<<<< HEAD
		(9_760_000 as Weight)
=======
		(10_030_000 as Weight)
>>>>>>> manta
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn on_finalize() -> Weight {
<<<<<<< HEAD
		(5_521_000 as Weight)
=======
		(5_631_000 as Weight)
>>>>>>> manta
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Timestamp Now (r:1 w:1)
	fn set() -> Weight {
<<<<<<< HEAD
		(9_760_000 as Weight)
=======
		(10_030_000 as Weight)
>>>>>>> manta
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn on_finalize() -> Weight {
<<<<<<< HEAD
		(5_521_000 as Weight)
=======
		(5_631_000 as Weight)
>>>>>>> manta
	}
}
