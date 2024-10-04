// This file is part of HydraDX.

// Copyright (C) 2020-2023  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


//! Autogenerated weights for `pallet_ema_oracle`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-09-10, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bench-bot`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --wasm-execution=compiled
// --pallet=pallet-ema-oracle
// --extrinsic=*
// --template=scripts/pallet-weight-template.hbs
// --output=./weights/pallet_ema_oracle.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_ema_oracle.
pub trait WeightInfo {
	fn add_oracle() -> Weight;
	fn remove_oracle() -> Weight;
	fn on_finalize_no_entry() -> Weight;
	fn on_finalize_multiple_tokens(b: u32) -> Weight;
	fn on_trade_multiple_tokens(b: u32) -> Weight;
	fn on_liquidity_changed_multiple_tokens(b: u32) -> Weight;
	fn get_entry() -> Weight;
}

/// Weights for `pallet_ema_oracle` using the HydraDX node and recommended hardware.
impl WeightInfo for () {
	/// Storage: `EmaOracle::WhitelistedAssets` (r:1 w:1)
	/// Proof: `EmaOracle::WhitelistedAssets` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	fn add_oracle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `860`
		//  Estimated: `2126`
		// Minimum execution time: 15_358_000 picoseconds.
		Weight::from_parts(15_612_000, 2126)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `EmaOracle::WhitelistedAssets` (r:1 w:1)
	/// Proof: `EmaOracle::WhitelistedAssets` (`max_values`: Some(1), `max_size`: Some(641), added: 1136, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:0 w:3)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	fn remove_oracle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `876`
		//  Estimated: `7406`
		// Minimum execution time: 32_501_000 picoseconds.
		Weight::from_parts(32_728_000, 7406)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: `EmaOracle::Accumulator` (r:1 w:0)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	fn on_finalize_no_entry() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `208`
		//  Estimated: `7406`
		// Minimum execution time: 2_187_000 picoseconds.
		Weight::from_parts(2_267_000, 7406)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:117 w:117)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 39]`.
	fn on_finalize_multiple_tokens(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `305 + b * (626 ±0)`
		//  Estimated: `7406 + b * (7956 ±0)`
		// Minimum execution time: 45_323_000 picoseconds.
		Weight::from_parts(10_030_058, 7406)
			// Standard Error: 15_380
			.saturating_add(Weight::from_parts(35_004_114, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().reads((3_u64).saturating_mul(b.into())))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((3_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 7956).saturating_mul(b.into()))
	}
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 39]`.
	fn on_trade_multiple_tokens(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `821 + b * (164 ±0)`
		//  Estimated: `7406`
		// Minimum execution time: 16_045_000 picoseconds.
		Weight::from_parts(16_698_666, 7406)
			// Standard Error: 2_712
			.saturating_add(Weight::from_parts(482_996, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// The range of component `b` is `[1, 39]`.
	fn on_liquidity_changed_multiple_tokens(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `821 + b * (164 ±0)`
		//  Estimated: `7406`
		// Minimum execution time: 16_293_000 picoseconds.
		Weight::from_parts(16_839_547, 7406)
			// Standard Error: 2_626
			.saturating_add(Weight::from_parts(478_879, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `EmaOracle::Oracles` (r:2 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	fn get_entry() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `638`
		//  Estimated: `6294`
		// Minimum execution time: 17_336_000 picoseconds.
		Weight::from_parts(17_680_000, 6294)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
}
