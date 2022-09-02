use crate::*;

impl NearLott {
    /**
     * @notice Set operator, treasury, and injector addresses
     * @dev Only callable by owner
     * @param _operator_address: address of the operator
     * @param _treasury_address: address of the treasury
     * @param _injector_address: address of the injector
     */
    pub fn set_operator_and_treasury_and_injector_addresses(
        &mut self,
        _operator_address: AccountId,
        _treasury_address: AccountId,
        _injector_address: AccountId,
    ) {
        self.assert_owner_calling();

        let mut data = self.data_mut();
        data.operator_address = _operator_address;
        data.treasury_address = _treasury_address;
        data.injector_address = _injector_address;
    }

    /**
     * @notice Set max number of tickets
     * @dev Only callable by owner
     */
    pub fn set_max_number_tickets_per_buy(&mut self, _max_number_tickets_per_buy: u128) {
        self.assert_owner_calling();
        let data = self.data_mut();
        data.max_number_tickets_per_buy_or_claim = _max_number_tickets_per_buy;
    }

    /**
     * @notice Set NEAR price ticket upper/lower limit
     * @dev Only callable by owner
     * @param _min_price_ticket_in_near: minimum price of a ticket in NEAR
     * @param _max_price_ticket_in_near: maximum price of a ticket in NEAR
     */
    pub fn set_min_and_max_ticket_price_in_near(
        &mut self,
        _min_price_ticket_in_near: u128,
        _max_price_ticket_in_near: u128,
    ) {
        self.assert_owner_calling();

        assert!(
            _min_price_ticket_in_near <= _max_price_ticket_in_near,
            ERR8_MIN_PRICE_MAX_PRICE
        );

        let data = self.data_mut();
        data.min_price_ticket_in_near = _min_price_ticket_in_near;
        data.max_price_ticket_in_near = _max_price_ticket_in_near;
    }
}
