use crate::*;
use near_contract_standards::storage_management::StorageBalanceBounds;
use near_sdk::{env, near_bindgen, AccountId, Balance, StorageUsage};

const U128_STORAGE: StorageUsage = 16;
const U64_STORAGE: StorageUsage = 8;
const U32_STORAGE: StorageUsage = 4;

/// max length of account id is 64 bytes. We charge per byte.
const ACC_ID_STORAGE: StorageUsage = 64;

// key prefix of a collection takes 4 bytes;
const COLLECTION_KEY_PREFIX: StorageUsage = 4;

// _number_tickers_per_lottery_id
const NUMBER_TICKERS_PER_LOTTERY_ID: StorageUsage =
    COLLECTION_KEY_PREFIX + U32_STORAGE + COLLECTION_KEY_PREFIX + U32_STORAGE + U128_STORAGE;

//_number_tickers_per_lottery_id:
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
    pub fn storage_minimum_balance(&mut self) -> u128 {
        self.min_storage_usage()
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

    fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        StorageBalanceBounds {
            min: U128(INIT_ACCOUNT_STORAGE),
            max: None,
        }
    }

    #[payable]
    pub fn storage_withdraw(&mut self) {
        self.assert_one_yoctor();
        let sender_id: AccountId = env::predecessor_account_id();
        let available_amount = self.storage_available(sender_id);
        if available_amount > 0 {
            Promise::new(env::predecessor_account_id()).transfer(available_amount);
        }

        let usage_data = self.storage_usage(env::predecessor_account_id());
        if usage_data > 0 {
            self.data_mut()
                ._storage_deposits
                .insert(&env::predecessor_account_id(), &usage_data);
        }
    }

    /// Returns minimal account deposit storage usage possible.
    pub fn min_storage_usage(&self) -> Balance {
        INIT_ACCOUNT_STORAGE as Balance * env::storage_byte_cost()
    }

    /// Returns how much NEAR is available for storage.
    pub fn storage_available(&self, _account_id: AccountId) -> Balance {
        let deposited = self.data()._storage_deposits.get(&_account_id).unwrap_or(0);
        let locked = self.storage_usage(_account_id);
        if deposited > locked {
            deposited - locked
        } else {
            0
        }
    }

    /// Asserts there is sufficient amount of $NEAR to cover storage usage.
    pub fn assert_storage_usage(&self, _account_id: AccountId) {
        let deposited = self.data()._storage_deposits.get(&_account_id).unwrap_or(0);
        assert!(
            self.storage_usage(_account_id) <= deposited,
            "{}",
            ERR32_INSUFFICIENT_STORAGE
        );
    }

    /// Returns amount of $NEAR necessary to cover storage used by this data structure.
    pub fn storage_usage(&self, _account_id: AccountId) -> u128 {
        let user_lotteries = self
            .data()
            ._user_ticket_ids_per_lottery_id
            .get(&_account_id)
            .unwrap_or(UnorderedMap::new(b"A".to_vec()));

        let values = user_lotteries
            .values()
            .map(|user_tickets| user_tickets.len() as u64)
            .collect::<Vec<_>>();
        let number_of_tickets: u64 = values.iter().sum();
        // number of lotteries x (each lottery including: list of ticket numbers and 4 bytes)
        let storage_use_store_lotteries: StorageUsage =
            user_lotteries.len() * (U32_STORAGE * number_of_tickets);

        // storage using for list of _number_tickers_per_lottery_id
        let storage_number_of_tickets_elements_numbers =
            user_lotteries.len() * (U32_STORAGE + U128_STORAGE * 6);

        INIT_ACCOUNT_STORAGE
            + storage_use_store_lotteries as u128
            + storage_number_of_tickets_elements_numbers as u128
    }

    pub fn storage_balance_of(&self, account_id: AccountId) -> U128 {
        self.data()
            ._storage_deposits
            .get(&account_id)
            .unwrap_or(0)
            .into()
    }
}
