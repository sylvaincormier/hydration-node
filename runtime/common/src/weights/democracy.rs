// This file is part of Hydra-node.

// Copyright (C) 2020-2021  Intergalactic, Limited (GIB).
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

//! Autogenerated weights for pallet_democracy
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-04-30, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// --chain=local
// --steps=5
// --repeat=20
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=.maintain/pallet-weight-template-no-back.hbs
// --pallet=pallet_democracy
// --output=democracy.rs
// --extrinsic=*
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

use pallet_democracy::weights::WeightInfo;

pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	fn propose() -> Weight {
		(53_468_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn second(s: u32) -> Weight {
		(34_429_000 as Weight) // Standard Error: 4_000
			.saturating_add((183_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn vote_new(r: u32) -> Weight {
		(41_810_000 as Weight) // Standard Error: 2_000
			.saturating_add((197_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn vote_existing(r: u32) -> Weight {
		(41_736_000 as Weight) // Standard Error: 3_000
			.saturating_add((200_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn emergency_cancel() -> Weight {
		(20_589_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn blacklist(p: u32) -> Weight {
		(48_500_000 as Weight) // Standard Error: 48_000
			.saturating_add((617_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn external_propose(v: u32) -> Weight {
		(9_935_000 as Weight) // Standard Error: 1_000
			.saturating_add((80_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn external_propose_majority() -> Weight {
		(2_086_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn external_propose_default() -> Weight {
		(1_984_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn fast_track() -> Weight {
		(20_973_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn veto_external(v: u32) -> Weight {
		(21_466_000 as Weight) // Standard Error: 1_000
			.saturating_add((100_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn cancel_proposal(p: u32) -> Weight {
		(48_281_000 as Weight) // Standard Error: 8_000
			.saturating_add((385_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn cancel_referendum() -> Weight {
		(13_026_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn cancel_queued(r: u32) -> Weight {
		(25_113_000 as Weight) // Standard Error: 4_000
			.saturating_add((1_813_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn on_initialize_base(r: u32) -> Weight {
		(3_860_000 as Weight) // Standard Error: 18_000
			.saturating_add((3_916_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn on_initialize_base_with_launch_period(r: u32) -> Weight {
		(10_189_000 as Weight) // Standard Error: 16_000
			.saturating_add((3_894_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn delegate(r: u32) -> Weight {
		(37_776_000 as Weight) // Standard Error: 24_000
			.saturating_add((5_068_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))
	}
	fn undelegate(r: u32) -> Weight {
		(19_177_000 as Weight) // Standard Error: 15_000
			.saturating_add((5_034_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))
	}
	fn clear_public_proposals() -> Weight {
		(2_178_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn note_preimage(b: u32) -> Weight {
		(24_989_000 as Weight) // Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn note_imminent_preimage(b: u32) -> Weight {
		(21_119_000 as Weight) // Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn reap_preimage(b: u32) -> Weight {
		(27_541_000 as Weight) // Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn unlock_remove(r: u32) -> Weight {
		(28_169_000 as Weight) // Standard Error: 6_000
			.saturating_add((59_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn unlock_set(r: u32) -> Weight {
		(26_010_000 as Weight) // Standard Error: 3_000
			.saturating_add((166_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn remove_vote(r: u32) -> Weight {
		(14_613_000 as Weight) // Standard Error: 7_000
			.saturating_add((123_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn remove_other_vote(r: u32) -> Weight {
		(15_066_000 as Weight) // Standard Error: 7_000
			.saturating_add((127_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}