// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

#![cfg(feature = "runtime-benchmarks")]

use crate::{Call, Config, Pallet};
use frame_benchmarking::{benchmarks};
use frame_system::RawOrigin;
use frame_support::traits::OnInitialize;
use frame_support::storage::migration::put_storage_value;
use nimbus_primitives::NimbusId;
use sp_application_crypto::ByteArray;
benchmarks! {
	kick_off_authorship_validation {
		let pallet_prefix: &[u8] = b"AuthorInherent";
		let storage_item_prefix: &[u8] = b"AUthor";
		let nimbus_key = [1u8; 32];

		put_storage_value(
			pallet_prefix,
			storage_item_prefix,
			&[],
			NimbusId::from_slice(&nimbus_key),
		);
	}: _(RawOrigin::None)
}
