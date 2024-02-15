// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>


//! Autogenerated weights for pallet_services_payment
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `tomasz-XPS-15-9520`, CPU: `12th Gen Intel(R) Core(TM) i7-12700H`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/tanssi-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_services_payment
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template=./benchmarking/frame-weight-template.hbs
// --json-file
// raw.json
// --output
// weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_services_payment.
pub trait WeightInfo {
	fn purchase_credits() -> Weight;
	fn set_credits() -> Weight;
	fn set_given_free_credits() -> Weight;
	fn on_container_author_noted() -> Weight;
	fn set_refund_address() -> Weight;
}

/// Weights for pallet_services_payment using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `ServicesPayment::BlockProductionCredits` (r:1 w:1)
	/// Proof: `ServicesPayment::BlockProductionCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn purchase_credits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `187`
		//  Estimated: `3593`
		// Minimum execution time: 22_986_000 picoseconds.
		Weight::from_parts(23_491_000, 3593)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ServicesPayment::BlockProductionCredits` (r:0 w:1)
	/// Proof: `ServicesPayment::BlockProductionCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn set_credits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_609_000 picoseconds.
		Weight::from_parts(6_839_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ServicesPayment::GivenFreeCredits` (r:0 w:1)
	/// Proof: `ServicesPayment::GivenFreeCredits` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	fn set_given_free_credits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_541_000 picoseconds.
		Weight::from_parts(2_691_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}

	/// Storage: `ServicesPayment::BlockProductionCredits` (r:1 w:0)
	/// Proof: `ServicesPayment::BlockProductionCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn on_container_author_noted() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `224`
		//  Estimated: `3593`
		// Minimum execution time: 19_043_000 picoseconds.
		Weight::from_parts(19_499_000, 3593)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Registrar::RegistrarDeposit` (r:1 w:0)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ServicesPayment::RefundAddress` (r:0 w:1)
	/// Proof: `ServicesPayment::RefundAddress` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn set_refund_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `195`
		//  Estimated: `3660`
		// Minimum execution time: 12_734_000 picoseconds.
		Weight::from_parts(13_245_000, 3660)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `ServicesPayment::BlockProductionCredits` (r:1 w:1)
	/// Proof: `ServicesPayment::BlockProductionCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn purchase_credits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `187`
		//  Estimated: `3593`
		// Minimum execution time: 22_986_000 picoseconds.
		Weight::from_parts(23_491_000, 3593)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `ServicesPayment::BlockProductionCredits` (r:0 w:1)
	/// Proof: `ServicesPayment::BlockProductionCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn set_credits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_609_000 picoseconds.
		Weight::from_parts(6_839_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `ServicesPayment::GivenFreeCredits` (r:0 w:1)
	/// Proof: `ServicesPayment::GivenFreeCredits` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	fn set_given_free_credits() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_541_000 picoseconds.
		Weight::from_parts(2_691_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `ServicesPayment::BlockProductionCredits` (r:1 w:0)
	/// Proof: `ServicesPayment::BlockProductionCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn on_container_author_noted() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `224`
		//  Estimated: `3593`
		// Minimum execution time: 19_043_000 picoseconds.
		Weight::from_parts(19_499_000, 3593)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Registrar::RegistrarDeposit` (r:1 w:0)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ServicesPayment::RefundAddress` (r:0 w:1)
	/// Proof: `ServicesPayment::RefundAddress` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn set_refund_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `195`
		//  Estimated: `3660`
		// Minimum execution time: 12_734_000 picoseconds.
		Weight::from_parts(13_245_000, 3660)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
