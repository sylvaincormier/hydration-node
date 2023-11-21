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

//! Autogenerated weights for pallet_transaction_multi_payment
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-17, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --chain=dev
// --steps=5
// --repeat=20
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=.maintain/pallet-weight-template.hbs
// --pallet=pallet_transaction_multi_payment
// --output=paymenta.rs
// --extrinsic=*

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_transaction_multi_payment.
pub trait WeightInfo {
	fn add_currency() -> Weight;
	fn remove_currency() -> Weight;
	fn set_currency() -> Weight;
	fn get_oracle_price() -> Weight;
}

/// Weights for pallet_transaction_multi_payment using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:1)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	fn add_currency() -> Weight {
		// Minimum execution time: 19_690 nanoseconds.
		Weight::from_ref_time(20_630_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:1)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	fn remove_currency() -> Weight {
		// Minimum execution time: 20_010 nanoseconds.
		Weight::from_ref_time(20_400_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:0 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn set_currency() -> Weight {
		// Minimum execution time: 24_179 nanoseconds.
		Weight::from_ref_time(24_620_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Router Routes (r:1 w:0)
	// Proof: Router Routes (max_values: None, max_size: Some(90), added: 2565, mode: MaxEncodedLen)
	// Storage: EmaOracle Oracles (r:10 w:0)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	fn get_oracle_price() -> Weight {
		// Minimum execution time: 72_539 nanoseconds.
		Weight::from_ref_time(73_320_000 as u64).saturating_add(T::DbWeight::get().reads(11 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:1)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	fn add_currency() -> Weight {
		// Minimum execution time: 19_690 nanoseconds.
		Weight::from_ref_time(20_630_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:1)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	fn remove_currency() -> Weight {
		// Minimum execution time: 20_010 nanoseconds.
		Weight::from_ref_time(20_400_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:0 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn set_currency() -> Weight {
		// Minimum execution time: 24_179 nanoseconds.
		Weight::from_ref_time(24_620_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Router Routes (r:1 w:0)
	// Proof: Router Routes (max_values: None, max_size: Some(90), added: 2565, mode: MaxEncodedLen)
	// Storage: EmaOracle Oracles (r:10 w:0)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	fn get_oracle_price() -> Weight {
		// Minimum execution time: 72_539 nanoseconds.
		Weight::from_ref_time(73_320_000).saturating_add(RocksDbWeight::get().reads(11))
	}
}
