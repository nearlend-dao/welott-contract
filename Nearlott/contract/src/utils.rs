use crate::callback::ext_ft_contract;
use crate::gas::GAS_FOR_FT_TRANSFER;
use crate::*;

use near_sdk::AccountId;

#[near_bindgen]
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
