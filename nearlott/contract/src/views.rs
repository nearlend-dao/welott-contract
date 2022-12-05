use crate::config::ConfigContractData;
use crate::*;
use near_sdk::serde::{Deserialize, Serialize};
const PAGINATION_SIZE: u32 = 10;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct LotteryUserData {
    pub final_number: u32,
    pub lottery_id: LotteryId,
    pub ticket_numbers: Vec<u32>,
    pub ticket_ids: Vec<TicketId>,
    pub cursor: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct LotteryNumberAndStatusData {
    pub ticket_numbers: Vec<u32>,
    pub ticket_status: Vec<u8>, //[0: NA, 1: Claimed, 2: Claimable, 3: Lose] // Nên chuyển thành enum
}

/// User account on this contract
impl Default for LotteryUserData {
    fn default() -> Self {
        Self {
            final_number: 0,
            lottery_id: 0,
            ticket_numbers: vec![],
            ticket_ids: vec![],
            cursor: 0,
        }
    }
}

#[near_bindgen]
impl NearLott {
    /**
     * @notice Calculate price of a set of tickets
     * @param _discountDivisor: divisor for the discount
     * @param _price_ticket price of a ticket (in NEAR)
     * @param _number_tickets number of tickets to buy
     */
    pub fn calculate_total_price_for_bulk_tickets(
        &self,
        _lottery_id: LotteryId,
        _number_tickets: u128,
    ) -> u128 {
        // check existing lottery
        let data = self.data();
        let lottery = data
            ._lotteries
            .get(&_lottery_id)
            .expect(ERR1_NOT_EXISTING_LOTTERY);

        // check number of tickets
        assert_ne!(_number_tickets, 0, "{}", ERR7_NUMBER_TICKETS_ZERO);

        // get discount divisor and ticket price
        let discount_divisor = lottery.discount_divisor;
        let ticket_price = lottery.price_ticket_in_near;

        return _calculate_total_price_for_bulk_tickets(
            discount_divisor,
            ticket_price,
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

    pub fn view_lotteries(&self, _cursor: Option<u64>, _size: Option<u64>) -> Vec<Lottery> {
        let values = self.data()._lotteries.values_as_vector();
        let from_index = _cursor.unwrap_or(0);
        let limit = _size.unwrap_or(values.len());
        (from_index..std::cmp::min(values.len(), from_index + limit))
            .map(|index| values.get(values.len() - index - 1).unwrap().into())
            .collect()
    }

    /**
     * @notice View lottery information
     * @param _lottery_id: lottery id
     */
    pub fn view_number_tickets_per_lottery(&self, _lottery_id: LotteryId) -> u32 {
        let data = self.data();
        let lottery = data
            ._lotteries
            .get(&_lottery_id)
            .expect(ERR1_NOT_EXISTING_LOTTERY);
        lottery.first_ticket_id_next_lottery - lottery.first_ticket_id
    }

    /**
     * @notice: Get detail the running lottery
     */
    pub fn view_current_lottery_running(&self) -> Lottery {
        let current_lottery_id = self.data().current_lottery_id;
        let lottery = self
            .data()
            ._lotteries
            .get(&current_lottery_id)
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
     * @notice View user ticket ids, numbers, and statuses of user for a all lottery joined
     * @param _user: user address
     * @param _cursor: cursor to start where to retrieve the tickets
     * @param _size: the number of tickets to retrieve
     */
    pub fn view_all_lotteries_by_user(
        &self,
        _user: AccountId,
        _cursor: Option<u32>,
        _size: Option<u32>,
    ) -> Vec<LotteryUserData> {
        let account = self.internal_unwrap_account(&_user);
        let lottery_key_ids = account.tickets.keys();

        lottery_key_ids
            .map(|&lottery_id| {
                self.view_user_info_for_lottery_id(_user.clone(), lottery_id, _cursor, _size)
            })
            .collect()
    }

    /**
     * @notice View user ticket ids, numbers of user for a given lottery
     * @param _user: user address
     * @param _lottery_id: lottery id
     * @param _cursor: cursor to start where to retrieve the tickets
     * @param _size: the number of tickets to retrieve
     */
    pub fn view_user_info_for_lottery_id(
        &self,
        _user: AccountId,
        _lottery_id: LotteryId,
        _cursor: Option<u32>,
        _size: Option<u32>,
    ) -> LotteryUserData {
        let mut empty_user_info = LotteryUserData {
            final_number: 0,
            lottery_id: 0,
            ticket_numbers: vec![],
            ticket_ids: vec![],
            cursor: _cursor.unwrap_or(0),
        };

        // get lottery status
        let lottery = self.data()._lotteries.get(&_lottery_id);
        if lottery.is_none() {
            return empty_user_info;
        }

        // assign data
        let current_lottery = lottery.unwrap();
        empty_user_info.final_number = current_lottery.final_number;
        empty_user_info.lottery_id = current_lottery.lottery_id;

        // check any ticket ids by this lottery id
        // if there is no tickets. Return as a default value
        let mut account = self.internal_unwrap_account(&_user);
        let user_tickets = account.internal_get_ticket_id_per_lottery_or_default(&_lottery_id);
        if user_tickets.len() == 0 {
            return empty_user_info;
        }

        let lottery_ticket_ids: Vec<TicketId> = user_tickets
            .iter()
            .map(|&x| x)
            .skip(_cursor.unwrap_or(0) as usize)
            .take(_size.unwrap_or(PAGINATION_SIZE) as usize)
            .collect();

        let ticket_numbers: Vec<u32> = self
            .data()
            ._tickets
            .iter()
            .filter(|x| lottery_ticket_ids.contains(&x.0))
            .map(|x| x.1.number)
            .collect();

        LotteryUserData {
            final_number: current_lottery.final_number,
            lottery_id: current_lottery.lottery_id,
            ticket_ids: lottery_ticket_ids,
            ticket_numbers,
            cursor: _cursor.unwrap_or(0),
        }
    }

    /**
     * @notice View ticker statuses and numbers for an array of ticket ids
     * @param _ticketIds: array of _ticketId
     */
    pub fn view_numbers_and_statuses_for_ticket_ids(
        &self,
        _ticket_ids: Vec<TicketId>,
        _lottery_id: LotteryId,
    ) -> LotteryNumberAndStatusData {
        let length = _ticket_ids.len();
        let mut ticket_numbers = vec![0; length];
        let mut ticket_statuses = vec![0; length]; // 0: NA

        let _brackets = vec![5, 4, 3, 2, 1, 0];
        for i in 0..length {
            let ticket_number = self
                .data()
                ._tickets
                .get(&_ticket_ids[i as usize])
                .unwrap_or(Ticket {
                    number: 0,
                    owner: AccountId::new_unchecked(ZERO_ADDRESS_WALLET.to_string()),
                });

            if ticket_number.owner == AccountId::new_unchecked(ZERO_ADDRESS_WALLET.to_string()) {
                ticket_statuses[i as usize] = 1; // Claimed
            } else {
                let mut rewards_per_bracket = 0;
                for bracket_position in 0.._brackets.len() {
                    let reward_for_ticket_id = _calculate_rewards_for_ticket_id(
                        self.data(),
                        _lottery_id,
                        _ticket_ids[i as usize],
                        _brackets[bracket_position],
                    );
                    rewards_per_bracket += reward_for_ticket_id;
                }
                if rewards_per_bracket > 0 {
                    ticket_statuses[i as usize] = 2; // Claimable
                } else {
                    ticket_statuses[i as usize] = 3; // Lose
                }
            }
            ticket_numbers[i as usize] = ticket_number.number;
        }

        LotteryNumberAndStatusData {
            ticket_numbers,
            ticket_status: ticket_statuses,
        }
    }

    /**
     * Using for unit-test
     */
    #[private]
    pub fn view_random(&self) -> u32 {
        get_random_number()
    }

    /**
     * View the current lottery's  final winning number
     */
    pub fn view_random_result(&self) -> u32 {
        let data = self.data();
        data.random_result as u32
    }

    /**
     * Get current blockt imestamp running in Near blockchain
     */
    pub fn get_current_timestamp(&self) -> u64 {
        env::block_timestamp()
    }

    /**
     * Get current config
     */
    pub fn get_config(&self) -> ConfigContractData {
        let config = self._get_config();
        return config;
    }
}
