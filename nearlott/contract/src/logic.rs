use crate::*;

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
    pub fn start_lottery(
        &mut self,
        _end_time: Timestamp,
        _price_ticket_in_near: u128,
        _discount_divisor: u128,
        _rewards_breakdown: Vec<u128>,
        _treasury_fee: u128,
    ) {
        self.assert_operator_calling();
        self.assert_contract_running();

        let mut data = self.data_mut();
        // after 4 hours - 5 minutes since now to  4 days + 5 minutes
        assert!(
            to_sec(_end_time - env::block_timestamp()) > data.min_length_lottery
                && to_sec(_end_time - env::block_timestamp()) < data.max_length_lottery,
            "{}",
            ERR11_LOTTERY_TIME_OUT_OF_RANGE
        );

        assert!(
            _price_ticket_in_near >= data.min_price_ticket_in_near
                && _price_ticket_in_near <= data.max_price_ticket_in_near,
            "{}",
            ERR12_LOTTERY_PRICE_OUTSIDE_LIMIT
        );

        assert!(
            _discount_divisor >= data.min_discount_divisor,
            "{}",
            ERR13_LOTTERY_DISCOUNT_DIVISOR_TOO_LOW
        );

        assert!(
            _treasury_fee <= data.max_treasury_fee,
            "{}",
            ERR15_LOTTERY_OVER_TREASURY_FEE
        );

        let sum_rewards: u128 = _rewards_breakdown.iter().sum();
        assert_eq!(sum_rewards, 10000, "{}", ERR14_LOTTERY_OVER_RANGE_REWARDS);

        let next_lottery_id = data.current_lottery_id + 1;
        data.current_lottery_id = next_lottery_id;

        data._lotteries.insert(
            &next_lottery_id,
            &Lottery {
                status: Status::Open,
                start_time: env::block_timestamp(),
                end_time: _end_time,
                price_ticket_in_near: _price_ticket_in_near,
                discount_divisor: _discount_divisor,
                rewards_breakdown: _rewards_breakdown,
                treasury_fee: _treasury_fee,
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
                    "price_ticket_in_near": U128(_price_ticket_in_near),
                    "first_ticket_id": data.current_ticket_id,
                    "first_ticket_id_next_lottery": data.current_ticket_id,
                    "pending_injection_next_lottery": U128(data.pending_injection_next_lottery),
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
    pub fn draw_final_number_and_make_lottery_claimable(
        &mut self,
        _lottery_id: LotteryId,
        _auto_injection: bool,
    ) {
        self.assert_operator_calling();
        self.assert_contract_running();

        let data = self.data_mut();
        let mut lottery = data
            ._lotteries
            .get(&data.current_lottery_id)
            .expect(ERR1_NOT_EXISTING_LOTTERY);

        assert_eq!(
            lottery.status,
            Status::Close,
            "{}",
            ERR30_LOTTERY_IS_NOT_CLOSE
        );

        assert_eq!(
            _lottery_id, data.current_lottery_id,
            "{}",
            ERR18_LOTTERY_FINAL_NUMBER_NOT_DRAWN
        );

        // Calculate the finalNumber based on the randomResult generated
        let _final_number = data.random_result as u32;

        // Initialize a number to count addresses in the previous bracket
        let mut _number_addresses_in_previous_bracket: u128 = 0;

        // Calculate the amount to share post-treasury fee
        let _amount_to_share_to_winners =
            (lottery.amount_collected_in_near * (10000 - lottery.treasury_fee)) / 10000;

        // Initializes the amount to withdraw to treasury
        let mut _amount_to_withdraw_to_treasury: u128 = 0;

        // Calculate prizes in NEAR for each bracket by starting from the highest one
        for i in 0..6 {
            let j = 5 - i;

            let bracket_number = data
                ._bracket_calculator
                .get(&j)
                .expect(ERR3_NOT_EXISTING_BRACKET);

            let _transformed_winning_number = bracket_number + (_final_number % (10u32.pow(j + 1)));

            let number_tickets_per_lottery = data
                ._number_tickers_per_lottery_id
                .get(&_lottery_id)
                .expect(ERR19_LOTTERY_NO_TICKERS_NUMBERS);

            let number_ticket_in_winning_number = number_tickets_per_lottery
                .get(&_transformed_winning_number)
                .unwrap_or(0);

            lottery.count_winners_per_bracket[j as usize] =
                number_ticket_in_winning_number - _number_addresses_in_previous_bracket;
            // A. If number of users for this _bracket number is superior to 0
            if (number_ticket_in_winning_number - number_ticket_in_winning_number) != 0 {
                // B. If rewards at this bracket are > 0, calculate, else, report the numberAddresses from previous bracket
                // rewardsBreakdown / total (10000) * amount_to_shared_to_winner / (total bracket winner - previous bracket received)
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
                    + (lottery.rewards_breakdown[j as usize] * _amount_to_share_to_winners) / 10000;
            }
        }

        // Update internal statuses for lottery
        lottery.final_number = _final_number;
        lottery.status = Status::Claimable;

        // save to chain
        data._lotteries.insert(&_lottery_id, &lottery);

        if _auto_injection {
            data.pending_injection_next_lottery = _amount_to_withdraw_to_treasury;
            _amount_to_withdraw_to_treasury = 0;
        }

        _amount_to_withdraw_to_treasury = _amount_to_withdraw_to_treasury
            + (lottery.amount_collected_in_near - _amount_to_share_to_winners);

        // Transfer NEAR to treasury address
        Promise::new(data.treasury_address.clone()).transfer(_amount_to_withdraw_to_treasury);

        env::log_str(
            &json!({
                "type": "draw_final_number_and_make_lottery_claimable",
                "params": {
                    "current_lottery_id": data.current_lottery_id,
                    "final_number":  _final_number,
                    "number_address_in_previous_bracket": _number_addresses_in_previous_bracket,
                    "amount_to_withdraw_to_treasury": _amount_to_withdraw_to_treasury,
                    "amount_collected_in_near": lottery.amount_collected_in_near,
                    "amount_to_share_to_winners": _amount_to_share_to_winners,
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
    pub fn buy_tickets(
        &mut self,
        _lottery_id: LotteryId,
        _ticket_numbers: Vec<TicketNumber>,
        amount: u128,
    ) {
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
            ERR30_LOTTERY_IS_NOT_CLOSE
        );

        assert!(
            to_sec(env::block_timestamp()) < lottery.end_time as u32,
            "{}",
            ERR31_LOTTERY_IS_OVER
        );

        // Calculate number of NEAR to this contract
        let amount_near_to_transfer = _calculate_total_price_for_bulk_tickets(
            lottery.discount_divisor,
            lottery.price_ticket_in_near,
            _ticket_numbers.len() as u128,
        );

        assert!(
            env::attached_deposit() >= amount,
            "{}",
            ERR16_ATTACHED_DEPOSIT_LESS_AMOUNT
        );
        assert!(
            amount_near_to_transfer >= amount,
            "{}",
            ERR16_ATTACHED_DEPOSIT_LESS_AMOUNT
        );

        // Increment the total amount collected for the lottery round
        lottery.amount_collected_in_near =
            lottery.amount_collected_in_near + amount_near_to_transfer;
        data._lotteries.insert(&_lottery_id, &lottery);

        let mut number_tickets_in_a_lottery = data
            ._number_tickers_per_lottery_id
            .get(&_lottery_id)
            .unwrap_or(UnorderedMap::new(b"A".to_vec()));

        let mut user_lotteries = data
            ._user_ticket_ids_per_lottery_id
            .get(&env::predecessor_account_id())
            .unwrap_or(UnorderedMap::new(b"A".to_vec()));

        for i in 0.._ticket_numbers.len() {
            let ticket_number = _ticket_numbers[i];
            assert!(
                ticket_number >= 1000000 && ticket_number <= 1999999,
                "{}",
                ERR31_TICKET_NUMBER_RANGE
            );

            let key_bracket1 = 1 + (ticket_number % 10);
            let key_bracket2 = 11 + (ticket_number % 100);
            let key_bracket3 = 111 + (ticket_number % 1000);
            let key_bracket4 = 1111 + (ticket_number % 10000);
            let key_bracket5 = 11111 + (ticket_number % 100000);
            let key_bracket6 = 111111 + (ticket_number % 1000000);

            let value_bracket1 = number_tickets_in_a_lottery.get(&key_bracket1).unwrap_or(0) + 1;
            let value_bracket2 = number_tickets_in_a_lottery.get(&key_bracket2).unwrap_or(0) + 1;
            let value_bracket3 = number_tickets_in_a_lottery.get(&key_bracket3).unwrap_or(0) + 1;
            let value_bracket4 = number_tickets_in_a_lottery.get(&key_bracket4).unwrap_or(0) + 1;
            let value_bracket5 = number_tickets_in_a_lottery.get(&key_bracket5).unwrap_or(0) + 1;
            let value_bracket6 = number_tickets_in_a_lottery.get(&key_bracket6).unwrap_or(0) + 1;

            number_tickets_in_a_lottery.insert(&key_bracket1, &value_bracket1);
            number_tickets_in_a_lottery.insert(&key_bracket2, &value_bracket2);
            number_tickets_in_a_lottery.insert(&key_bracket3, &value_bracket3);
            number_tickets_in_a_lottery.insert(&key_bracket4, &value_bracket4);
            number_tickets_in_a_lottery.insert(&key_bracket5, &value_bracket5);
            number_tickets_in_a_lottery.insert(&key_bracket6, &value_bracket6);

            data._number_tickers_per_lottery_id
                .insert(&_lottery_id, &number_tickets_in_a_lottery);

            // push new ticket id
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

        env::log_str(
            &json!({
                "type": "buy_tickets",
                "params": {
                    "buyer": &env::predecessor_account_id(),
                    "current_lottery_id":  data.current_lottery_id,
                    "ticket_numbers": _ticket_numbers.len()
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
    pub fn claim_tickets(
        &mut self,
        _lottery_id: LotteryId,
        _ticket_ids: Vec<TicketId>,
        _brackets: Vec<BracketPosition>,
    ) {
        self.assert_contract_running();
        let data = self.data_mut();
        assert_eq!(
            _ticket_ids.len(),
            _brackets.len(),
            "{}",
            ERR20_LOTTERY_CLAIM_NOT_SAME_LENGTH
        );

        assert_ne!(_ticket_ids.len(), 0, "{}", ERR21_TICKETS__LENGTH);

        assert!(
            _ticket_ids.len() <= data.max_number_tickets_per_buy_or_claim as usize,
            "{}",
            ERR22_LOTTERY_CLAIM_TOO_MANY_TICKETS
        );

        let lottery = data
            ._lotteries
            .get(&_lottery_id)
            .expect(ERR1_NOT_EXISTING_LOTTERY);

        assert_eq!(
            lottery.status,
            Status::Claimable,
            "{}",
            ERR23_LOTTERY_CLAIM_TOO_MANY_TICKETS
        );

        // Initializes the reward_in_near_to_transfer
        let mut reward_in_near_to_transfer = 0;
        for i in 0.._ticket_ids.len() {
            assert!(_brackets[i] < 6, "{}", ERR24_BRACKETS_OUT_RANGE); // Must be between 0 and 5

            let this_ticket_id = _ticket_ids[i];

            assert!(
                lottery.first_ticket_id_next_lottery > this_ticket_id,
                "{}",
                ERR25_LOTTERY_CLAIM_TICKET_TOO_HIGH
            );
            assert!(
                lottery.first_ticket_id <= this_ticket_id,
                "{}",
                ERR26_LOTTERY_CLAIM_TICKET_TOO_LOW
            );

            let mut ticket = data
                ._tickets
                .get(&this_ticket_id)
                .expect(ERR2_NOT_EXISTING_TICKET);
            assert_eq!(
                env::predecessor_account_id(),
                ticket.owner,
                "{}",
                ERR25_LOTTERY_CLAIM_TICKET_TOO_HIGH
            );

            // Update the lottery ticket owner to 0x address
            ticket.owner = AccountId::new_unchecked("0".to_string());
            data._tickets.insert(&this_ticket_id, &ticket);

            let reward_for_ticket_id =
                _calculate_rewards_for_ticket_id(data, _lottery_id, this_ticket_id, _brackets[i]);

            // Check user is claiming the correct bracket
            assert_ne!(reward_for_ticket_id, 0, "{}", ERR28_LOTTERY_CLAIM_NO_PRIZE);

            if _brackets[i] != 5 {
                assert_eq!(
                    _calculate_rewards_for_ticket_id(
                        data,
                        _lottery_id,
                        this_ticket_id,
                        _brackets[i] + 1
                    ),
                    0,
                    "{}",
                    ERR29_LOTTERY_CLAIM_BRACKET_MUST_BE_HIGHER
                );
            }
            // Increment the reward to transfer
            reward_in_near_to_transfer += reward_for_ticket_id;
        }

        // Transfer money to msg.sender
        Promise::new(env::predecessor_account_id()).transfer(reward_in_near_to_transfer);

        env::log_str(
            &json!({
                "type": "claim_ticket",
                "params": {
                    "claimer": env::predecessor_account_id(),
                    "transfer_amount":  reward_in_near_to_transfer,
                    "current_lottery_id": _lottery_id,
                    "ticket_ids_length": _ticket_ids.len(),
                    "brackets_length": _brackets.len(),
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
    pub fn close_lottery(&mut self, _lottery_id: LotteryId) {
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
        let final_number = get_random_number();
        data.random_result = final_number;
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
