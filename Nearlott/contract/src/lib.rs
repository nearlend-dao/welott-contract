use crate::info::DEFAULT_AUDITOR_ACCOUNT_ID;
use crate::info::DEFAULT_WEB_APP_URL;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, serde_json::json, AccountId, Balance, BorshStorageKey, Promise, Timestamp,
};
use std::fmt;

pub use crate::assert::*;
pub use crate::callback::*;
pub use crate::errors::*;
pub use crate::gas::*;
pub use crate::logic::*;
pub use crate::owner::*;
pub use crate::storage::*;
pub use crate::utils::*;
pub use crate::views::*;

mod assert;
mod callback;
mod config;
mod errors;
mod gas;
mod info;
mod logic;
mod owner;
mod storage;
mod utils;
mod views;

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

impl Default for Lottery {
    fn default() -> Self {
        Self {
            status: Status::Open,
            start_time: 0,
            end_time: 0,
            price_ticket_in_near: 0,
            discount_divisor: 0,
            rewards_breakdown: vec![],
            treasury_fee: 0,
            near_per_bracket: vec![],
            count_winners_per_bracket: vec![],
            first_ticket_id: 0,
            first_ticket_id_next_lottery: 0,
            amount_collected_in_near: 0,
            final_number: 0,
        }
    }
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
    pub random_result: u32,
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
    pub web_app_url: Option<String>,
    pub auditor_account_id: Option<AccountId>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Ticket {
    pub number: u32,
    pub owner: AccountId,
}

impl Default for Ticket {
    fn default() -> Self {
        Self {
            number: 0,
            owner: AccountId::new_unchecked("no_account".to_string()),
        }
    }
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
            web_app_url: Some(String::from(DEFAULT_WEB_APP_URL)),
            auditor_account_id: Some(AccountId::new_unchecked(String::from(
                DEFAULT_AUDITOR_ACCOUNT_ID,
            ))),
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

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::info::CONTRACT_NAME;
    use crate::info::CONTRACT_VERSION;
    use crate::info::DEVELOPERS_ACCOUNT_ID;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn setup_contract() -> (VMContextBuilder, NearLott) {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        let contract = NearLott::new(accounts(0), accounts(1), accounts(2), accounts(3));
        (context, contract)
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());

        let contract = NearLott::new(accounts(0), accounts(1), accounts(2), accounts(3));
        testing_env!(context.is_view(true).build());

        let data = contract.data();
        assert_eq!(contract.get_owner(), accounts(0));
        assert_eq!(data.injector_address, accounts(1));
        assert_eq!(data.operator_address, accounts(2));
        assert_eq!(data.treasury_address, accounts(3));

        assert_eq!(data.state, RunningState::Running);
        assert_eq!(data.current_lottery_id, 0);
        assert_eq!(data.current_ticket_id, 0);
        assert_eq!(data.max_number_tickets_per_buy_or_claim, 100);
        assert_eq!(data.max_price_ticket_in_near, 50000);
        assert_eq!(data.min_price_ticket_in_near, 5);

        assert_eq!(data.pending_injection_next_lottery, 0);
        assert_eq!(data.min_discount_divisor, 300);
        assert_eq!(data.min_length_lottery, 14100);
        assert_eq!(data.max_length_lottery, 345900);
        assert_eq!(data.max_treasury_fee, 3000);

        assert_eq!(data._lotteries.len(), 0);
        assert_eq!(data._tickets.len(), 0);
        assert_eq!(data._number_tickers_per_lottery_id.len(), 0);
        assert_eq!(data._user_ticket_ids_per_lottery_id.len(), 0);
        assert_eq!(data.random_result, 0);

        assert_eq!(data._bracket_calculator.get(&0), Some(1));
        assert_eq!(data._bracket_calculator.get(&1), Some(11));
        assert_eq!(data._bracket_calculator.get(&2), Some(111));
        assert_eq!(data._bracket_calculator.get(&3), Some(1111));
        assert_eq!(data._bracket_calculator.get(&4), Some(11111));
        assert_eq!(data._bracket_calculator.get(&5), Some(111111));
    }

    #[test]
    #[should_panic(expected = "Can only be called by the owner")]
    fn test_set_owner_invalid() {
        let (mut context, mut contract) = setup_contract();

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(1)
            .build());

        contract.set_owner(accounts(1));
    }

    #[test]
    fn test_set_owner() {
        let (mut context, mut contract) = setup_contract();

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(1)
            .build());

        contract.set_owner(accounts(1));
        let data = contract.data();
        assert_eq!(data.owner_id, accounts(1));
    }

    #[test]
    fn _test_full_asserts() {
        let (mut context, contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(1)
            .build());

        contract.assert_owner_calling();

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(2)
            .build());
        contract.assert_injector_or_owner_calling();

        testing_env!(context
            .predecessor_account_id(accounts(2))
            .attached_deposit(1)
            .build());
        contract.assert_operator_calling();
        contract.assert_operator_or_owner_calling();

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(1)
            .build());
        contract.assert_one_yoctor();

        contract.assert_contract_running();
    }

    #[test]
    pub fn test_set_operator_and_treasury_and_injector_addresses() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(1)
            .build());

        contract.set_operator_and_treasury_and_injector_addresses(
            accounts(3),
            accounts(4),
            accounts(5),
        );

        let data = contract.data();
        assert_eq!(data.operator_address, accounts(3));
        assert_eq!(data.treasury_address, accounts(4));
        assert_eq!(data.injector_address, accounts(5));
    }

    #[test]
    pub fn test_set_max_number_tickets_per_buy() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(1)
            .build());

        contract.set_max_number_tickets_per_buy(1000);
        let data = contract.data();
        assert_eq!(data.max_number_tickets_per_buy_or_claim, 1000);
    }

    #[test]
    pub fn test_set_min_and_max_ticket_price_in_near() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(1)
            .build());

        contract.set_min_and_max_ticket_price_in_near(500, 1000);
        let data = contract.data();
        assert_eq!(data.min_price_ticket_in_near, 500);
        assert_eq!(data.max_price_ticket_in_near, 1000);
    }

    #[test]
    pub fn test_get_contract_info() {
        let (_, contract) = setup_contract();
        let contract_info = contract.get_contract_info();

        assert_eq!(contract_info.dataVersion, 1);
        assert_eq!(contract_info.name, CONTRACT_NAME);
        assert_eq!(contract_info.version, CONTRACT_VERSION);
        assert_eq!(
            contract_info.source,
            String::from("https://gitlab.com/nearlend/nearlott")
        );
        assert_eq!(
            contract_info.standards,
            vec!["NEP-141", "NEP-145".into(), "SP".into()]
        );
        assert_eq!(contract_info.webAppUrl, contract.web_app_url);
        assert_eq!(
            contract_info.developersAccountId,
            String::from(DEVELOPERS_ACCOUNT_ID)
        );
        assert_eq!(
            contract_info.auditorAccountId,
            contract.auditor_account_id.into()
        );
    }
    #[test]
    fn test_get_random_number() {
        let (mut context, contract) = setup_contract();

        testing_env!(context
            .predecessor_account_id(accounts(5))
            .random_seed([
                1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 4, 5, 6, 7, 8, 9, 1, 2, 3, 3, 4, 5, 6, 6, 7, 8, 9,
                1, 2, 4, 5
            ])
            .block_timestamp(111 + 1)
            .build());

        let result = contract.view_random();
        println!("result: {:?}", result);
    }
}
