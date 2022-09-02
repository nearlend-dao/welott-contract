use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, serde_json::json, AccountId, Balance, BorshStorageKey, Promise, Timestamp,
};
use std::fmt;

mod assert;
mod callback;
mod errors;
mod gas;
mod logic;
mod owner;
mod random_generator;
mod storage;
mod utils;
mod views;

pub use crate::assert::*;
pub use crate::callback::*;
pub use crate::errors::*;
pub use crate::gas::*;
pub use crate::logic::*;
pub use crate::owner::*;
pub use crate::random_generator::*;
pub use crate::storage::*;
pub use crate::utils::*;
pub use crate::views::*;

pub type TimestampSec = u32;
pub type LotteryId = u32;
pub type TicketId = u32;
pub type TicketNumber = u32;
pub type TicketElementNumber = u32;
pub type CountTicketValue = u128;
pub type BracketPosition = u32;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
pub enum Status {
    Pending,
    Open,
    Close,
    Claimable,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Pending => write!(f, "Pending"),
            Status::Open => write!(f, "Open"),
            Status::Close => write!(f, "Close"),
            Status::Claimable => write!(f, "Claimable"),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Lottery {
    pub status: Status,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub price_ticket_in_near: u128,
    pub discount_divisor: u128,
    pub rewards_breakdown: Vec<u128>,
    pub treasury_fee: u128,
    pub near_per_bracket: Vec<u128>,
    pub count_winners_per_bracket: Vec<u128>,
    pub first_ticket_id: u32,
    pub first_ticket_id_next_lottery: u32,
    pub amount_collected_in_near: u128,
    pub final_number: u32,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Lotteries,
    Tickets,
    BracketCalculator,
    NumberTickersPerLotteryId,
    UserTicketsPerLottery,
    StorageDeposits,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
pub enum RunningState {
    Running,
    Paused,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ContractData {
    pub owner_id: AccountId,
    pub state: RunningState,
    pub current_lottery_id: LotteryId,
    pub current_ticket_id: TicketId,

    pub injector_address: AccountId,
    pub operator_address: AccountId,
    pub treasury_address: AccountId,
    pub max_number_tickets_per_buy_or_claim: u64,

    pub max_price_ticket_in_near: u128,
    pub min_price_ticket_in_near: u128,

    pub pending_injection_next_lottery: u128,

    pub min_discount_divisor: u128,
    pub min_length_lottery: u32,
    pub max_length_lottery: u32,
    pub max_treasury_fee: u128,

    // mapping are cheaper than arrays
    pub _lotteries: UnorderedMap<LotteryId, Lottery>,
    pub _tickets: UnorderedMap<TicketId, Ticket>,

    // bracket calculator is used for verifying claims for ticket prizes
    pub _bracket_calculator: LookupMap<BracketPosition, u32>,

    // keeps track of number of ticket per unique combination for each lotteryId
    pub _number_tickers_per_lottery_id:
        UnorderedMap<LotteryId, UnorderedMap<TicketElementNumber, CountTicketValue>>,

    // keep track of user ticket ids for a given lotteryId
    pub _user_ticket_ids_per_lottery_id:
        UnorderedMap<AccountId, UnorderedMap<LotteryId, Vec<TicketId>>>,

    // keep track of user deposit storage
    pub _storage_deposits: LookupMap<AccountId, Balance>,

    // the last random result
    pub random_result: u128,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VersionedContractData {
    V0001(ContractData),
}
impl VersionedContractData {}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NearLott {
    data: VersionedContractData,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Ticket {
    pub number: u32,
    pub owner: AccountId,
}

// Implement the contract structure
#[near_bindgen]
impl NearLott {
    #[init]
    pub fn new(
        owner_id: AccountId,
        injector_address: AccountId,
        operator_address: AccountId,
        treasury_address: AccountId,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");

        // Initializes a mapping
        let mut brackets = LookupMap::new(StorageKey::BracketCalculator);
        brackets.insert(&0, &1);
        brackets.insert(&1, &11);
        brackets.insert(&2, &111);
        brackets.insert(&3, &1111);
        brackets.insert(&4, &11111);
        brackets.insert(&5, &111111);

        let this = Self {
            data: VersionedContractData::V0001(ContractData {
                owner_id,
                injector_address,
                operator_address,
                treasury_address,
                state: RunningState::Running,
                current_lottery_id: 0,
                current_ticket_id: 0,
                max_number_tickets_per_buy_or_claim: 100,
                max_price_ticket_in_near: 50000, // max: 50 x 1000
                min_price_ticket_in_near: 5,     // 0.005 x 1000
                pending_injection_next_lottery: 0,
                min_discount_divisor: 300,
                min_length_lottery: 14100,  // 4 hours - 5 minutes;
                max_length_lottery: 345900, //4 days + 5 minutes;
                max_treasury_fee: 3000,     // 30%
                _lotteries: UnorderedMap::new(StorageKey::Lotteries),
                _tickets: UnorderedMap::new(StorageKey::Tickets),
                _bracket_calculator: brackets,
                _number_tickers_per_lottery_id: UnorderedMap::new(
                    StorageKey::NumberTickersPerLotteryId,
                ),
                _user_ticket_ids_per_lottery_id: UnorderedMap::new(
                    StorageKey::UserTicketsPerLottery,
                ),
                _storage_deposits: LookupMap::new(StorageKey::StorageDeposits),
                random_result: 0,
            }),
        };

        this
    }
}

impl NearLott {
    fn data(&self) -> &ContractData {
        match &self.data {
            VersionedContractData::V0001(data) => data,
            _ => unimplemented!(),
        }
    }

    fn data_mut(&mut self) -> &mut ContractData {
        match &mut self.data {
            VersionedContractData::V0001(data) => data,
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {}
