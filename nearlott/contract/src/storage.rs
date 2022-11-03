use crate::*;
use near_contract_standards::storage_management::StorageBalance;
use near_contract_standards::storage_management::StorageBalanceBounds;
use near_sdk::{env, near_bindgen, AccountId, Balance, StorageUsage};

pub const U128_STORAGE: StorageUsage = 16;
// const U64_STORAGE: StorageUsage = 8;
pub const U32_STORAGE: StorageUsage = 4;

/// max length of account id is 64 bytes. We charge per byte.
pub const ACC_ID_STORAGE: StorageUsage = 64;

// key prefix of a collection takes 4 bytes;
const COLLECTION_KEY_PREFIX: StorageUsage = 4;

// _number_tickers_per_lottery_id
const NUMBER_TICKERS_PER_LOTTERY_ID: StorageUsage =
    COLLECTION_KEY_PREFIX + U32_STORAGE + COLLECTION_KEY_PREFIX + U32_STORAGE + U128_STORAGE;

//_user_ticket_ids_per_lottery_id
const NUMBER_TICKETS_PER_LOTTERY_ID_USAGE: StorageUsage =
    COLLECTION_KEY_PREFIX + ACC_ID_STORAGE + COLLECTION_KEY_PREFIX + U32_STORAGE + U32_STORAGE;

// _storage_deposits
const STORAGE_DEPOSITS_USAGE: StorageUsage = COLLECTION_KEY_PREFIX + ACC_ID_STORAGE + U128_STORAGE;

// Contract Storage
pub const INIT_ACCOUNT_STORAGE: u128 = (NUMBER_TICKERS_PER_LOTTERY_ID
    + NUMBER_TICKETS_PER_LOTTERY_ID_USAGE
    + STORAGE_DEPOSITS_USAGE) as u128;

#[near_bindgen]
impl NearLott {
    /// NEP154 required
    pub fn storage_minimum_balance(&self) -> U128 {
        self.min_storage_usage().into()
    }

    #[payable]
    pub fn storage_deposit(&mut self, account_id: Option<AccountId>) {
        let storage_account_id = account_id
            .map(|a| a.into())
            .unwrap_or_else(env::predecessor_account_id);
        let deposit = env::attached_deposit();
        assert!(
            deposit >= INIT_ACCOUNT_STORAGE,
            "{}: {}",
            ERR33_INSUFFICIENT_MINIMUM_REQUIRES,
            INIT_ACCOUNT_STORAGE
        );

        let mut balance: u128 = self
            .data_mut()
            ._storage_deposits
            .get(&storage_account_id)
            .unwrap_or(0);
        balance += deposit;
        self.data_mut()
            ._storage_deposits
            .insert(&storage_account_id, &balance);
    }

    pub fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        StorageBalanceBounds {
            min: self.min_storage_usage(),
            max: None,
        }
    }

    #[payable]
    pub fn storage_withdraw(&mut self) {
        self.assert_one_yoctor();
        let sender_id: AccountId = env::predecessor_account_id();
        let available_amount = self.storage_available(sender_id.clone());
        if available_amount.0 > 0 {
            Promise::new(env::predecessor_account_id()).transfer(available_amount.into());
        }

        let usage_data = self.account_storage_usage(sender_id.clone());
        if usage_data > 0 {
            self.data_mut()
                ._storage_deposits
                .insert(&env::predecessor_account_id(), &usage_data);
        }
    }

    /// Returns minimal account deposit storage usage possible.
    pub fn min_storage_usage(&self) -> U128 {
        U128(INIT_ACCOUNT_STORAGE as Balance * env::storage_byte_cost())
    }

    /// Returns how much NEAR is available for storage.
    pub fn storage_available(&self, _account_id: AccountId) -> U128 {
        let deposited = self.data()._storage_deposits.get(&_account_id).unwrap_or(0);
        let locked = self.account_storage_usage(_account_id);
        if deposited > locked {
            U128(deposited - locked)
        } else {
            U128(0)
        }
    }

    /// Retuns how much NEAR need to cover for storage
    pub fn get_user_cover_for_storage(&self, _account_id: AccountId) -> U128 {
        let deposited = self.data()._storage_deposits.get(&_account_id).unwrap_or(0);
        let locked = self.account_storage_usage(_account_id);
        if locked > deposited {
            let cover_usage: u128 = locked - deposited;
            return U128(cover_usage);
        }
        U128(0)
    }

    /// Asserts there is sufficient amount of $NEAR to cover storage usage.
    pub fn assert_storage_usage(&self) {
        assert_storage_usage_data(&self.data())
    }

    /// Returns amount of $NEAR necessary to cover storage used by this data structure.
    pub fn account_storage_usage(&self, account_id: AccountId) -> Balance {
        account_storage_usage_data(&self.data(), account_id)
    }

    pub fn storage_balance_of(&self, account_id: AccountId) -> U128 {
        self.data()
            ._storage_deposits
            .get(&account_id)
            .unwrap_or(0)
            .into()
    }

    #[allow(unused_variables)]
    #[payable]
    fn storage_unregister(&mut self, force: Option<bool>) -> bool {
        env::panic_str("The account can't be unregistered");
    }
}
