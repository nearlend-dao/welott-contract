use crate::callback::ext_ft_contract;
use crate::gas::GAS_FOR_FT_TRANSFER;
use crate::*;
use rand::Rng; // 0.8.0

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
    (_price_ticket * _number_ticket * (_discount_divisor + 1 - _number_ticket)) / _discount_divisor
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
    let mut rng = rand::thread_rng();
    let random1 = rng.gen_range(1..=9);
    let random2 = rng.gen_range(1..=9);
    let random3 = rng.gen_range(1..=9);
    let random4 = rng.gen_range(1..=9);
    let random5 = rng.gen_range(1..=9);
    let random6 = rng.gen_range(1..=9);
    let random7 = rng.gen_range(1..=9);
    let random8 = rng.gen_range(1..=9);
    let random9 = rng.gen_range(1..=9);
    let random10 = rng.gen_range(1..=9);
    let random11 = rng.gen_range(1..=9);
    let random12 = rng.gen_range(1..=9);
    let random13 = rng.gen_range(1..=9);
    let random14 = rng.gen_range(1..=9);
    let random15 = rng.gen_range(1..=9);

    let rand_array = [
        *env::random_seed().get(random1).unwrap(),
        *env::random_seed().get(random2).unwrap(),
        *env::random_seed().get(random3).unwrap(),
        *env::random_seed().get(random4).unwrap(),
        *env::random_seed().get(random5).unwrap(),
        *env::random_seed().get(random6).unwrap(),
        *env::random_seed().get(random7).unwrap(),
        *env::random_seed().get(random8).unwrap(),
        *env::random_seed().get(random9).unwrap(),
        *env::random_seed().get(random10).unwrap(),
        *env::random_seed().get(random11).unwrap(),
        *env::random_seed().get(random12).unwrap(),
        *env::random_seed().get(random13).unwrap(),
        *env::random_seed().get(random14).unwrap(),
        *env::random_seed().get(random15).unwrap(),
    ];

    let randomness_instr = format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        rand_array[1],
        rand_array[2],
        rand_array[3],
        rand_array[4],
        rand_array[5],
        rand_array[6],
        rand_array[7],
        rand_array[8],
        rand_array[9],
        rand_array[10],
        rand_array[11],
        rand_array[12],
        rand_array[13],
        rand_array[14],
        rand_array[0]
    )
    .to_string();

    let randomness = randomness_instr
        .parse::<u64>()
        .expect(ERR34_RANDOM_NUMBER_INVALID);

    let win_number = (1000000 + (randomness % 1000000)) as u32;

    win_number
}
