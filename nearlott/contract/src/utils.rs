use crate::callback::ext_ft_contract;
use crate::gas::GAS_FOR_FT_TRANSFER;
use near_sdk::StorageUsage;

use crate::*;
// use rand::Rng; // 0.8.0

use near_sdk::AccountId;

impl NearLott {
    /**
     * @notice It allows the admin to recover wrong tokens sent to the contract
     * @param _token_address: the address of the token to withdraw
     * @param _token_amount: the number of token amount to withdraw
     * @dev Only callable by owner.
     */
    pub fn recover_wrong_tokens(&self, _token_address: AccountId, _token_amount: u128) {
        self.assert_owner_calling();

        let data = self.data();
        let sender_id = data.owner_id.clone();

        ext_ft_contract::ft_transfer(
            sender_id,
            _token_amount.into(),
            None,
            _token_address,
            1, // one yocto near
            GAS_FOR_FT_TRANSFER,
        );
    }
}

pub(crate) fn to_sec(timestamp: Timestamp) -> TimestampSec {
    (timestamp / 10u64.pow(9)) as u32
}

/**
* @notice Create a number generrated from number of '1'
* @param sequence: the time repeats number generator
* For example: 1, 11, 111, 1111, 1111...etc
*/
pub(crate) fn create_number_one(sequence: u32) -> u32 {
    (1..=sequence)
        .into_iter()
        .map(|_| '1')
        .collect::<String>()
        .parse::<u32>()
        .expect(ERR36_STRING_NUMBER_INVALID)
}

/**
 * @notice Calculate final price for bulk of tickets
 * @param _discount_divisor: divisor for the discount (the smaller it is, the greater the discount is)
 * @param _price_ticket: price of a ticket
 * @param _number_ticket: number of tickets purchased
 */
pub(crate) fn _calculate_total_price_for_bulk_tickets(
    _discount_divisor: u128,
    _price_ticket: u128,
    _number_ticket: u128,
) -> u128 {
    if _discount_divisor == 0 {
        return _price_ticket * _number_ticket;
    }
    (_price_ticket * _number_ticket * (_discount_divisor + 1 - _number_ticket)) / _discount_divisor
}

/**
 * @notice Calculate the number of ticket in a lottery id
 * @param _discount_divisor: divisor for the discount (the smaller it is, the greater the discount is)
 * @param _price_ticket: price of a ticket
 */
pub(crate) fn _calculate_number_of_tickets(
    _number_tickets_per_lottery: u128,
    near_per_bracket: u128,
) -> u128 {
    _number_tickets_per_lottery / near_per_bracket
}

/**
 * @notice Calculate rewards for a given ticket
 * @param _lottery_id: lottery id
 * @param _ticket_id: ticket id
 * @param _bracket: bracket for the ticketId to verify the claim and calculate rewards
 */
pub(crate) fn _calculate_rewards_for_ticket_id(
    data: &ContractData,
    _lottery_id: LotteryId,
    _ticket_id: TicketId,
    _bracket: BracketPosition,
) -> u128 {
    // Retrieve the winning number combination
    let lottery: Lottery = data
        ._lotteries
        .get(&_lottery_id)
        .expect(ERR1_NOT_EXISTING_LOTTERY);
    let user_number = lottery.final_number;

    // Retrieve the user number combination from the ticketId
    let ticket = data
        ._tickets
        .get(&_ticket_id)
        .expect(ERR2_NOT_EXISTING_TICKET);
    let winning_ticket_number = ticket.number;

    // Apply transformation to verify the claim provided by the user is true
    let bracket_number = data
        ._bracket_calculator
        .get(&_bracket)
        .expect(ERR3_NOT_EXISTING_BRACKET);
    let transformed_winning_number =
        bracket_number + (winning_ticket_number % (10u32.pow(_bracket + 1)));
    let transformed_user_number = bracket_number + (user_number % (10u32.pow(_bracket + 1)));
    // Confirm that the two transformed numbers are the same, if not throw
    if transformed_winning_number == transformed_user_number {
        let lottery_brackets = lottery.near_per_bracket;
        return lottery_brackets[_bracket as usize];
    }
    return 0;
}

/**
 * @notice Request randomness from a user-provided seed
 * @param _seed: seed provided by the NearLott lottery
 */
pub(crate) fn get_random_number() -> u32 {
    // generate 15 number position with random position from [1..9]
    let random: Vec<u8> = random_position();
    assert!(random.len() >= 10, "{}", ERR37_NOT_ENOUGH_RANDOM_NUMBERS);
    // Specific position to get values. Random_seeds defauls return to 32 number in a vector<u8>.
    let rand_array: Vec<u8> = random
        .into_iter()
        .map(|x| *env::random_seed().get(x as usize).unwrap())
        .collect();
    // convert so tring
    let randomness_instr = rand_array
        .into_iter()
        .map(|x| x.to_string())
        .collect::<String>();
    // comvert to u64 to prepare for final number
    let randomness = randomness_instr
        .parse::<u128>()
        .expect(ERR34_RANDOM_NUMBER_INVALID);

    // determine final number
    let win_number = (1000000 + (randomness % 1000000)) as u32;

    // return
    win_number
}

/**
 * @notice Random a number from 1..9
 */
pub(crate) fn random_position() -> Vec<u8> {
    let positions = env::random_seed();
    if positions.len() > 10 {
        let slice: Vec<u8> = positions[0..10].iter().map(|x| x % 9).collect();
        return slice;
    }
    positions
}

/// Asserts there is sufficient amount of $NEAR to cover storage usage.
pub(crate) fn assert_storage_usage_data(data: &ContractData) {
    let deposited = data
        ._storage_deposits
        .get(&env::predecessor_account_id())
        .unwrap_or(0);
    assert!(
        account_storage_usage_data(data) <= deposited,
        "{}",
        ERR32_INSUFFICIENT_STORAGE
    );
}

/// Asserts there is sufficient amount of $NEAR to cover storage usage.
pub(crate) fn assert_estimate_storage_usage_data(data: &ContractData, _number_of_ticket: u64) {
    let deposited = data
        ._storage_deposits
        .get(&env::predecessor_account_id())
        .unwrap_or(0);
    let usage = account_storage_usage_data(data);
    let estimate_storage = estimate_account_storage_usage_data(_number_of_ticket);
    assert!(
        deposited > (usage + estimate_storage),
        "{}. Deposited: {}, used: {}, estimate nescessray NEAR should be deposited to cover fee: {}",
        ERR32_INSUFFICIENT_STORAGE,
        deposited,
        usage,
        estimate_storage
    );
}

/// Returns amount of $NEAR necessary to cover storage used by this data structure.
pub(crate) fn account_storage_usage_data(data: &ContractData) -> Balance {
    let account_id = env::predecessor_account_id();
    let user_lotteries = data
        ._user_ticket_ids_per_lottery_id
        .get(&account_id)
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

/// Estimate the storage need for number of tickets
pub(crate) fn estimate_account_storage_usage_data(number_of_tickets: u64) -> Balance {
    // number of lotteries x (each lottery including: list of ticket numbers and 4 bytes)
    let storage_use_store_lotteries: StorageUsage = 1 * (U32_STORAGE * number_of_tickets);

    // storage using for list of _number_tickers_per_lottery_id
    let storage_number_of_tickets_elements_numbers =
        number_of_tickets * (U32_STORAGE + U128_STORAGE * 6);

    storage_use_store_lotteries as u128 + storage_number_of_tickets_elements_numbers as u128
}

pub(crate) fn extract_data(value: Option<U128>) -> u128 {
    let value_in_128 = if let Some(amount) = value.map(|a| a.0) {
        amount
    } else {
        0
    };
    value_in_128
}
