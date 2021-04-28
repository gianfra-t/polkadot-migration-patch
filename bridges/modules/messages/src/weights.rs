// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_bridge_messages
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-04-14, STEPS: [50, ], REPEAT: 20
//! LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled
//! CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/rialto-bridge-node
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_bridge_messages
// --extrinsic=*
// --execution=wasm
// --wasm-execution=Compiled
// --heap-pages=4096
// --output=./modules/messages/src/weights.rs
// --template=./.maintain/rialto-weight-template.hbs

#![allow(clippy::all)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_bridge_messages.
pub trait WeightInfo {
	fn send_minimal_message_worst_case() -> Weight;
	fn send_1_kb_message_worst_case() -> Weight;
	fn send_16_kb_message_worst_case() -> Weight;
	fn increase_message_fee() -> Weight;
	fn receive_single_message_proof() -> Weight;
	fn receive_two_messages_proof() -> Weight;
	fn receive_single_message_proof_with_outbound_lane_state() -> Weight;
	fn receive_single_message_proof_1_kb() -> Weight;
	fn receive_single_message_proof_16_kb() -> Weight;
	fn receive_delivery_proof_for_single_message() -> Weight;
	fn receive_delivery_proof_for_two_messages_by_single_relayer() -> Weight;
	fn receive_delivery_proof_for_two_messages_by_two_relayers() -> Weight;
	fn send_messages_of_various_lengths(i: u32) -> Weight;
	fn receive_multiple_messages_proof(i: u32) -> Weight;
	fn receive_message_proofs_with_extra_nodes(i: u32) -> Weight;
	fn receive_message_proofs_with_large_leaf(i: u32) -> Weight;
	fn receive_multiple_messages_proof_with_outbound_lane_state(i: u32) -> Weight;
	fn receive_delivery_proof_for_multiple_messages_by_single_relayer(i: u32) -> Weight;
	fn receive_delivery_proof_for_multiple_messages_by_multiple_relayers(i: u32) -> Weight;
}

/// Weights for pallet_bridge_messages using the Rialto node and recommended hardware.
pub struct RialtoWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for RialtoWeight<T> {
	fn send_minimal_message_worst_case() -> Weight {
		(149_497_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(12 as Weight))
	}
	fn send_1_kb_message_worst_case() -> Weight {
		(154_339_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(12 as Weight))
	}
	fn send_16_kb_message_worst_case() -> Weight {
		(200_066_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(12 as Weight))
	}
	fn increase_message_fee() -> Weight {
		(6_432_637_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn receive_single_message_proof() -> Weight {
		(141_671_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn receive_two_messages_proof() -> Weight {
		(247_393_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn receive_single_message_proof_with_outbound_lane_state() -> Weight {
		(159_312_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn receive_single_message_proof_1_kb() -> Weight {
		(167_935_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn receive_single_message_proof_16_kb() -> Weight {
		(449_846_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn receive_delivery_proof_for_single_message() -> Weight {
		(127_322_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn receive_delivery_proof_for_two_messages_by_single_relayer() -> Weight {
		(134_120_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn receive_delivery_proof_for_two_messages_by_two_relayers() -> Weight {
		(191_193_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn send_messages_of_various_lengths(i: u32) -> Weight {
		(115_699_000 as Weight)
			.saturating_add((3_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(12 as Weight))
	}
	fn receive_multiple_messages_proof(i: u32) -> Weight {
		(0 as Weight)
			.saturating_add((113_551_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn receive_message_proofs_with_extra_nodes(i: u32) -> Weight {
		(458_731_000 as Weight)
			.saturating_add((9_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn receive_message_proofs_with_large_leaf(i: u32) -> Weight {
		(82_314_000 as Weight)
			.saturating_add((7_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn receive_multiple_messages_proof_with_outbound_lane_state(i: u32) -> Weight {
		(16_766_000 as Weight)
			.saturating_add((115_533_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn receive_delivery_proof_for_multiple_messages_by_single_relayer(i: u32) -> Weight {
		(122_146_000 as Weight)
			.saturating_add((6_789_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn receive_delivery_proof_for_multiple_messages_by_multiple_relayers(i: u32) -> Weight {
		(155_671_000 as Weight)
			.saturating_add((63_020_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn send_minimal_message_worst_case() -> Weight {
		(149_497_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(12 as Weight))
	}
	fn send_1_kb_message_worst_case() -> Weight {
		(154_339_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(12 as Weight))
	}
	fn send_16_kb_message_worst_case() -> Weight {
		(200_066_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(12 as Weight))
	}
	fn increase_message_fee() -> Weight {
		(6_432_637_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn receive_single_message_proof() -> Weight {
		(141_671_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn receive_two_messages_proof() -> Weight {
		(247_393_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn receive_single_message_proof_with_outbound_lane_state() -> Weight {
		(159_312_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn receive_single_message_proof_1_kb() -> Weight {
		(167_935_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn receive_single_message_proof_16_kb() -> Weight {
		(449_846_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn receive_delivery_proof_for_single_message() -> Weight {
		(127_322_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn receive_delivery_proof_for_two_messages_by_single_relayer() -> Weight {
		(134_120_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn receive_delivery_proof_for_two_messages_by_two_relayers() -> Weight {
		(191_193_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn send_messages_of_various_lengths(i: u32) -> Weight {
		(115_699_000 as Weight)
			.saturating_add((3_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(12 as Weight))
	}
	fn receive_multiple_messages_proof(i: u32) -> Weight {
		(0 as Weight)
			.saturating_add((113_551_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn receive_message_proofs_with_extra_nodes(i: u32) -> Weight {
		(458_731_000 as Weight)
			.saturating_add((9_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn receive_message_proofs_with_large_leaf(i: u32) -> Weight {
		(82_314_000 as Weight)
			.saturating_add((7_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn receive_multiple_messages_proof_with_outbound_lane_state(i: u32) -> Weight {
		(16_766_000 as Weight)
			.saturating_add((115_533_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn receive_delivery_proof_for_multiple_messages_by_single_relayer(i: u32) -> Weight {
		(122_146_000 as Weight)
			.saturating_add((6_789_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn receive_delivery_proof_for_multiple_messages_by_multiple_relayers(i: u32) -> Weight {
		(155_671_000 as Weight)
			.saturating_add((63_020_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().reads((2 as Weight).saturating_mul(i as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
}