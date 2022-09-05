use crate::*;
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct LotteryUserData {
    pub lottery_ticket_ids: Vec<TicketId>,
    pub ticket_numbers: Vec<u32>,
    pub ticket_status: Vec<bool>,
    pub cursor: u32,
}

/// User account on this contract
impl Default for LotteryUserData {
    fn default() -> Self {
        Self {
            lottery_ticket_ids: vec![],
            ticket_numbers: vec![],
            ticket_status: vec![],
            cursor: 0,
        }
    }
}

impl NearLott {
    /**
     * @notice Calculate price of a set of tickets
     * @param _discountDivisor: divisor for the discount
     * @param _price_ticket price of a ticket (in NEAR)
     * @param _number_tickets number of tickets to buy
     */
    pub fn calculate_total_price_for_bulk_tickets(
        &mut self,
        _discount_divisor: u128,
        _price_ticket: u128,
        _number_tickets: u128,
    ) -> u128 {
        let data = self.data();
        assert!(
            _discount_divisor >= data.min_discount_divisor,
            "{}",
            ERR6_MIN_DISCOUNT_DIVISOR
        );

        assert_ne!(_number_tickets, 0, "{}", ERR7_NUMBER_TICKETS_ZERO);

        return _calculate_total_price_for_bulk_tickets(
            _discount_divisor,
            _price_ticket,
            _number_tickets,
        );
    }
    /**
     * @notice View latest lottery id
     */
    pub fn view_latest_lottery_id(&self) -> u32 {
        self.data().current_lottery_id
    }

    /**
     * @notice View lottery information
     * @param _lottery_id: lottery id
     */
    pub fn view_lottery(&self, _lottery_id: LotteryId) -> Lottery {
        let lottery = self
            .data()
            ._lotteries
            .get(&_lottery_id)
            .expect(ERR1_NOT_EXISTING_LOTTERY);
        lottery
    }

    /**
     * @notice View rewards for a given ticket, providing a bracket, and lottery id
     * @dev Computations are mostly off chain. This is used to verify a ticket!
     * @param _lottery_id: lottery id
     * @param _ticket_id: ticket id
     * @param _bracket: bracket for the ticketId to verify the claim and calculate rewards
     */
    pub fn view_rewards_for_ticket_id(
        &self,
        _lottery_id: LotteryId,
        _ticket_id: TicketId,
        _bracket: BracketPosition,
    ) -> u128 {
        // Check lottery is in claimable status
        let data = self.data();
        let lottery = data
            ._lotteries
            .get(&_lottery_id)
            .expect(ERR1_NOT_EXISTING_LOTTERY);
        if lottery.status != Status::Claimable {
            return 0;
        }

        // Check ticketId is within range
        if lottery.first_ticket_id_next_lottery < _ticket_id
            && lottery.first_ticket_id >= _ticket_id
        {
            return 0;
        }

        return _calculate_rewards_for_ticket_id(data, _lottery_id, _ticket_id, _bracket);
    }

    /**
     * @notice View user ticket ids, numbers, and statuses of user for a given lottery
     * @param _user: user address
     * @param _lottery_id: lottery id
     * @param _cursor: cursor to start where to retrieve the tickets
     * @param _size: the number of tickets to retrieve
     */
    pub fn view_user_info_for_lottery_id(
        &self,
        _user: AccountId,
        _lottery_id: LotteryId,
        _cursor: u32,
        _size: u32,
    ) -> LotteryUserData {
        let mut length: u32 = _size;
        let lotteries_user_tickets = self
            .data()
            ._user_ticket_ids_per_lottery_id
            .get(&_user)
            .expect(ERR4_NOT_EXISTING_LOTTERIES_PER_USER);
        let tickets_in_a_lottery = lotteries_user_tickets
            .get(&_lottery_id)
            .expect(ERR1_NOT_EXISTING_LOTTERY);

        let number_tickets_bought_at_lottery_id = tickets_in_a_lottery.len() as u32;
        if length > (number_tickets_bought_at_lottery_id - _cursor) {
            length = number_tickets_bought_at_lottery_id - _cursor;
        }

        let mut lottery_ticket_ids = vec![];
        let mut ticket_numbers = vec![];
        let mut ticket_statuses = vec![];
        for i in 0..length {
            lottery_ticket_ids[i as usize] = tickets_in_a_lottery[(i + _cursor) as usize];

            let ticket_number = self
                .data()
                ._tickets
                .get(&lottery_ticket_ids[i as usize])
                .unwrap_or(Ticket {
                    number: 0,
                    owner: AccountId::new_unchecked("no_account".to_string()),
                });
            ticket_numbers[i as usize] = ticket_number.number;

            if ticket_number.owner == AccountId::new_unchecked("0".to_string()) {
                ticket_statuses[i as usize] = true;
            } else {
                ticket_statuses[i as usize] = false;
            }
        }

        LotteryUserData {
            lottery_ticket_ids: lottery_ticket_ids,
            ticket_numbers: ticket_numbers,
            ticket_status: ticket_statuses,
            cursor: _cursor,
        }
    }

    pub fn view_random(&self) -> u32 {
        get_random_number()
    }

    pub fn view_random_result(&self) -> u32 {
        let data = self.data();
        data.random_result as u32
    }

    pub fn get_current_timestamp(&self) -> u64 {
        env::block_timestamp()
    }
}