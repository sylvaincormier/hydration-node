// Copyright (C) 2020-2022  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Bonds pallet
//!
//! ## Overview
//!
//! This pallet provides functionality to issue fungible bonds.
//! Once the bonds are mature, they can be redeemed for the underlying asset.
//! The pallet uses `Time` trait to get the timestamp of the last block, provided by the timestamp pallet.
//!
//! ## Assumptions
//!
//! * When issuing new bonds, new nameless asset of the `AssetType::Bond` is registered for the bonds.
//! * New amount of bonds is issued when the underlying asset and maturity matches already registered bonds.
//! * It's possible to create multiple bonds for the same underlying asset.
//! * Bonds can be issued for all available asset types.
//! * The existential deposit of the bonds is the same as of the underlying asset.
//! * A user receives the same amount of bonds as the amount of the underlying asset he provided, minus the protocol fee.
//! * Maturity of bonds is represented using the Unix time in milliseconds.
//! * Underlying assets are stored in the pallet account until redeemed.
//! * Protocol fee is applied to the amount of the underlying asset and transferred to the fee receiver.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	ensure,
	pallet_prelude::{DispatchResult, Get},
	sp_runtime::{
		traits::{AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedSub, Zero},
		ArithmeticError, DispatchError, Permill,
	},
	traits::Time,
	PalletId,
};
use frame_system::{ensure_signed, pallet_prelude::OriginFor};
use scale_info::TypeInfo;
use sp_core::MaxEncodedLen;

use hydradx_traits::{AssetKind, CreateRegistry};
use orml_traits::{GetByKey, MultiCurrency};
use primitives::Moment;

#[cfg(test)]
mod tests;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarks;

pub mod weights;

pub use pallet::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use codec::HasCompact;
	use frame_support::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(crate) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Identifier for the class of asset.
		type AssetId: Member
			+ Parameter
			+ Default
			+ Copy
			+ HasCompact
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TypeInfo;

		/// Balance type.
		type Balance: Parameter
			+ Member
			+ Copy
			+ PartialOrd
			+ MaybeSerializeDeserialize
			+ Default
			+ CheckedAdd
			+ CheckedSub
			+ AtLeast32BitUnsigned
			+ MaxEncodedLen
			+ From<u128>;

		/// Multi currency mechanism.
		type Currency: MultiCurrency<Self::AccountId, CurrencyId = Self::AssetId, Balance = Self::Balance>;

		/// Asset Registry mechanism - used to register bonds in the asset registry.
		type AssetRegistry: CreateRegistry<Self::AssetId, Self::Balance, Error = DispatchError>;

		/// Provider for existential deposits of assets.
		type ExistentialDeposits: GetByKey<Self::AssetId, Self::Balance>;

		/// Provider for the current timestamp.
		type TimestampProvider: Time<Moment = Moment>;

		/// The pallet id, used for deriving its sovereign account ID.
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Min time for maturity.
		#[pallet::constant]
		type MinMaturity: Get<Moment>;

		/// Protocol fee.
		#[pallet::constant]
		type ProtocolFee: Get<Permill>;

		/// Protocol fee receiver.
		#[pallet::constant]
		type FeeReceiver: Get<Self::AccountId>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	/// Registered bond ids. Fully redeemed bonds are removed from the storage.
	/// Maps (underlying asset ID, maturity) -> bond ID
	#[pallet::getter(fn bond_id)]
	pub(super) type BondIds<T: Config> = StorageMap<_, Blake2_128Concat, (T::AssetId, Moment), T::AssetId>;

	#[pallet::storage]
	/// Registered bonds. Fully redeemed bonds are removed from the storage.
	/// Maps bond ID -> (underlying asset ID, maturity)
	#[pallet::getter(fn bonds)]
	pub(super) type Bonds<T: Config> = StorageMap<_, Blake2_128Concat, T::AssetId, (T::AssetId, Moment)>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A bond asset was registered
		BondTokenCreated {
			issuer: T::AccountId,
			asset_id: T::AssetId,
			bond_id: T::AssetId,
			maturity: Moment,
		},
		/// New bond were issued
		BondsIssued {
			issuer: T::AccountId,
			bond_id: T::AssetId,
			amount: T::Balance,
			fee: T::Balance,
		},
		/// Bonds were redeemed
		BondsRedeemed {
			who: T::AccountId,
			bond_id: T::AssetId,
			amount: T::Balance,
		},
	}

	#[pallet::error]
	#[cfg_attr(test, derive(PartialEq, Eq))]
	pub enum Error<T> {
		/// Bond not registered
		BondNotRegistered,
		/// Bond is not mature
		BondNotMature,
		/// Maturity not long enough
		InvalidMaturity,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Issue new fungible bonds.
		/// New asset id is registered and assigned to the bonds.
		/// The number of bonds the issuer receives is 1:1 to the `amount` of the underlying asset
		/// minus the protocol fee.
		/// The bond asset is registered with the empty string for the asset name,
		/// and with the same existential deposit as of the underlying asset.
		/// Bonds can be redeemed for the underlying asset once mature.
		/// Protocol fee is applied to the amount, and transferred to `T::FeeReceiver`.
		/// When issuing new bonds with the underlying asset and maturity that matches existing bonds,
		/// new amount of these existing bonds is issued, instead of registering new bonds.
		/// It's possible to issue new bonds for bonds that are already mature.
		///
		/// Parameters:
		/// - `origin`: issuer of new bonds
		/// - `asset_id`: underlying asset id
		/// - `amount`: the amount of the underlying asset
		/// - `maturity`: Unix time in milliseconds, when the bonds will be mature. Needs to be set
		/// more than `T::MinMaturity` from now.
		///
		/// Emits `BondTokenCreated` event when successful and new bonds were registered.
		/// Emits `BondsIssued` event when successful.
		///
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::issue())]
		pub fn issue(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			amount: T::Balance,
			maturity: Moment,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			BondIds::<T>::try_mutate_exists((asset_id, maturity), |maybe_bond| -> DispatchResult {
				let fee = T::ProtocolFee::get().mul_ceil(amount); // TODO: check
				let amount_without_fee = amount.checked_sub(&fee).ok_or(ArithmeticError::Overflow)?;
				let pallet_account = Self::pallet_account_id();

				let bond_id = match maybe_bond {
					Some(bond_id) => {
						// bonds exist

						*bond_id
					}
					None => {
						// register new bonds
						let time_diff = maturity
							.checked_sub(T::TimestampProvider::now())
							.ok_or(ArithmeticError::Overflow)?;
						ensure!(time_diff >= T::MinMaturity::get(), Error::<T>::InvalidMaturity);

						let ed = T::ExistentialDeposits::get(&asset_id);

						// not covered in the tests.
						let bond_id = T::AssetRegistry::create_asset(&[], AssetKind::Bond, ed)?;

						*maybe_bond = Some(bond_id);
						Bonds::<T>::insert(bond_id, (asset_id, maturity));

						Self::deposit_event(Event::BondTokenCreated {
							issuer: who.clone(),
							asset_id,
							bond_id,
							maturity,
						});

						bond_id
					}
				};

				T::Currency::transfer(asset_id, &who.clone(), &pallet_account, amount_without_fee)?;
				T::Currency::transfer(asset_id, &who.clone(), &T::FeeReceiver::get(), fee)?;
				T::Currency::deposit(bond_id, &who, amount_without_fee)?;

				Self::deposit_event(Event::BondsIssued {
					issuer: who.clone(),
					bond_id,
					amount: amount_without_fee,
					fee,
				});

				Ok(())
			})
		}

		/// Redeem bonds for the underlying asset.
		/// The amount of the underlying asset the `origin` receives is 1:1 to the `amount` of the bonds.
		/// Anyone who holds the bonds is able to redeem them.
		/// The bonds can be partially redeemed.
		///
		/// Parameters:
		/// - `origin`: account id
		/// - `asset_id`: bond asset id
		/// - `amount`: the amount of the bonds to redeem for the underlying asset
		///
		/// Emits `BondsRedeemed` event when successful.
		///
		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::redeem())]
		pub fn redeem(origin: OriginFor<T>, bond_id: T::AssetId, amount: T::Balance) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let (underlying_asset_id, maturity) = Self::bonds(bond_id).ok_or(Error::<T>::BondNotRegistered)?;

			let now = T::TimestampProvider::now();
			ensure!(now >= maturity, Error::<T>::BondNotMature);

			T::Currency::withdraw(bond_id, &who, amount)?;

			let pallet_account = Self::pallet_account_id();
			T::Currency::transfer(underlying_asset_id, &pallet_account, &who, amount)?;

			// if there are no bonds left, remove the bonds entry from the storage
			let bond_balance = T::Currency::total_issuance(bond_id);
			if bond_balance.is_zero() {
				BondIds::<T>::remove((underlying_asset_id, maturity));
				Bonds::<T>::remove(bond_id);
			}

			Self::deposit_event(Event::BondsRedeemed { who, bond_id, amount });

			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// The account ID of the bonds pallet.
	///
	/// This actually does computation. If you need to keep using it, then make sure you cache the
	/// value and only call this once.
	pub fn pallet_account_id() -> T::AccountId {
		T::PalletId::get().into_account_truncating()
	}
}
