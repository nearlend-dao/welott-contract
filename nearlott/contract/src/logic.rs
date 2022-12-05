use crate::*;

pub const ZERO_ADDRESS_WALLET: &str = "no_account.near";

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
        _reserve_fee: Option<U128>,
        _operate_fee: Option<U128>,
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
        let reserve_fee = extract_data(_reserve_fee);
        let operate_fee = extract_data(_operate_fee);

        assert!(
            discount_divisor >= data.min_discount_divisor,
            "{}",
            ERR13_LOTTERY_DISCOUNT_DIVISOR_TOO_LOW
        );

        assert!(
            reserve_fee <= data.max_reserve_fee,
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
                reserve_fee,
                near_per_bracket: vec![0, 0, 0, 0, 0, 0],
                count_winners_per_bracket: vec![0, 0, 0, 0, 0, 0],
                first_ticket_id: data.current_ticket_id,
                first_ticket_id_next_lottery: data.current_ticket_id,
                amount_collected_in_near: data.pending_injection_next_lottery,
                last_pot_size: data.pending_injection_next_lottery,
                final_number: 0,
                operate_fee,
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
                    "reserve_fee": U128(reserve_fee),
                    "operate_fee": U128(operate_fee),
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
        let final_number = get_random_number();
        // let final_number = 1327419; // TODO: Only remove on the mainnet. It's necessary for testing purpose
        data.random_result = final_number;

        // Calculate the finalNumber based on the randomResult generated
        let _final_number = data.random_result as u32;
        // Initialize a number to count addresses in the previous bracket
        let mut _number_addresses_in_previous_bracket: u128 = 0;

        // Calculate the amount to share post-treasury fee
        // The totally amount_collected_in_near minus 20% of the reserve pool, minutes 5% of the operator fee
        let _operate_fee = ((lottery.amount_collected_in_near - lottery.last_pot_size)
            * lottery.operate_fee)
            / 10000;
        let _reserver_fee =
            ((lottery.amount_collected_in_near - _operate_fee) * lottery.reserve_fee) / 10000;
        let mut _amount_to_share_to_winners = 0;
        if lottery.amount_collected_in_near > _operate_fee {
            _amount_to_share_to_winners =
                lottery.amount_collected_in_near - _operate_fee - _reserver_fee
        }

        // Initializes the amount to withdraw to the next lottery
        let mut _amount_to_withdraw_to_next_lottery: u128 = 0;

        if lottery.first_ticket_id_next_lottery - lottery.first_ticket_id > 0 {
            let number_tickets_per_lottery = data
                ._bracket_tickets_number
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
                    _amount_to_withdraw_to_next_lottery = _amount_to_withdraw_to_next_lottery
                        + (lottery.rewards_breakdown[j as usize] * _amount_to_share_to_winners)
                            / 10000;
                }
            }
        } else {
            _amount_to_withdraw_to_next_lottery = _amount_to_share_to_winners
        }

        // Update internal statuses for lottery
        lottery.final_number = _final_number;
        lottery.status = Status::Claimable;

        // save to chain
        data._lotteries.insert(&_lottery_id, &lottery);

        if _auto_injection {
            // incase there is no one won, we automatically get the number of shares winner per breakdown to pending injector next lottery
            // add reserve fee to the next lottery
            data.pending_injection_next_lottery =
                _amount_to_withdraw_to_next_lottery + _reserver_fee;
            _amount_to_withdraw_to_next_lottery = 0;
        }

        // Transfer NEAR to treasury_address
        Promise::new(data.treasury_address.clone()).transfer(_operate_fee);

        // convert near per bracket to string
        let near_per_bracket: Vec<String> = lottery
            .near_per_bracket
            .iter()
            .map(|&id| id.to_string())
            .collect();

        // get rewards breakdown
        let rewards_breakdown: Vec<String> = lottery
            .rewards_breakdown
            .iter()
            .map(|&id| id.to_string())
            .collect();

        // get count tickets for each bracket
        let counter_winners: Vec<String> = lottery
            .count_winners_per_bracket
            .iter()
            .map(|&id| id.to_string())
            .collect();

        env::log_str(
            &json!({
                "type": "draw_final_number_and_make_lottery_claimable",
                "params": {
                    "final_number":  _final_number,
                    "current_lottery_id":  _lottery_id,
                    "amount_to_withdraw_to_treasury": U128(_amount_to_withdraw_to_next_lottery),
                    "near_per_bracket": near_per_bracket.join(","),
                    "rewards_breakdown": rewards_breakdown.join(","),
                    "counter_winners": counter_winners.join(","),
                    "amount_collected_in_near": U128(lottery.amount_collected_in_near),
                    "operator_fee": U128(_operate_fee),
                    "reserver_fee": U128(_reserver_fee),
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
        // Check total tickets of user per a lottery
        let account_id = env::predecessor_account_id();
        let mut account = self.internal_unwrap_account(&account_id);
        let user_tickets = account.internal_get_ticket_id_per_lottery_or_default(&_lottery_id);
        assert!(
            (user_tickets.len() + _ticket_numbers.len()) <= 120,
            "{}",
            ERR43_ACCOUNT_MAX_TICKETS_PER_A_LOTTERY
        );

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

        // Calculate number of NEAR to this contract
        let amount_near_to_transfer = _calculate_total_price_for_bulk_tickets(
            lottery.discount_divisor,
            lottery.price_ticket_in_near,
            _ticket_numbers.len() as u128,
        );
        assert!(
            env::attached_deposit() >= amount_near_to_transfer,
            "{}: {}",
            ERR16_ATTACHED_DEPOSIT_NOT_EQUAL_AMOUNT,
            amount_near_to_transfer
        );

        // make sure the range of numbers is invalid
        let _valid_ticket_arrays: Vec<TicketNumber> = _ticket_numbers
            .iter()
            .map(|&x| {
                assert!(
                    x >= 1000000 && x <= 1999999,
                    "{}",
                    ERR31_TICKET_NUMBER_RANGE
                );
                x
            })
            .collect();

        // update lottery data
        let mut _bracket_tickets_number =
            data._bracket_tickets_number
                .get(&_lottery_id)
                .unwrap_or(UnorderedMap::new(StorageKey::BracketTicketNumbers {
                    lottery_id: _lottery_id,
                }));

        // prepare key bracket for kind of decimals values
        let bracket_placeholder: Vec<u32> = (1..=6 as u32)
            .into_iter()
            .map(|x| create_number_one(x))
            .collect();

        let mut ticket_ids: Vec<String> = vec![];

        for i in 0.._valid_ticket_arrays.len() {
            let ticket_number = _valid_ticket_arrays[i];

            // generate bracket key and number values. Increase by 1
            for x in &bracket_placeholder {
                let key_bracket = x + (ticket_number % 10u32.pow(x.to_string().len() as u32));
                let value_bracket = _bracket_tickets_number.get(&key_bracket).unwrap_or(0) + 1;
                _bracket_tickets_number.insert(&key_bracket, &value_bracket);
            }

            // save data to user info
            let mut account = internal_get_account_unwrap_by_contract_data(data, &account_id);
            account.internal_set_ticket_ids_per_lottery(&_lottery_id, data.current_ticket_id);

            // calcualte deposit storage
            internal_set_account_data(data, &account_id, account);

            // save bracket counting number
            data._bracket_tickets_number
                .insert(&_lottery_id, &_bracket_tickets_number);

            // save tickets with current ticket id
            data._tickets.insert(
                &data.current_ticket_id,
                &Ticket {
                    number: ticket_number,
                    owner: env::predecessor_account_id().clone(),
                },
            );

            // Increase lottery ticket number
            let ticket_id = data.current_ticket_id;
            ticket_ids.push(ticket_id.to_string());

            data.current_ticket_id = data.current_ticket_id + 1;
        }

        // saving data
        // Increment the total amount collected for the lottery round
        lottery.amount_collected_in_near =
            lottery.amount_collected_in_near + amount_near_to_transfer;
        data._lotteries.insert(&_lottery_id, &lottery);
        data.permission_update = PermissionUpdateState::Allow;

        // fire log
        let _ticket_numbers_str: Vec<String> =
            _ticket_numbers.iter().map(|&id| id.to_string()).collect();
        env::log_str(
            &json!({
                "type": "buy_tickets",
                "params": {
                    "buyer": &env::predecessor_account_id(),
                    "current_lottery_id":  data.current_lottery_id,
                    "ticket_numbers": _ticket_numbers_str.join(","),
                    "ticket_ids": ticket_ids.join(",")
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
    pub fn claim_tickets(
        &mut self,
        _lottery_id: LotteryId,
        _ticket_ids: Vec<TicketId>,
        _brackets: Vec<BracketPosition>,
    ) {
        self.assert_one_yoctor();
        self.assert_contract_running();
        let data = self.data_mut();

        // check ticket len and bracket
        assert_eq!(
            _ticket_ids.len(),
            _brackets.len(),
            "{}",
            ERR20_LOTTERY_CLAIM_NOT_SAME_LENGTH
        );

        // Preventing ticket ids
        assert_ne!(_ticket_ids.len(), 0, "{}", ERR21_TICKETS__LENGTH);

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

        // Initializes the reward_in_near_to_transfer
        let mut reward_in_near_to_transfer = 0;
        let mut rewards = vec![];
        for i in 0.._ticket_ids.len() {
            // If position of the bracket >= 6
            assert!(_brackets[i] < 6, "{}", ERR24_BRACKETS_OUT_RANGE); // Must be between 0 and 5

            // get ticket id
            let this_ticket_id = _ticket_ids[i];

            assert!(
                lottery.first_ticket_id_next_lottery > this_ticket_id,
                "{} - first_ticket_id_next_lottery: {}, this_ticket_id: {}",
                ERR25_LOTTERY_CLAIM_TICKET_TOO_HIGH,
                lottery.first_ticket_id_next_lottery,
                this_ticket_id
            );

            assert!(
                lottery.first_ticket_id <= this_ticket_id,
                "{} - first_ticket_id: {}, this_ticket_id: {}",
                ERR26_LOTTERY_CLAIM_TICKET_TOO_LOW,
                lottery.first_ticket_id,
                this_ticket_id
            );

            let mut ticket = data
                ._tickets
                .get(&this_ticket_id)
                .expect(ERR2_NOT_EXISTING_TICKET);

            assert_eq!(
                env::predecessor_account_id(),
                ticket.owner,
                "{}",
                ERR27_LOTTERY_CLAIM_TICKET_NOT_OWNER
            );

            // calculate near rewards
            let reward_for_ticket_id =
                _calculate_rewards_for_ticket_id(data, _lottery_id, this_ticket_id, _brackets[i]);

            // Check user is claiming the correct bracket
            assert_ne!(reward_for_ticket_id, 0, "{}", ERR28_LOTTERY_CLAIM_NO_PRIZE);

            // revevalute the bracket positon. If still have a bracket higher has reward. We fire an exception
            if _brackets[i] != 5 {
                assert_eq!(
                    _calculate_rewards_for_ticket_id(
                        data,
                        _lottery_id,
                        this_ticket_id,
                        _brackets[i] + 1 // bracket higher
                    ),
                    0,
                    "{} - ticket: {}, bracket should be: {}",
                    ERR29_LOTTERY_CLAIM_BRACKET_MUST_BE_HIGHER,
                    this_ticket_id,
                    _brackets[i] + 1
                );
            }

            // Update the lottery ticket owner to 0x address
            let zero_address = AccountId::new_unchecked(ZERO_ADDRESS_WALLET.to_string());
            ticket.owner = zero_address;
            data._tickets.insert(&this_ticket_id, &ticket);

            // Increment the reward to transfer
            reward_in_near_to_transfer += reward_for_ticket_id;

            // add reward into vector rewards
            rewards.push(reward_for_ticket_id.to_string());
        }

        // Transfer money to msg.sender
        assert!(reward_in_near_to_transfer > 0, "{}", ERR41_ALREADY_CLAIMED);

        // transfer
        if reward_in_near_to_transfer > 0 {
            // before transfer
            Promise::new(env::predecessor_account_id()).transfer(reward_in_near_to_transfer);

            let _ticket_ids_str: Vec<String> =
                _ticket_ids.iter().map(|&id| id.to_string()).collect();
            let _brackets_str: Vec<String> = _brackets.iter().map(|&id| id.to_string()).collect();
            env::log_str(
                &json!({
                    "type": "claim_tickets",
                    "params": {
                        "claimer": env::predecessor_account_id(),
                        "transfer_amount_in_reward":  U128(reward_in_near_to_transfer),
                        "current_lottery_id": _lottery_id,
                        "ticket_ids_length": _ticket_ids.len(),
                        "bracket_rewards": _brackets_str.join(","),
                        "ticket_ids": _ticket_ids_str.join(","),
                        "rewards": rewards.join(","),
                    }
                })
                .to_string(),
            );
        }
    }

    /**
     * @notice Close lottery
     * @dev Callable by operator
     */
    #[payable]
    pub fn close_lottery(&mut self) {
        self.assert_one_yoctor();
        self.assert_operator_calling();
        self.assert_contract_running();
        let data = self.data_mut();
        let _lottery_id = data.current_lottery_id;
        let mut lottery = data
            ._lotteries
            .get(&_lottery_id)
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
                    "current_lottery_id": _lottery_id,
                    "current_ticket_id":  data.current_ticket_id,
                }
            })
            .to_string(),
        );
    }
}
