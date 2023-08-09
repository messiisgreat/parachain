
//! Autogenerated weights for `pallet_enclave_bridge`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-09, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `caribe`, CPU: `12th Gen Intel(R) Core(TM) i7-1260P`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("integritee-rococo-local-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/integritee-collator
// benchmark
// pallet
// --chain=integritee-rococo-local-dev
// --steps=50
// --repeat=20
// --pallet=pallet_enclave_bridge
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./polkadot-parachains/integritee-runtime/src/weights/pallet_enclave_bridge.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_enclave_bridge`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_enclave_bridge::WeightInfo for WeightInfo<T> {
	fn invoke() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 15_696_000 picoseconds.
		Weight::from_parts(16_507_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: Teerex SovereignEnclaves (r:1 w:0)
	/// Proof Skipped: Teerex SovereignEnclaves (max_values: None, max_size: None, mode: Measured)
	/// Storage: EnclaveBridge ShardConfigRegistry (r:1 w:0)
	/// Proof Skipped: EnclaveBridge ShardConfigRegistry (max_values: None, max_size: None, mode: Measured)
	/// Storage: EnclaveBridge ShardStatus (r:1 w:1)
	/// Proof Skipped: EnclaveBridge ShardStatus (max_values: None, max_size: None, mode: Measured)
	fn confirm_processed_parentchain_block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `408`
		//  Estimated: `3873`
		// Minimum execution time: 36_270_000 picoseconds.
		Weight::from_parts(37_521_000, 0)
			.saturating_add(Weight::from_parts(0, 3873))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn shield_funds() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `3593`
		// Minimum execution time: 88_755_000 picoseconds.
		Weight::from_parts(90_452_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Teerex SovereignEnclaves (r:1 w:0)
	/// Proof Skipped: Teerex SovereignEnclaves (max_values: None, max_size: None, mode: Measured)
	/// Storage: EnclaveBridge ShardConfigRegistry (r:1 w:0)
	/// Proof Skipped: EnclaveBridge ShardConfigRegistry (max_values: None, max_size: None, mode: Measured)
	/// Storage: EnclaveBridge ShardStatus (r:1 w:1)
	/// Proof Skipped: EnclaveBridge ShardStatus (max_values: None, max_size: None, mode: Measured)
	/// Storage: EnclaveBridge ExecutedUnshieldCalls (r:1 w:1)
	/// Proof Skipped: EnclaveBridge ExecutedUnshieldCalls (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn unshield_funds() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `511`
		//  Estimated: `6196`
		// Minimum execution time: 120_600_000 picoseconds.
		Weight::from_parts(122_273_000, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Teerex SovereignEnclaves (r:1 w:0)
	/// Proof Skipped: Teerex SovereignEnclaves (max_values: None, max_size: None, mode: Measured)
	/// Storage: EnclaveBridge ShardStatus (r:1 w:1)
	/// Proof Skipped: EnclaveBridge ShardStatus (max_values: None, max_size: None, mode: Measured)
	/// Storage: System EventTopics (r:6 w:6)
	/// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
	/// The range of component `l` is `[0, 100]`.
	/// The range of component `t` is `[1, 5]`.
	fn publish_hash(_l: u32, t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `408`
		//  Estimated: `3873 + t * (2475 ±0)`
		// Minimum execution time: 24_021_000 picoseconds.
		Weight::from_parts(39_366_456, 0)
			.saturating_add(Weight::from_parts(0, 3873))
			// Standard Error: 33_262
			.saturating_add(Weight::from_parts(1_722_781, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(t.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(t.into())))
			.saturating_add(Weight::from_parts(0, 2475).saturating_mul(t.into()))
	}
	/// Storage: Teerex SovereignEnclaves (r:1 w:0)
	/// Proof Skipped: Teerex SovereignEnclaves (max_values: None, max_size: None, mode: Measured)
	/// Storage: EnclaveBridge ShardConfigRegistry (r:1 w:1)
	/// Proof Skipped: EnclaveBridge ShardConfigRegistry (max_values: None, max_size: None, mode: Measured)
	/// Storage: EnclaveBridge ShardStatus (r:1 w:1)
	/// Proof Skipped: EnclaveBridge ShardStatus (max_values: None, max_size: None, mode: Measured)
	fn update_shard_config() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `521`
		//  Estimated: `3986`
		// Minimum execution time: 27_492_000 picoseconds.
		Weight::from_parts(28_568_000, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}