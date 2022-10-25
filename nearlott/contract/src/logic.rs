use crate::*;

pub const ZERO_ADDRESS_WALLET: &str = "0.near";

#[near_bindgen]
impl NearLott {
    /**
     * @notice Start the lottery
     * @dev Callable by operator
     * @param _end_time: endTime of the lottery
     * @param _price_ticket_in_near: price of a ticket in NEAR
     * @param _discount_divisor: the divisor to calculate the discount magnitude for bulks
     * @param _rewards_breakdown: breakdown of rewards per bracket (must sum to 10,000)
     * @param _treasury_fee: treasury fee (10,000 = 100%, 100 = 1%)
     */
    #[payable]
    pub fn start_lottery(
        &mut self,
        _end_time: Timestamp,
        _price_ticket_in_near: Option<U128>,
        _discount_divisor: Option<U128>,
        _rewards_breakdown: Vec<u128>,
        _treasury_fee: Option<U128>,
    ) {
        self.assert_one_yoctor();
        self.assert_operator_calling();
        self.assert_contract_running();
        self.assert_lottery_running();

        let mut data = self.data_mut();
        // after 4 hours - 5 minutes since now to  4 days + 5 minutes

        // extract data
        let price_ticket_in_near = extract_data(_price_ticket_in_near);
        let discount_divisor = extract_data(_discount_divisor);
        let treasury_fee = extract_data(_treasury_fee);

        assert!(
            discount_divisor >= data.min_discount_divisor,
            "{}",
            ERR13_LOTTERY_DISCOUNT_DIVISOR_TOO_LOW
        );

        assert!(
            treasury_fee <= data.max_treasury_fee,
            "{}",
            ERR15_LOTTERY_OVER_TREASURY_FEE
        );

        let sum_rewards: u128 = _rewards_breakdown.iter().sum();
        assert_eq!(sum_rewards, 10000, "{}", ERR14_LOTTERY_OVER_RANGE_REWARDS);

        let next_lottery_id = data.current_lottery_id + 1;
        data.current_lottery_id = next_lottery_id;
        data.permission_update = PermissionUpdateState::Disallow;
        data._lotteries.insert(
            &next_lottery_id,
            &Lottery {
                lottery_id: data.current_lottery_id,
                status: Status::Open,
                start_time: env::block_timestamp(),
                end_time: _end_time,
                price_ticket_in_near,
                discount_divisor,
                rewards_breakdown: _rewards_breakdown,
                treasury_fee,
                near_per_bracket: vec![0, 0, 0, 0, 0, 0],
                count_winners_per_bracket: vec![0, 0, 0, 0, 0, 0],
                first_ticket_id: data.current_ticket_id,
                first_ticket_id_next_lottery: data.current_ticket_id,
                amount_collected_in_near: data.pending_injection_next_lottery,
                final_number: 0,
            },
        );

        env::log_str(
            &json!({
                "type": "start_lottery",
                "params": {
                    "current_lottery_id": next_lottery_id,
                    "start_time":  env::block_timestamp(),
                    "end_time": _end_time,
                    "price_ticket_in_near": _price_ticket_in_near,
                    "first_ticket_id": data.current_ticket_id,
                    "first_ticket_id_next_lottery": data.current_ticket_id,
                    "pending_injection_next_lottery": U128(data.pending_injection_next_lottery),
                    "_discount_divisor": _discount_divisor,
                }
            })
            .to_string(),
        );

        data.pending_injection_next_lottery = 0;
    }
    /**
     * @notice Draw the final number, calculate reward in NEAR per group, and make lottery claimable
     * @param _lotteryId: lottery id
     * @param _autoInjection: re-injects funds into next lottery (vs. withdrawing all)
     * @dev Callable by operator
     */
    #[payable]
    pub fn draw_final_number_and_make_lottery_claimable(
        &mut self,
        _lottery_id: LotteryId,
        _auto_injection: bool,
    ) {
        self.assert_one_yoctor();
        self.assert_operator_calling();
        self.assert_contract_running();

        let data = self.data_mut();
        let mut lottery = data
            ._lotteries
            .get(&_lottery_id)
            .expect(ERR1_NOT_EXISTING_LOTTERY);
        assert_eq!(
            lottery.status,
            Status::Close,
            "{}",
            ERR30_LOTTERY_IS_NOT_CLOSE
        );
        //  genrate winning number from env:seed
        // let final_number = get_random_number();
        let final_number = 1327419u32;
        data.random_result = final_number;

        // Calculate the finalNumber based on the randomResult generated
        let _final_number = data.random_result as u32;
        // Initialize a number to count addresses in the previous bracket
        let mut _number_addresses_in_previous_bracket: u128 = 0;

        // Calculate the amount to share post-treasury fee
        let _amount_to_share_to_winners =
            (lottery.amount_collected_in_near * (10000 - lottery.treasury_fee)) / 10000;
        // Initializes the amount to withdraw to treasury
        let mut _amount_to_withdraw_to_treasury: u128 = 0;

        if lottery.first_ticket_id_next_lottery - lottery.first_ticket_id > 0 {
            let number_tickets_per_lottery = data
                ._number_tickers_per_lottery_id
                .get(&_lottery_id)
                .expect(ERR19_LOTTERY_NO_TICKERS_NUMBERS);

            // Calculate prizes in NEAR for each bracket by starting from the highest one
            for i in 0..6 {
                let j = 5 - i;

                let bracket_number = data
                    ._bracket_calculator
                    .get(&j)
                    .expect(ERR3_NOT_EXISTING_BRACKET);
                let _transformed_winning_number =
                    bracket_number + (_final_number % (10u32.pow(j + 1)));

                let number_ticket_in_winning_number = number_tickets_per_lottery
                    .get(&_transformed_winning_number)
                    .unwrap_or(0);

                lottery.count_winners_per_bracket[j as usize] =
                    number_ticket_in_winning_number - _number_addresses_in_previous_bracket;

                // A. If number of users for this _bracket number is superior to 0
                if (number_ticket_in_winning_number - _number_addresses_in_previous_bracket) != 0 {
                    // B. If rewards at this bracket are > 0, calculate, else, report the numberAddresses from previous bracket
                    // rewardsBreakdown / total (10000) * amount_to_shared_to_winner / (total bracket winner - previous bracket received. Winner lower bracket does not calculate in higher bracket
                    if lottery.rewards_breakdown[j as usize] != 0 {
                        lottery.near_per_bracket[j as usize] =
                            ((lottery.rewards_breakdown[j as usize] * _amount_to_share_to_winners)
                                / (number_ticket_in_winning_number
                                    - _number_addresses_in_previous_bracket))
                                / 10000;
                        // Update numberAddressesInPreviousBracket
                        _number_addresses_in_previous_bracket = number_ticket_in_winning_number;
                    }
                    // A. No NEAR to distribute, they are added to the amount to withdraw to treasury address
                } else {
                    lottery.near_per_bracket[j as usize] = 0;
                    _amount_to_withdraw_to_treasury = _amount_to_withdraw_to_treasury
                        + (lottery.rewards_breakdown[j as usize] * _amount_to_share_to_winners)
                            / 10000;
                }
            }
        }

        // Update internal statuses for lottery
        lottery.final_number = _final_number;
        lottery.status = Status::Claimable;

        // save to chain
        data._lotteries.insert(&_lottery_id, &lottery);

        if _auto_injection {
            // incase there is no one won, we automatically get the number of shares winner per breakdown to pending injector next lottery
            data.pending_injection_next_lottery = _amount_to_withdraw_to_treasury;
            _amount_to_withdraw_to_treasury = 0;
        }

        // the fee of the protocol is calcualted by colllected in near minus the amount to share to winners.
        _amount_to_withdraw_to_treasury = _amount_to_withdraw_to_treasury
            + (lottery.amount_collected_in_near - _amount_to_share_to_winners);

        // Transfer NEAR to treasury address
        Promise::new(data.treasury_address.clone()).transfer(_amount_to_withdraw_to_treasury);

        env::log_str(
            &json!({
                "type": "draw_final_number_and_make_lottery_claimable",
                "params": {
                    "final_number":  _final_number,
                    "number_address_in_previous_bracket": U128(_number_addresses_in_previous_bracket),
                    "amount_to_withdraw_to_treasury": U128(_amount_to_withdraw_to_treasury),
                    "amount_collected_in_near": U128(lottery.amount_collected_in_near),
                    "amount_to_share_to_winners": U128(_amount_to_share_to_winners),
                }
            })
            .to_string(),
        );
    }

    /**
     * @notice Buy tickets for the current lottery
     * @param _lotteryId: lotteryId
     * @param _ticketNumbers: array of ticket numbers between 1,000,000 and 1,999,999
     * @dev Callable by users
     */
    #[payable]
    pub fn buy_tickets(&mut self, _lottery_id: LotteryId, _ticket_numbers: Vec<TicketNumber>) {
        self.assert_contract_running();
        assert!(_ticket_numbers.len() > 0, "{}", ERR21_TICKETS__LENGTH);
        let data = self.data_mut();
        let mut lottery = data
            ._lotteries
            .get(&_lottery_id)
            .expect(ERR1_NOT_EXISTING_LOTTERY);

        assert!(
            _ticket_numbers.len() <= data.max_number_tickets_per_buy_or_claim as usize,
            "{}",
            ERR22_LOTTERY_CLAIM_TOO_MANY_TICKETS
        );

        assert_eq!(
            lottery.status,
            Status::Open,
            "{}",
            ERR17_LOTTERY_IS_NOT_OPEN
        );

        assert!(
            env::block_timestamp() < lottery.end_time,
            "{}",
            ERR31_LOTTERY_IS_OVER
        );

        // asert storage cover
        assert_estimate_storage_usage_data(&data, _ticket_numbers.len() as u64);

        // Calculate number of NEAR to this contract
        let amount_near_to_transfer = _calculate_total_price_for_bulk_tickets(
            lottery.discount_divisor,
            lottery.price_ticket_in_near,
            _ticket_numbers.len() as u128,
        );
        assert!(
            env::attached_deposit() == amount_near_to_transfer,
            "{}: {}",
            ERR16_ATTACHED_DEPOSIT_NOT_EQUAL_AMOUNT,
            amount_near_to_transfer
        );

        // Increment the total amount collected for the lottery round
        lottery.amount_collected_in_near =
            lottery.amount_collected_in_near + amount_near_to_transfer;
        data._lotteries.insert(&_lottery_id, &lottery);
        data.permission_update = PermissionUpdateState::Allow;

        // update lottery data
        let mut number_tickets_in_a_lottery = data
            ._number_tickers_per_lottery_id
            .get(&_lottery_id)
            .unwrap_or(UnorderedMap::new(StorageKey::NumberTickersPerLotteryId {
                lottery_id: _lottery_id,
            }));
        let mut user_lotteries = data
            ._user_ticket_ids_per_lottery_id
            .get(&env::predecessor_account_id())
            .unwrap_or(UnorderedMap::new(StorageKey::UserTicketsPerLottery {
                account_id: env::predecessor_account_id(),
            }));
        for i in 0.._ticket_numbers.len() {
            let ticket_number = _ticket_numbers[i];
            assert!(
                ticket_number >= 1000000 && ticket_number <= 1999999,
                "{}",
                ERR31_TICKET_NUMBER_RANGE
            );

            // generate bracket key and number values. Increase by 1
            let key_brackets: Vec<u32> = (1..=6 as u32)
                .into_iter()
                .map(|x| create_number_one(x) + (ticket_number % 10u32.pow(x)))
                .collect();
            for key_bracket in key_brackets.into_iter() {
                // increase counting for each key number
                let value_bracket = number_tickets_in_a_lottery.get(&key_bracket).unwrap_or(0) + 1;
                // insert each bracket
                number_tickets_in_a_lottery.insert(&key_bracket, &value_bracket);
            }
            data._number_tickers_per_lottery_id
                .insert(&_lottery_id, &number_tickets_in_a_lottery);

            // push new ticket id to user_tickets
            let mut user_tickets_in_a_lottery = user_lotteries.get(&_lottery_id).unwrap_or(vec![]);
            user_tickets_in_a_lottery.push(data.current_ticket_id);
            user_lotteries.insert(&_lottery_id, &user_tickets_in_a_lottery);
            data._user_ticket_ids_per_lottery_id
                .insert(&env::predecessor_account_id(), &user_lotteries);

            // save tickets with current ticket id
            data._tickets.insert(
                &data.current_ticket_id,
                &Ticket {
                    number: ticket_number,
                    owner: env::predecessor_account_id().clone(),
                },
            );

            // Increase lottery ticket number
            data.current_ticket_id = data.current_ticket_id + 1;
        }

        // fire log
        env::log_str(
            &json!({
                "type": "buy_tickets",
                "params": {
                    "buyer": &env::predecessor_account_id(),
                    "current_lottery_id":  data.current_lottery_id,
                    "numbers_tickets": _ticket_numbers.len(),
                    "ticket_numbers": _ticket_numbers,

                }
            })
            .to_string(),
        );
    }

    /**
     * @notice Claim all set of winning tickets
     * @param _lotteryId: lottery id
     * @param _ticketIds: array of ticket ids
     * @param _brackets: array of brackets for the ticket ids
     * @dev Callable by users only, not contract!
     */
    #[payable]
    pub fn claim_all_tickets(&mut self) {
        self.assert_one_yoctor();
        self.assert_contract_running();
        let data = self.data_mut();

        // get full tickets ids of a user
        let account_id = env::predecessor_account_id();
        let lotteries_user_list = data
            ._user_ticket_ids_per_lottery_id
            .get(&account_id)
            .expect(ERR4_NOT_EXISTING_LOTTERIES_PER_USER);

        let lottery_key_ids = lotteries_user_list.keys_as_vector();
        let mut near_rewards = 0;
        let mut all_tickets_won = vec![];
        let mut lotteries_won = vec![];
        for i in 0..lottery_key_ids.len() {
            let lottery_id = lottery_key_ids.get(i).unwrap_or(0);

            let user_tickets_in_a_lottery = lotteries_user_list.get(&lottery_id).unwrap_or(vec![]);
            let lottery = data._lotteries.get(&lottery_id);
            if user_tickets_in_a_lottery.len() > 0 && lottery.is_some() {
                let (reward_in_near_to_transfer, tickets_won) =
                    calcualte_near_for_lottery(data, &lottery.unwrap(), &user_tickets_in_a_lottery);
                if reward_in_near_to_transfer > 0 {
                    lotteries_won.push(lottery_id);
                    near_rewards = near_rewards + reward_in_near_to_transfer;
                    all_tickets_won.extend(tickets_won);
                }
            }
        }

        // update all tickets won owner

        for i in 0..all_tickets_won.len() {
            let ticket_id = all_tickets_won[i];

            let mut ticket = data
                ._tickets
                .get(&ticket_id)
                .expect(ERR2_NOT_EXISTING_TICKET);
            assert_eq!(
                env::predecessor_account_id(),
                ticket.owner,
                "{}",
                ERR27_LOTTERY_CLAIM_TICKET_NOT_OWNER
            );
            ticket.owner = AccountId::new_unchecked(ZERO_ADDRESS_WALLET.to_string());
            data._tickets.insert(&ticket_id, &ticket);
        }

        // Transfer money to msg.sender
        if near_rewards > 0 {
            Promise::new(env::predecessor_account_id()).transfer(near_rewards);
        }

        env::log_str(
            &json!({
                "type": "claim_all_tickets",
                "params": {
                    "claimer": env::predecessor_account_id(),
                    "transfer_amount_in_reward":  U128(near_rewards),
                    "lotteries_won": lotteries_won,
                    "ticket_ids_length": all_tickets_won.len(),
                    "ticket_ids": all_tickets_won
                }
            })
            .to_string(),
        );
    }

    /**
     * @notice Claim a set of winning tickets for a lottery
     * @param _lotteryId: lottery id
     * @param _ticketIds: array of ticket ids
     * @param _brackets: array of brackets for the ticket ids
     * @dev Callable by users only, not contract!
     */
    #[payable]
    pub fn claim_tickets(&mut self, _lottery_id: LotteryId) {
        self.assert_one_yoctor();
        self.assert_contract_running();
        let data = self.data_mut();
        // check lottery existing
        let lottery = data
            ._lotteries
            .get(&_lottery_id)
            .expect(ERR1_NOT_EXISTING_LOTTERY);

        // Only allow claimming after drawing a winning number
        assert_eq!(
            lottery.status,
            Status::Claimable,
            "{}",
            ERR23_LOTTERY_NOT_CLAMABLE
        );

        // get full tickets ids of a user
        let account_id = env::predecessor_account_id();
        let lotteries_user_list = data
            ._user_ticket_ids_per_lottery_id
            .get(&account_id)
            .expect(ERR4_NOT_EXISTING_LOTTERIES_PER_USER);
        let user_lottery_tickets_ids = lotteries_user_list
            .get(&_lottery_id)
            .expect(ERR2_NOT_EXISTING_TICKET);

        assert_ne!(
            user_lottery_tickets_ids.len(),
            0,
            "{}",
            ERR21_TICKETS__LENGTH
        );

        // calculate near in reward for each bracket might be received
        let number_of_user_tickets = user_lottery_tickets_ids.len();
        let data = self.data_mut();
        let (reward_in_near_to_transfer, tickets_won) =
            calcualte_near_for_lottery(&data, &lottery, &user_lottery_tickets_ids);

        // update all tickets won owner
        for i in 0..tickets_won.len() {
            let ticket_id = tickets_won[i];

            let mut ticket = data
                ._tickets
                .get(&ticket_id)
                .expect(ERR2_NOT_EXISTING_TICKET);
            assert_eq!(
                env::predecessor_account_id(),
                ticket.owner,
                "{}",
                ERR27_LOTTERY_CLAIM_TICKET_NOT_OWNER
            );
            ticket.owner = AccountId::new_unchecked(ZERO_ADDRESS_WALLET.to_string());
            data._tickets.insert(&ticket_id, &ticket);
        }

        // Transfer money to msg.sender
        if reward_in_near_to_transfer > 0 {
            Promise::new(env::predecessor_account_id()).transfer(reward_in_near_to_transfer);
        }

        env::log_str(
            &json!({
                "type": "claim_ticket",
                "params": {
                    "claimer": env::predecessor_account_id(),
                    "transfer_amount_in_reward":  U128(reward_in_near_to_transfer),
                    "current_lottery_id": _lottery_id,
                    "ticket_ids_length": number_of_user_tickets,
                    "ticket_ids": user_lottery_tickets_ids
                }
            })
            .to_string(),
        );
    }

    /**
     * @notice Close lottery
     * @param _lotteryId: lottery id
     * @dev Callable by operator
     */
    #[payable]
    pub fn close_lottery(&mut self, _lottery_id: LotteryId) {
        self.assert_one_yoctor();
        self.assert_operator_calling();
        self.assert_contract_running();
        let data = self.data_mut();
        let mut lottery = data
            ._lotteries
            .get(&data.current_lottery_id)
            .expect(ERR1_NOT_EXISTING_LOTTERY);

        assert_eq!(
            lottery.status,
            Status::Open,
            "{}",
            ERR17_LOTTERY_IS_NOT_OPEN
        );

        assert!(
            env::block_timestamp() >= lottery.end_time,
            "{}",
            ERR40_CURRENTY_TIME_LESS_END_TIME
        );

        // mark the next id
        lottery.first_ticket_id_next_lottery = data.current_ticket_id;

        // random winning number
        data.permission_update = PermissionUpdateState::Allow;
        lottery.status = Status::Close;
        data._lotteries.insert(&_lottery_id, &lottery);

        env::log_str(
            &json!({
                "type": "close_lottery",
                "params": {
                    "current_lottery_id": data.current_lottery_id,
                    "current_ticket_id":  data.current_ticket_id,
                }
            })
            .to_string(),
        );
    }
}
