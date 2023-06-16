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

//! Autogenerated weights for pallet_scheduler
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-16, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --pallet=pallet-scheduler
// --execution=wasm
// --wasm-execution=compiled
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// scheduler.rs
// --template
// .maintain/pallet-weight-template-no-back.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

use pallet_scheduler::weights::WeightInfo;

/// Weights for pallet_scheduler using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	// Storage: Scheduler IncompleteSince (r:1 w:1)
	// Proof: Scheduler IncompleteSince (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn service_agendas_base() -> Weight {
		// Minimum execution time: 4_850 nanoseconds.
		Weight::from_ref_time(4_988_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 50]`.
	fn service_agenda_base(s: u32) -> Weight {
		// Minimum execution time: 4_273 nanoseconds.
		Weight::from_ref_time(6_472_216 as u64) // Standard Error: 12_401
			.saturating_add(Weight::from_ref_time(850_077 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn service_task_base() -> Weight {
		// Minimum execution time: 7_946 nanoseconds.
		Weight::from_ref_time(8_215_000 as u64)
	}
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Proof: Preimage PreimageFor (max_values: None, max_size: Some(4194344), added: 4196819, mode: Measured)
	// Storage: Preimage StatusFor (r:1 w:1)
	// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// The range of component `s` is `[128, 4194304]`.
	fn service_task_fetched(s: u32) -> Weight {
		// Minimum execution time: 26_328 nanoseconds.
		Weight::from_ref_time(26_604_000 as u64) // Standard Error: 7
			.saturating_add(Weight::from_ref_time(1_172 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Scheduler Lookup (r:0 w:1)
	// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn service_task_named() -> Weight {
		// Minimum execution time: 10_124 nanoseconds.
		Weight::from_ref_time(10_427_000 as u64).saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn service_task_periodic() -> Weight {
		// Minimum execution time: 7_995 nanoseconds.
		Weight::from_ref_time(8_327_000 as u64)
	}
	fn execute_dispatch_signed() -> Weight {
		// Minimum execution time: 4_466 nanoseconds.
		Weight::from_ref_time(4_639_000 as u64)
	}
	fn execute_dispatch_unsigned() -> Weight {
		// Minimum execution time: 4_385 nanoseconds.
		Weight::from_ref_time(4_517_000 as u64)
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 49]`.
	fn schedule(s: u32) -> Weight {
		// Minimum execution time: 17_649 nanoseconds.
		Weight::from_ref_time(19_813_429 as u64) // Standard Error: 10_431
			.saturating_add(Weight::from_ref_time(846_131 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	// Storage: Scheduler Lookup (r:0 w:1)
	// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// The range of component `s` is `[1, 50]`.
	fn cancel(s: u32) -> Weight {
		// Minimum execution time: 21_787 nanoseconds.
		Weight::from_ref_time(21_504_773 as u64) // Standard Error: 5_061
			.saturating_add(Weight::from_ref_time(1_428_516 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	// Storage: Scheduler Agenda (r:1 w:1)
	// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// The range of component `s` is `[0, 49]`.
	fn schedule_named(s: u32) -> Weight {
		// Minimum execution time: 21_329 nanoseconds.
		Weight::from_ref_time(24_866_801 as u64) // Standard Error: 18_480
			.saturating_add(Weight::from_ref_time(919_823 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	// Storage: Scheduler Agenda (r:1 w:1)
	// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// The range of component `s` is `[1, 50]`.
	fn cancel_named(s: u32) -> Weight {
		// Minimum execution time: 24_480 nanoseconds.
		Weight::from_ref_time(24_713_695 as u64) // Standard Error: 9_865
			.saturating_add(Weight::from_ref_time(1_466_560 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
