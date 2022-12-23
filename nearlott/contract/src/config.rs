use std::ops::Div;

use crate::*;

const LIMIT_TIME_IN_LOTTERY: u64 = 36_000_000_000_000; // 10 hours
const MINIMUM_PRICE_PER_TICKET: Balance = ONE_NEAR / 10; // 0.1 NEAR

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ConfigContractData {
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
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ConfigLottery {
    pub time_run_lottery: u64,
    pub price_ticket_in_near: U128,
    pub discount_divisor: U128,
    pub rewards_breakdown: Vec<u128>,
    pub reserve_fee: U128,
    pub operate_fee: U128,
}

#[near_bindgen]
impl NearLott {
    /**
     * @notice Set operator, treasury, and injector addresses
     * @dev Only callable by owner
     * @param _operator_address: address of the operator
     * @param _treasury_address: address of the treasury
     * @param _injector_address: address of the injector
     * @param _max_number_tickets_per_buy: maximum number tickets per buy
     * @param _min_discount_divisor: minimum number tickets per buy
     */
    pub fn set_config(
        &mut self,
        _operator_address: AccountId,
        _treasury_address: AccountId,
        _injector_address: AccountId,
        _max_number_tickets_per_buy: u64,
        _min_discount_divisor: u128,
    ) {
        self.assert_owner_calling();
        let mut data = self.data_mut();
        data.operator_address = _operator_address;
        data.treasury_address = _treasury_address;
        data.injector_address = _injector_address;
        data.max_number_tickets_per_buy_or_claim = _max_number_tickets_per_buy;
        data.min_discount_divisor = _min_discount_divisor;
    }

    /**
     * @notice Set config for run a lottery
     * @dev Only callable by owner
     */
    pub fn set_config_lottery(&mut self, _config_lottery: ConfigLottery) {
        // only owner can call
        self.assert_owner_calling();
        assert!(
            _config_lottery.time_run_lottery >= LIMIT_TIME_IN_LOTTERY,
            "{}",
            ERR45_MINIMUM_TIME_FOR_RUN_LOTTERY
        );
        assert!(
            _config_lottery.price_ticket_in_near.0 >= MINIMUM_PRICE_PER_TICKET,
            "{}",
            ERR45_MINIMUM_TIME_FOR_RUN_LOTTERY
        );

        // get latest lotteryid
        let data = self.data_mut();

        // update
        data.config_lottery = _config_lottery;
    }

    /**
     * Get current config
     */
    pub fn _get_config(&self) -> ConfigContractData {
        let data = self.data();
        ConfigContractData {
            owner_id: data.owner_id.clone(),
            injector_address: data.injector_address.clone(),
            operator_address: data.operator_address.clone(),
            treasury_address: data.treasury_address.clone(),
            state: data.state.clone(),
            current_lottery_id: data.current_lottery_id,
            current_ticket_id: data.current_ticket_id,
            max_number_tickets_per_buy_or_claim: data.max_number_tickets_per_buy_or_claim,
            pending_injection_next_lottery: data.pending_injection_next_lottery,
            min_discount_divisor: data.min_discount_divisor,
            max_reserve_fee: data.max_reserve_fee,
        }
    }
}
