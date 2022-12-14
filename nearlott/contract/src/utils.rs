use crate::callback::ext_ft_contract;
use crate::gas::GAS_FOR_FT_TRANSFER;

use crate::*;

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

/**
* @notice Create a number generated from number of '1'
* @param sequence: the time repeats number generator
* For example: 1, 11, 111, 1111, 1111...etc
*/
pub fn create_number_one(sequence: u32) -> u32 {
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
pub fn _calculate_total_price_for_bulk_tickets(
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
pub fn _calculate_number_of_tickets(
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
pub fn _calculate_rewards_for_ticket_id(
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
    0
}

/**
 * @notice Request randomness from a user-provided seed
 * @param _seed: seed provided by the NearLott lottery
 */
pub fn get_random_number() -> u32 {
    // generate 15 number position with random position from [1..9]
    let random: Vec<u8> = random_position();
    assert!(random.len() >= 10, "{}", ERR37_NOT_ENOUGH_RANDOM_NUMBERS);
    let rand_array_str = format!("{:?}", &random);
    // Specific position to get values. Random_seeds default return to 32 number in a vector<u8>.
    let rand_array: Vec<u8> = random
        .into_iter()
        .map(|x| *env::random_seed().get(x as usize).unwrap())
        .collect();
    // convert so string
    let randomness_instr = rand_array
        .into_iter()
        .map(|x| x.to_string())
        .collect::<String>();
    // convert to u64 to prepare for final number
    let randomness = randomness_instr
        .parse::<u128>()
        .expect(ERR34_RANDOM_NUMBER_INVALID);

    // determine final number
    let win_number = (1000000 + (randomness % 1000000)) as u32;

    // write log
    env::log_str(
        &json!({
            "type": "draw_final_number_process",
            "params": {
                "block_height": env::block_height(),
                "vrf_numbers": env::random_seed(),
                "ten_selected_positions": rand_array_str,
                "number_generated_by_ten_positions":randomness_instr,
                "current_timestamp": env::block_timestamp(),
                "logic": format!("(1000000 + ({} % 1000000))", &randomness),
                "final_number":  &win_number
            }
        })
        .to_string(),
    );
    // return
    win_number
}

/**
 * @notice Random a number from 1..9
 */
pub fn random_position() -> Vec<u8> {
    let positions = env::random_seed();
    if positions.len() > 10 {
        let slice: Vec<u8> = positions[0..10].iter().map(|x| x % 9).collect();
        return slice;
    }
    positions
}

pub fn extract_data(value: Option<U128>) -> u128 {
    if let Some(amount) = value.map(|a| a.0) {
        amount
    } else {
        0
    }
}

#[allow(unused)]
pub fn unordered_map_pagination<K, VV, V>(
    m: &UnorderedMap<K, VV>,
    from_index: Option<u64>,
    limit: Option<u64>,
) -> Vec<(K, V)>
where
    K: BorshSerialize + BorshDeserialize,
    VV: BorshSerialize + BorshDeserialize,
    V: From<VV>,
{
    let keys = m.keys_as_vector();
    let values = m.values_as_vector();
    let from_index = from_index.unwrap_or(0);
    let limit = limit.unwrap_or(keys.len());
    (from_index..std::cmp::min(keys.len(), from_index + limit))
        .map(|index| (keys.get(index).unwrap(), values.get(index).unwrap().into()))
        .collect()
}

pub fn internal_get_account_unwrap_by_contract_data(
    contract_data: &mut ContractData,
    account_id: &AccountId,
) -> Account {
    let account = contract_data.accounts.get(account_id).map(|o| o.into());
    account.expect(ERR42_ACCOUNT_NO_EXISTING)
}

pub fn internal_set_storage_data(
    data: &mut ContractData,
    account_id: &AccountId,
    mut storage: Storage,
) {
    if storage.storage_tracker.bytes_added >= storage.storage_tracker.bytes_released {
        let extra_bytes_used =
            storage.storage_tracker.bytes_added - storage.storage_tracker.bytes_released;
        storage.used_bytes += extra_bytes_used;

        // check assert
        let storage_balance_needed = Balance::from(storage.used_bytes) * env::storage_byte_cost();
        assert!(
            storage_balance_needed <= storage.storage_balance,
            "Not enough storage balance"
        );
    } else {
        let bytes_released =
            storage.storage_tracker.bytes_released - storage.storage_tracker.bytes_added;
        assert!(
            storage.used_bytes >= bytes_released,
            "Internal storage accounting bug"
        );
        storage.used_bytes -= bytes_released;
    }
    storage.storage_tracker.bytes_released = 0;
    storage.storage_tracker.bytes_added = 0;
    data.storage.insert(account_id, &storage.into());
}

pub fn internal_set_account_data(
    data: &mut ContractData,
    account_id: &AccountId,
    mut account: Account,
) {
    let mut storage: Storage = data
        .storage
        .get(account_id)
        .map(|o| o.into())
        .expect("Storage for account is missing");
    storage
        .storage_tracker
        .consume(&mut account.storage_tracker);
    storage.storage_tracker.start();
    data.accounts.insert(account_id, &account.into());
    storage.storage_tracker.stop();
    internal_set_storage_data(data, account_id, storage);
}
