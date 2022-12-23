use crate::info::DEFAULT_AUDITOR_ACCOUNT_ID;
use crate::info::DEFAULT_WEB_APP_URL;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, serde_json::json, AccountId, Balance, BorshStorageKey, PanicOnDefault,
    Promise, Timestamp, ONE_NEAR,
};
use std::collections::HashMap;
use std::fmt;

pub use crate::account::*;
pub use crate::account_btn_counting::*;
pub use crate::assert::*;
pub use crate::callback::*;
pub use crate::config::*;
pub use crate::errors::*;
pub use crate::gas::*;
pub use crate::logic::*;
pub use crate::owner::*;
pub use crate::storage::*;
pub use crate::storage_tracker::*;
pub use crate::utils::*;
pub use crate::views::*;

mod account;
mod account_btn_counting;
mod assert;
mod callback;
mod config;
mod errors;
mod gas;
mod info;
mod logic;
mod owner;
mod storage;
mod storage_tracker;
mod utils;
mod views;

pub type TimestampSec = u32;
pub type LotteryId = u32;
pub type TicketId = u32;
pub type TicketNumber = u32;
pub type BracketTicketNumber = u32;
pub type CountTicketValue = u128;
pub type BracketPosition = u32;

#[derive(Debug, BorshSerialize, BorshDeserialize, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum Status {
    Open,
    Close,
    Claimable,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Open => write!(f, "Open"),
            Status::Close => write!(f, "Close"),
            Status::Claimable => write!(f, "Claimable"),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Lottery {
    pub lottery_id: LotteryId,
    pub status: Status,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub price_ticket_in_near: u128,
    pub discount_divisor: u128,
    pub rewards_breakdown: Vec<u128>,
    pub reserve_fee: u128,
    pub near_per_bracket: Vec<u128>,
    pub count_winners_per_bracket: Vec<u128>,
    pub first_ticket_id: u32,
    pub first_ticket_id_next_lottery: u32,
    pub amount_collected_in_near: u128,
    pub last_pot_size: u128,
    pub final_number: u32,
    pub operate_fee: u128,
}

impl Default for Lottery {
    fn default() -> Self {
        Self {
            lottery_id: 0,
            status: Status::Open,
            start_time: 0,
            end_time: 0,
            price_ticket_in_near: 0,
            discount_divisor: 0,
            rewards_breakdown: vec![],
            reserve_fee: 0,
            near_per_bracket: vec![],
            count_winners_per_bracket: vec![],
            first_ticket_id: 0,
            first_ticket_id_next_lottery: 0,
            amount_collected_in_near: 0,
            last_pot_size: 0,
            final_number: 0,
            operate_fee: 0,
        }
    }
}

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Accounts,
    Lotteries,
    Tickets,
    BracketCalculator,
    Storage,
    BracketTicketNumbers { lottery_id: LotteryId },
    AccountTickets { account_id: AccountId },
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
pub enum RunningState {
    Running,
    Paused,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
pub enum PermissionUpdateState {
    Allow,
    Disallow,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct ContractData {
    pub owner_id: AccountId,
    pub state: RunningState,
    pub current_lottery_id: LotteryId,
    pub current_ticket_id: TicketId,

    pub injector_address: AccountId,
    pub operator_address: AccountId,
    pub treasury_address: AccountId,
    pub max_number_tickets_per_buy_or_claim: u64,
    pub pending_injection_next_lottery: u128,

    pub min_discount_divisor: u128,
    pub max_reserve_fee: u128,

    // Config Lottery
    config_lottery: ConfigLottery,

    // mapping are cheaper than arrays
    pub _lotteries: UnorderedMap<LotteryId, Lottery>,
    pub _tickets: UnorderedMap<TicketId, Ticket>,
    pub _bracket_tickets_number:
        UnorderedMap<LotteryId, UnorderedMap<BracketTicketNumber, CountTicketValue>>,

    // bracket calculator is used for verifying claims for ticket prizes
    pub _bracket_calculator: LookupMap<BracketPosition, u32>,

    // the last random result
    pub random_result: u32,

    // a flag permission to update any configuration if a lottery running
    pub permission_update: PermissionUpdateState,

    pub accounts: UnorderedMap<AccountId, VAccount>,
    pub storage: LookupMap<AccountId, VStorage>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VersionedContractData {
    V0001(ContractData),
}
impl VersionedContractData {}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NearLott {
    data: VersionedContractData,
    pub web_app_url: Option<String>,
    pub auditor_account_id: Option<AccountId>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Ticket {
    pub number: u32,
    pub owner: AccountId,
}

impl Default for Ticket {
    fn default() -> Self {
        Self {
            number: 0,
            owner: AccountId::new_unchecked("welott_initialize.near".to_string()),
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
        // Config Lottery
        config_lottery: ConfigLottery,
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

        Self {
            data: VersionedContractData::V0001(ContractData {
                owner_id,
                injector_address,
                operator_address,
                treasury_address,
                state: RunningState::Running,
                current_lottery_id: 0,
                current_ticket_id: 0,
                max_number_tickets_per_buy_or_claim: 12,
                pending_injection_next_lottery: 0,
                min_discount_divisor: 0,
                max_reserve_fee: 3000, // 30%
                config_lottery,
                _lotteries: UnorderedMap::new(StorageKey::Lotteries),
                _tickets: UnorderedMap::new(StorageKey::Tickets),
                _bracket_calculator: brackets,
                random_result: 0,
                permission_update: PermissionUpdateState::Allow,
                storage: LookupMap::new(StorageKey::Storage),
                accounts: UnorderedMap::new(StorageKey::Accounts),
                _bracket_tickets_number: UnorderedMap::new(StorageKey::BracketTicketNumbers {
                    lottery_id: 0,
                }),
            }),
            web_app_url: Some(String::from(DEFAULT_WEB_APP_URL)),
            auditor_account_id: Some(AccountId::new_unchecked(String::from(
                DEFAULT_AUDITOR_ACCOUNT_ID,
            ))),
        }
    }
}

impl NearLott {
    fn data(&self) -> &ContractData {
        match &self.data {
            VersionedContractData::V0001(data) => data,
        }
    }

    fn data_mut(&mut self) -> &mut ContractData {
        match &mut self.data {
            VersionedContractData::V0001(data) => data,
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

    fn set_config_lottery(is_default: bool) -> ConfigLottery {
        if is_default {
            ConfigLottery {
                time_run_lottery: 12345678 as u64 + 1,
                price_ticket_in_near: U128::from(0),
                discount_divisor: U128::from(0),
                rewards_breakdown: vec![],
                reserve_fee: U128::from(0),
                operate_fee: U128::from(0),
            }
        } else {
            ConfigLottery {
                time_run_lottery: 12345678 as u64 + 1,
                price_ticket_in_near: U128(1000000000000000000000000),
                discount_divisor: U128(2000),
                rewards_breakdown: vec![125, 375, 750, 1250, 2500, 5000],
                reserve_fee: U128(2000),
                operate_fee: U128(500),
            }
        }
    }

    fn setup_contract(config_lottery: ConfigLottery) -> (VMContextBuilder, NearLott) {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        let contract = NearLott::new(
            accounts(0),
            accounts(1),
            accounts(2),
            accounts(3),
            config_lottery,
        );
        (context, contract)
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());

        let contract = NearLott::new(
            accounts(0),
            accounts(1),
            accounts(2),
            accounts(3),
            set_config_lottery(true),
        );
        testing_env!(context.is_view(true).build());

        let config = contract.get_config();
        assert_eq!(contract.get_owner(), accounts(0));
        assert_eq!(config.injector_address, accounts(1));
        assert_eq!(config.operator_address, accounts(2));
        assert_eq!(config.treasury_address, accounts(3));

        assert_eq!(config.state, RunningState::Running);
        assert_eq!(config.current_lottery_id, 0);
        assert_eq!(config.current_ticket_id, 0);
        assert_eq!(config.max_number_tickets_per_buy_or_claim, 12);

        assert_eq!(config.pending_injection_next_lottery, 0);
        assert_eq!(config.min_discount_divisor, 0);
        assert_eq!(config.max_reserve_fee, 3000);

        let data = contract.data();
        assert_eq!(data._lotteries.len(), 0);
        assert_eq!(data._tickets.len(), 0);
        assert_eq!(data.random_result, 0);
        assert_eq!(data._bracket_tickets_number.len(), 0);

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
        let (mut context, mut contract) = setup_contract(set_config_lottery(true));

        testing_env!(context
            .predecessor_account_id(accounts(1))
            .attached_deposit(1)
            .build());

        contract.set_owner(accounts(1));
    }

    #[test]
    fn test_set_owner() {
        let (mut context, mut contract) = setup_contract(set_config_lottery(true));

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
        let (mut context, contract) = setup_contract(set_config_lottery(true));
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
    pub fn test_get_contract_info() {
        let (_, contract) = setup_contract(set_config_lottery(true));
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
    fn test_start_lottery() {
        let (mut context, mut contract) = setup_contract(set_config_lottery(false));
        // deposit storage
        deposit_for_account(&mut context, &mut contract, accounts(2));

        // start a lottery
        start_a_lottery(&mut context, &mut contract, accounts(2));

        let current_lottery_id = contract.data().current_lottery_id;
        let data2 = contract.data();
        assert_eq!(current_lottery_id, 1);
        assert_eq!(data2.current_lottery_id, 1);
    }

    fn deposit_for_account(
        context: &mut VMContextBuilder,
        contract: &mut NearLott,
        account_id: AccountId,
    ) {
        let minimum_deposit = contract.storage_balance_bounds();
        testing_env!(context
            .predecessor_account_id(account_id.clone())
            .attached_deposit(minimum_deposit.min.0)
            .build());
        contract.storage_deposit(Some(account_id.clone()), Some(true));

        let buyer = Account::new(&account_id);
        println!(
            "storage_tracker.bytes_added: {:?}",
            buyer.storage_tracker.bytes_added
        );
        println!(
            "storage_tracker.bytes_released: {:?}",
            buyer.storage_tracker.bytes_released
        );
        let available_storage = contract.debug_storage_balance_of(account_id).unwrap();
        println!(
            "debug_storage_balance_of: ({:?}, {:?})",
            available_storage.total.0, available_storage.available.0
        );
    }

    fn start_a_lottery(
        context: &mut VMContextBuilder,
        contract: &mut NearLott,
        account_id: AccountId,
    ) {
        testing_env!(context
            .predecessor_account_id(account_id.clone())
            .attached_deposit(1)
            .build());
        contract.start_lottery();
    }

    fn close_lottery(context: &mut VMContextBuilder, contract: &mut NearLott) {
        // // close ticket
        let start_time = 162615600000000;
        let end_time = start_time + 12345678 as u64 + 1;
        testing_env!(context
            .predecessor_account_id(accounts(2))
            .attached_deposit(1)
            .random_seed([
                1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 4, 5, 6, 7, 8, 9, 1, 2, 3, 3, 4, 5, 6, 6, 7, 8, 9,
                1, 2, 4, 5
            ])
            .block_timestamp(end_time + 5000)
            .build());
        contract.close_lottery();
    }

    fn buy_a_ticket(
        context: &mut VMContextBuilder,
        contract: &mut NearLott,
        account_id: AccountId,
        current_lottery_id: LotteryId,
        ticket_number: Vec<TicketNumber>,
    ) {
        testing_env!(context
            .predecessor_account_id(account_id)
            .attached_deposit(ticket_number.len() as u128 * 10u128.pow(24))
            .build());
        contract.buy_tickets(current_lottery_id, ticket_number);
    }

    #[test]
    fn test_desposit_buy_tickets() {
        let (mut context, mut contract) = setup_contract(set_config_lottery(false));

        // deposit storage
        deposit_for_account(&mut context, &mut contract, accounts(2));

        // start a lottery
        start_a_lottery(&mut context, &mut contract, accounts(2));

        // do buying tickets
        let current_lottery_id = contract.data().current_lottery_id;
        // buy some tickets
        buy_a_ticket(
            &mut context,
            &mut contract,
            accounts(2),
            current_lottery_id,
            vec![1292877],
        );
        let number_of_tickets = contract.view_number_tickets_per_lottery(current_lottery_id);
        assert_eq!(number_of_tickets, 1);

        let ticket_number = 1292877;
        let key_bracket1 = 1 + (ticket_number % 10);
        let key_bracket2 = 11 + (ticket_number % 100);
        let key_bracket3 = 111 + (ticket_number % 1000);
        let key_bracket4 = 1111 + (ticket_number % 10000);
        let key_bracket5 = 11111 + (ticket_number % 100000);
        let key_bracket6 = 111111 + (ticket_number % 1000000);
        println!("key_bracket1: {:?}", key_bracket1);
        println!("key_bracket2: {:?}", key_bracket2);
        println!("key_bracket3: {:?}", key_bracket3);
        println!("key_bracket4: {:?}", key_bracket4);
        println!("key_bracket5: {:?}", key_bracket5);
        println!("key_bracket6: {:?}", key_bracket6);

        // check available storage
        let available_storage = contract.debug_storage_balance_of(accounts(2)).unwrap();
        println!(
            "debug_storage_balance_of: ({:?}, {:?})",
            available_storage.total.0, available_storage.available.0
        );
        let buyer = internal_get_account_unwrap_by_contract_data(contract.data_mut(), &accounts(2));
        let data2 = contract.data();
        let ticket_per_lottery_id = data2
            ._bracket_tickets_number
            .get(&current_lottery_id)
            .unwrap();
        assert_eq!(ticket_per_lottery_id.len(), 6);

        // number of lottery user has joined.
        let tickets = buyer
            .internal_get_ticket_ids_per_lottery(&current_lottery_id)
            .unwrap_or(vec![]);
        assert_eq!(tickets.len(), 1);

        // get first ticket. Start a ticketId is zero.
        assert_eq!(tickets[0], 0);

        // // test general number of tickets
        let ticket = contract.data()._tickets.get(&tickets[0]).unwrap();
        assert_eq!(ticket.number, 1292877);
        assert_eq!(ticket.owner, accounts(2));
    }

    #[test]
    fn test_draw_final_number_and_make_lottery_claimable() {
        let (mut context, mut contract) = setup_contract(set_config_lottery(false));

        // deposit storage
        deposit_for_account(&mut context, &mut contract, accounts(2));

        // start a lottery
        start_a_lottery(&mut context, &mut contract, accounts(2));

        let current_lottery_id = contract.data().current_lottery_id;

        // buy ticket 1
        buy_a_ticket(
            &mut context,
            &mut contract,
            accounts(2),
            current_lottery_id,
            vec![1292877],
        );

        //buy ticket 2
        deposit_for_account(&mut context, &mut contract, accounts(3));
        buy_a_ticket(
            &mut context,
            &mut contract,
            accounts(3),
            current_lottery_id,
            vec![1292876, 1380611],
        );

        let data_mut = contract.data_mut();
        let current_lottery_id = data_mut.current_lottery_id;

        // test user tickets
        let account3 = internal_get_account_unwrap_by_contract_data(data_mut, &accounts(3));
        let user3_ticket_ids = account3
            .internal_get_ticket_ids_per_lottery(&current_lottery_id)
            .unwrap_or(vec![]);
        assert_eq!(user3_ticket_ids.len(), 2);
        assert_eq!(user3_ticket_ids[0], 1); // current ticketid of account(3) is 1
        assert_eq!(user3_ticket_ids[1], 2); // current ticketid of account(3) is 2

        // test tickets
        assert_eq!(data_mut._tickets.len(), 3);
        let firt_ticket: Ticket = data_mut._tickets.get(&0).unwrap();
        let second_ticket: Ticket = data_mut._tickets.get(&1).unwrap();
        let third_ticket: Ticket = data_mut._tickets.get(&2).unwrap();
        assert_eq!(firt_ticket.number, 1292877);
        assert_eq!(second_ticket.number, 1292876);
        assert_eq!(third_ticket.number, 1380611);

        // close lottery
        close_lottery(&mut context, &mut contract);

        // check random number generated.
        let data2 = contract.data();
        let wining_number = data2.random_result;
        assert_eq!(wining_number, 0);
        println!("Random result: {}", data2.random_result);
        let lottery_close_lottery = data2._lotteries.get(&current_lottery_id).unwrap();
        assert_eq!(lottery_close_lottery.status, Status::Close);
        // // draw the final number
        testing_env!(context
            .predecessor_account_id(accounts(2))
            .attached_deposit(1)
            .build());
        contract.draw_final_number_and_make_lottery_claimable(current_lottery_id, true);
        contract.data_mut().random_result = 1327419;

        let data3 = contract.data();
        let lottery_claim_lottery = data3._lotteries.get(&current_lottery_id).unwrap();

        assert_eq!(lottery_claim_lottery.status, Status::Claimable);
        // assert_eq!(lottery_claim_lottery.final_number, 1327419);
        // assert_eq!(data3.random_result, 1327419);
        // there is no one be a winner
        let operate_fee = (lottery_claim_lottery.amount_collected_in_near
            - lottery_claim_lottery.last_pot_size)
            * data3.config_lottery.operate_fee.0
            / 10000;
        println!(
            "amount_collected_in_near: {:?}",
            lottery_claim_lottery.amount_collected_in_near
        );
        println!("operate_fee: {:?}", operate_fee);
        let amount_to_shared = ((lottery_claim_lottery.amount_collected_in_near - operate_fee)
            * (10000 - data3.config_lottery.reserve_fee.0))
            / 10000;
        println!("amount_to_shared: {:?}", amount_to_shared);
        assert_eq!(amount_to_shared, 2279240000000000000000000);
        // reserver pool
        let reserver_pool = ((lottery_claim_lottery.amount_collected_in_near - operate_fee)
            * data3.config_lottery.reserve_fee.0)
            / 10000;
        // incase there is no one won.
        assert_eq!(
            data3.pending_injection_next_lottery,
            amount_to_shared + reserver_pool
        );
    }

    #[test]
    fn test_get_user_cover_for_storage() {
        let (mut context, mut contract) = setup_contract(set_config_lottery(false));

        // deposit storage
        deposit_for_account(&mut context, &mut contract, accounts(2));

        // check balance available
        let available_storage = contract.debug_storage_balance_of(accounts(2)).unwrap();
        println!(
            "test_get_user_cover_for_storage: {:?}, {:?}",
            available_storage.available.0, available_storage.total.0
        );
        assert_eq!(available_storage.total.0, 100000000000000000000000);
        assert_eq!(available_storage.available.0, 96720000000000000000000);
    }

    #[test]
    fn test_buy_tickets() {
        let (mut context, mut contract) = setup_contract(set_config_lottery(false));

        // deposit storage
        deposit_for_account(&mut context, &mut contract, accounts(2));

        // start a lottery
        start_a_lottery(&mut context, &mut contract, accounts(2));

        let current_lottery_id = contract.data().current_lottery_id;

        // buy ticket 1
        buy_a_ticket(
            &mut context,
            &mut contract,
            accounts(2),
            current_lottery_id,
            vec![1302877],
        );

        // check amount_collected_in_near
        let data2 = contract.data();
        let lottery2 = data2._lotteries.get(&current_lottery_id).unwrap();
        assert_eq!(lottery2.amount_collected_in_near, 1000000000000000000000000);

        // buy the second time
        buy_a_ticket(
            &mut context,
            &mut contract,
            accounts(2),
            current_lottery_id,
            vec![1292877, 1292837],
        );

        let data3 = contract.data();
        let lottery3 = data3._lotteries.get(&current_lottery_id).unwrap();
        assert_eq!(lottery3.amount_collected_in_near, 2999000000000000000000000);

        assert_eq!(
            data3
                ._bracket_tickets_number
                .get(&current_lottery_id)
                .unwrap()
                .keys_as_vector()
                .len(),
            13
        );

        // number of ticket should be 3
        let data_mut = contract.data_mut();
        let buyer = internal_get_account_unwrap_by_contract_data(data_mut, &accounts(2));
        let tickets = buyer
            .internal_get_ticket_ids_per_lottery(&current_lottery_id)
            .unwrap_or(vec![]);
        assert_eq!(tickets.len(), 3);

        // check current tickets  assert_eq!(
        assert_eq!(data_mut.current_ticket_id, 3);
    }

    #[test]
    fn test_claim_tickets() {
        let (mut context, mut contract) = setup_contract(set_config_lottery(false));
        // deposit storage
        deposit_for_account(&mut context, &mut contract, accounts(2));

        // start a lottery
        start_a_lottery(&mut context, &mut contract, accounts(2));

        let current_lottery_id = contract.data().current_lottery_id;
        // buy 4 tickets
        buy_a_ticket(
            &mut context,
            &mut contract,
            accounts(2),
            current_lottery_id,
            vec![1039219, 1106409, 1192039, 1000699],
        );

        // close lottery
        close_lottery(&mut context, &mut contract);

        // draw final number
        contract.data_mut().random_result = 1327419;
        contract.draw_final_number_and_make_lottery_claimable(current_lottery_id, true);

        // fixed update lottery final number
        // view numbers and status for ticket ids
        let tk_numbers =
            contract.view_numbers_and_statuses_for_ticket_ids(vec![0, 1, 2, 3], current_lottery_id);
        assert_eq!(tk_numbers.ticket_numbers.len(), 4);
        assert!(tk_numbers.ticket_numbers.contains(&1192039));
        assert_eq!(tk_numbers.ticket_status.len(), 4);
        println!("Winning number: {}", contract.data().random_result);
        println!("Ticket numbers: {:?}", tk_numbers.ticket_numbers);
        println!("Ticket statuses: {:?}", tk_numbers.ticket_status);

        // view rewards for ticket: ticketId: 3, bracket: 0
        let rewards_for_ticket = contract.view_rewards_for_ticket_id(current_lottery_id, 3, 0);
        // assert_eq!(rewards_for_ticket, 13313333333333333333333);
        println!("rewards_for_ticket: {:?}", rewards_for_ticket);

        // // check rewards per bracket
        let data2 = contract.data();
        let lottery: Lottery = data2._lotteries.get(&current_lottery_id).unwrap();
        let rewards_brackets = lottery.near_per_bracket;
        println!(
            "near_per_bracket: [{},{},{},{},{},{}]",
            rewards_brackets[0],
            rewards_brackets[1],
            rewards_brackets[2],
            rewards_brackets[3],
            rewards_brackets[4],
            rewards_brackets[5]
        );
        assert_eq!(rewards_brackets.len(), 6);
        assert_eq!(rewards_brackets[0], 12647666666666666666666);
        assert_eq!(rewards_brackets[1], 113829000000000000000000);
        assert_eq!(rewards_brackets[2], 0);
        assert_eq!(rewards_brackets[3], 0);
        assert_eq!(rewards_brackets[4], 0);
        assert_eq!(rewards_brackets[5], 0);
        println!("Bracket 1 reward: {}", rewards_brackets[0]);
        println!("Bracket 2 reward: {}", rewards_brackets[1]);
        // number of winners per brackets
        let couting_winners_brackets = lottery.count_winners_per_bracket;
        assert_eq!(couting_winners_brackets.len(), 6);
        // assert_eq!(couting_winners_brackets[0], 3); // 1106409, 1192039, 1000699
        // assert_eq!(couting_winners_brackets[1], 1); // 1039219
        // assert_eq!(couting_winners_brackets[2], 0);
        // assert_eq!(couting_winners_brackets[3], 0);
        // assert_eq!(couting_winners_brackets[4], 0);
        // assert_eq!(couting_winners_brackets[5], 0);

        // view status a list of tickets
        let status_tickets =
            contract.view_numbers_and_statuses_for_ticket_ids(vec![0, 1, 2, 3], current_lottery_id);
        println!("status_tickets: {:?}", status_tickets);

        //Claim tickets
        println!("first_ticket_id: {:?}", lottery.first_ticket_id);
        println!(
            "first_ticket_id_next_lottery: {:?}",
            lottery.first_ticket_id_next_lottery
        );
        contract.claim_tickets(current_lottery_id, vec![0, 1, 2, 3], vec![1, 0, 0, 0]);
        // check rewards summary share share
        let data3 = contract.data();
        let lottery_claim_lottery = data3._lotteries.get(&current_lottery_id).unwrap();
        let operate_fee = (lottery_claim_lottery.amount_collected_in_near
            - lottery_claim_lottery.last_pot_size)
            * data3.config_lottery.operate_fee.0
            / 10000;
        println!("claim_tickets_operate_fee: {}", operate_fee);

        // reserver pool
        let reserver_pool = ((lottery_claim_lottery.amount_collected_in_near - operate_fee)
            * data3.config_lottery.reserve_fee.0)
            / 10000;

        let amount_to_shared = ((lottery_claim_lottery.amount_collected_in_near - operate_fee)
            * (10000 - data3.config_lottery.reserve_fee.0))
            / 10000;

        println!("amount_to_shared: {}", amount_to_shared);
        // there 3 tickets winning at bracket[0], 1: bracket[1], 0: bracket[2], 0: bracket[3], 0: bracket[4], 0: bracket[5]
        // check pending_injection_next_lottery
        assert_eq!(
            data3.pending_injection_next_lottery,
            3642528000000000000000000, //number of near in vauls add for the next turns
        );
        assert_eq!(
            data3.pending_injection_next_lottery + 2, // increase by 1 to match the value.
            amount_to_shared
                - ((couting_winners_brackets[0] * rewards_brackets[0])
                    + (couting_winners_brackets[1] * rewards_brackets[1])
                    + (couting_winners_brackets[2] * rewards_brackets[2])
                    + (couting_winners_brackets[3] * rewards_brackets[3])
                    + (couting_winners_brackets[4] * rewards_brackets[4])
                    + (couting_winners_brackets[5] * rewards_brackets[5]))
                + reserver_pool
        );
        // check total nears divides into each winners and treasury, pending next turns
        assert_eq!(
            (couting_winners_brackets[0] * rewards_brackets[0])
                + (couting_winners_brackets[1] * rewards_brackets[1])
                + (couting_winners_brackets[2] * rewards_brackets[2])
                + (couting_winners_brackets[3] * rewards_brackets[3])
                + (couting_winners_brackets[4] * rewards_brackets[4])
                + (couting_winners_brackets[5] * rewards_brackets[5])
                + data3.pending_injection_next_lottery, // number of Near left without any winners will send treasury
            amount_to_shared + reserver_pool - 2
        );

        //     // check all status is owner of zero address
        let data2 = contract.data();
        for key in data2._tickets.keys_as_vector().iter() {
            let ticket = data2._tickets.get(&key).unwrap();
            assert_eq!(
                ticket.owner,
                AccountId::new_unchecked(ZERO_ADDRESS_WALLET.to_string())
            )
        }

        // view status ticket after claimed
        let status_tickets =
            contract.view_numbers_and_statuses_for_ticket_ids(vec![0, 1, 2, 3], current_lottery_id);
        let view_user_info_for_lottery_id = contract.view_user_info_for_lottery_id(
            accounts(2),
            current_lottery_id,
            Some(0),
            Some(100),
        );
        println!("status_tickets: {:?}", status_tickets);
        println!(
            "view_user_info_for_lottery_id: {:?}",
            view_user_info_for_lottery_id
        );
    }

    /// Test utiles functions
    #[test]
    fn test_create_number_one() {
        let number = create_number_one(6);
        assert_eq!(number, 111111);
    }

    #[test]
    fn test_calculate_total_price_for_bulk_tickets() {
        let final_price = _calculate_total_price_for_bulk_tickets(
            2000,
            1200000000000000000, //1.2 NEAR
            1,
        );
        assert_eq!(final_price, 1200000000000000000);

        // buy 10 tickets per time
        let final_price2 = _calculate_total_price_for_bulk_tickets(
            2000,
            1200000000000000000, //1.2 NEAR
            10,
        );
        assert_eq!(final_price2, 11946000000000000000); //~11.946 NEAR, 0.45% Bulk discount
    }

    #[test]
    fn test_calculate_rewards_for_ticket_id() {
        let (mut context, mut contract) = setup_contract(set_config_lottery(false));

        // deposit storage
        deposit_for_account(&mut context, &mut contract, accounts(2));

        // start a lottery
        start_a_lottery(&mut context, &mut contract, accounts(2));

        let current_lottery_id = contract.data().current_lottery_id;

        // buy ticket 1
        buy_a_ticket(
            &mut context,
            &mut contract,
            accounts(2),
            current_lottery_id,
            vec![1039219],
        );

        // close lottery
        close_lottery(&mut context, &mut contract);

        // draw final number
        testing_env!(context
            .predecessor_account_id(accounts(2))
            .attached_deposit(1)
            .build());
        contract.draw_final_number_and_make_lottery_claimable(current_lottery_id, true);
        //// draw final number
        contract.data_mut().random_result = 1327419;

        // win two number 91, in the front end the string winner should be 327419. without 1.
        let reward = _calculate_rewards_for_ticket_id(contract.data(), current_lottery_id, 0, 1);
        // assert_eq!(reward, 30000000000000000000000); // = 375/10000*(1.2(total pool)-0.24(treasury fee))
        println!("Rewards: {}", reward);
    }

    #[test]
    fn test_close_lottery() {
        let (mut context, mut contract) = setup_contract(set_config_lottery(false));
        // deposit storage
        deposit_for_account(&mut context, &mut contract, accounts(2));

        // start a lottery
        start_a_lottery(&mut context, &mut contract, accounts(2));

        let current_lottery_id = contract.data().current_lottery_id;

        // close lottery
        close_lottery(&mut context, &mut contract);

        let data2 = contract.data();
        let lottery = data2._lotteries.get(&current_lottery_id).unwrap();
        assert_eq!(lottery.status, Status::Close);
        assert_eq!(
            lottery.first_ticket_id_next_lottery,
            data2.current_ticket_id
        );
        assert_eq!(data2.permission_update, PermissionUpdateState::Allow);
    }

    #[test]
    fn test_get_random_number() {
        let (mut context, contract) = setup_contract(set_config_lottery(false));

        testing_env!(context
            .predecessor_account_id(accounts(5))
            .random_seed([
                1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 4, 5, 6, 7, 8, 9, 1, 2, 3, 3, 4, 5, 6, 6, 7, 8, 9,
                1, 2, 4, 5
            ])
            .block_timestamp(111 + 1)
            .build());

        let result = contract.view_random();
        println!("random_number: {:?}", result);
    }

    #[test]
    fn test_get_current_timestamp() {
        let (mut context, contract) = setup_contract(set_config_lottery(false));
        testing_env!(context
            .predecessor_account_id(accounts(5))
            .random_seed([
                1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 4, 5, 6, 7, 8, 9, 1, 2, 3, 3, 4, 5, 6, 6, 7, 8, 9,
                1, 2, 4, 5
            ])
            .block_timestamp(1662562383)
            .build());

        let current_timestamp = contract.get_current_timestamp();
        println!("current_timestamp: {:?}", current_timestamp);
    }

    #[test]
    fn test_random_position() {
        let (mut context, _) = setup_contract(set_config_lottery(false));
        let random_seed: [u8; 32] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 4, 5, 6, 7, 8, 9, 1, 2, 3, 3, 4, 5, 6, 6, 7, 8, 9, 1,
            2, 4, 5,
        ];
        testing_env!(context
            .predecessor_account_id(accounts(5))
            .random_seed(random_seed)
            .block_timestamp(1662562383)
            .build());

        let random_positions = random_position();
        println!("random_positions: {:?}", random_positions);
        assert_eq!(random_positions, [1, 2, 3, 4, 5, 6, 7, 8, 0, 1]);
    }
    #[test]
    fn test_view_user_info_for_lottery_id() {
        let (mut context, mut contract) = setup_contract(set_config_lottery(false));

        // deposit storage
        deposit_for_account(&mut context, &mut contract, accounts(2));
        // start lottery 2
        start_a_lottery(&mut context, &mut contract, accounts(2));
        let current_lottery_id = contract.data().current_lottery_id;
        buy_a_ticket(
            &mut context,
            &mut contract,
            accounts(2),
            current_lottery_id,
            vec![1039291],
        );

        // view user info
        let view_user_info = contract.view_user_info_for_lottery_id(
            accounts(2),
            current_lottery_id,
            Some(0),
            Some(25),
        );

        println!(
            "view user info [test_view_user_info_for_lottery_id]: {:?}",
            view_user_info
        );
        assert_eq!(view_user_info.lottery_id, 1);
        assert_eq!(view_user_info.ticket_numbers.len(), 1);
        assert_eq!(view_user_info.ticket_numbers[0], 1039291);

        assert_eq!(view_user_info.ticket_ids.len(), 1);
        assert_eq!(view_user_info.ticket_ids[0], 0);

        // get view all lotteries
        let view_ticket_all_lotteries =
            contract.view_all_tickets_by_user_in_lottery_id(accounts(2), Some(0), Some(25));
        println!("view_lotteries {:?}", view_ticket_all_lotteries);
        // lottery 1
        let _lottery_first = contract.view_lottery(1);
        println!("view lottery {:?}", contract.view_lottery(1));
    }

    #[test]
    fn test_view_calculate_total_price_for_bulk_tickets_include_discount() {
        let (mut context, mut contract) = setup_contract(set_config_lottery(false));

        // deposit storage
        deposit_for_account(&mut context, &mut contract, accounts(2));

        // start lottery 2
        start_a_lottery(&mut context, &mut contract, accounts(2));

        let current_lottery_id = contract.data().current_lottery_id;
        // buy 1 ticket
        let final_price = contract.calculate_total_price_for_bulk_tickets(current_lottery_id, 1);
        assert_eq!(final_price, 10 * 10u128.pow(23)); //~1.0 NEAR

        // buy 2 ticket
        let final_2_ticket_price =
            contract.calculate_total_price_for_bulk_tickets(current_lottery_id, 10);
        println!("Final price: {:}", final_2_ticket_price);
        assert_eq!(
            final_2_ticket_price,
            (10 * 10u128.pow(23) * 10 * (2000 + 1 - 10)) / 2000
        );
    }

    #[test]
    fn test_view_calculate_total_price_for_bulk_tickets_no_discount() {
        let (mut context, mut contract) = setup_contract(set_config_lottery(false));
        // call owner
        testing_env!(context
            .predecessor_account_id(accounts(0))
            .attached_deposit(1)
            .build());

        // set no discount
        // get current config
        let current_config = contract.get_config();
        println!(
            "min_discount_divisor: {:?}",
            current_config.min_discount_divisor
        );
        assert_eq!(current_config.min_discount_divisor, 0);

        // start lottery
        deposit_for_account(&mut context, &mut contract, accounts(2));

        // start lottery 2
        start_a_lottery(&mut context, &mut contract, accounts(2));

        // get current lottery id
        let current_lottery_id = contract.data().current_lottery_id;

        let final_price = contract.calculate_total_price_for_bulk_tickets(current_lottery_id, 1);
        assert_eq!(final_price, 10 * 10u128.pow(23)); //~1.2 NEAR

        // buy 2 ticket should be the price x number of ticket
        let final_2_ticket_price =
            contract.calculate_total_price_for_bulk_tickets(current_lottery_id, 10);
        assert_eq!(final_2_ticket_price, 9955000000000000000000000);
    }

    #[test]
    #[should_panic(expected = "E38: The lottery is running. Could not change any configuration.")]
    fn test_permission_running() {
        let (mut context, mut contract) = setup_contract(set_config_lottery(false));
        // deposit storage
        deposit_for_account(&mut context, &mut contract, accounts(2));

        // start lottery 2
        start_a_lottery(&mut context, &mut contract, accounts(2));
        start_a_lottery(&mut context, &mut contract, accounts(2));
    }

    #[test]
    fn test_view_lotteries() {
        let (mut context, mut contract) = setup_contract(set_config_lottery(false));

        // deposit storage
        deposit_for_account(&mut context, &mut contract, accounts(2));
        // start lottery 2
        start_a_lottery(&mut context, &mut contract, accounts(2));

        // view current
        let current_lottery_id = contract.data().current_lottery_id;
        buy_a_ticket(
            &mut context,
            &mut contract,
            accounts(2),
            current_lottery_id,
            vec![1039219],
        );

        let data = contract.data();
        // check
        let lotteries = contract.view_lotteries(Some(0), Some(1000));
        assert_eq!(1, lotteries.len());
        let lottery = lotteries.get(0).unwrap();
        assert_eq!(1, lottery.lottery_id);
        assert_eq!(Status::Open, lottery.status);
        assert_eq!(0, lottery.start_time);
        assert_eq!(162615612345679, lottery.end_time);
        assert_eq!(
            U128(1000000000000000000000000),
            data.config_lottery.price_ticket_in_near
        );
        assert_eq!(U128(2000), data.config_lottery.discount_divisor);
        assert_eq!(6, data.config_lottery.rewards_breakdown.len());
        assert_eq!(U128(2000), data.config_lottery.reserve_fee);
        assert_eq!(vec![0, 0, 0, 0, 0, 0], lottery.near_per_bracket);
        assert_eq!(vec![0, 0, 0, 0, 0, 0], lottery.count_winners_per_bracket);
        assert_eq!(0, lottery.first_ticket_id);
        assert_eq!(1, lottery.first_ticket_id_next_lottery);
        assert_eq!(1000000000000000000000000, lottery.amount_collected_in_near);
        assert_eq!(0, lottery.final_number);
    }
}
