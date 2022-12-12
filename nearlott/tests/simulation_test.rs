use crate::utils::{init, DEFAULT_GAS};
use contract::LotteryUserData;
use near_contract_standards::storage_management::StorageBalance;
use near_contract_standards::storage_management::StorageBalanceBounds;
use near_sdk::{
    json_types::{U128},
    serde_json::json,
    AccountId,
};
use near_sdk_sim::{to_yocto, view};
mod utils;

#[test]
fn test_new() {
    let (nearlott_contract, _owner, _operator, _injector, _treasury, _, _, _, _, _) = init();

    let owner_id: AccountId = view!(nearlott_contract.get_owner()).unwrap_json();
    assert_eq!(owner_id, _owner.account_id());
}

#[test]
fn test_storage_deposit() {
    let (
        nearlott_contract,
        _owner,
        _operator,
        _injector,
        _treasury,
        _alice,
        _bob,
        chandra,
        _darmaji,
        _root,
    ) = init();

    chandra
        .call(
            nearlott_contract.account_id(),
            "storage_deposit",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            500000000000000000000000,
        )
        .assert_success();
}

#[test]
fn lotter_actions() {
    let (
        nearlott_contract,
        _owner,
        _operator,
        _injector,
        _treasury,
        alice,
        _bob,
        chandra,
        _darmaji,
        _root,
    ) = init();

    let minium_deposit_amount: StorageBalanceBounds = chandra
        .view(
            nearlott_contract.account_id(),
            "storage_balance_bounds",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();
    println!("minium_deposit_amount: {:?}", minium_deposit_amount.min);

    const SEP_8_2022: u64 = 1633046400000000000;
    const ONE_DAY: u64 = 86400000000000;

    let _msg = &json!({
      "_end_time": "1633046400000000000",
      "_price_ticket_in_near": "1000000000000000000000000",
      "_discount_divisor": "2000",
      "_rewards_breakdown": "[125, 375, 750, 1250, 2500, 5000]",
      "_treasury_fee": "2000",
    })
    .to_string();

    // deposit near to cover contract
    _operator
        .call(
            nearlott_contract.account_id(),
            "storage_deposit",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            minium_deposit_amount.min.0,
        )
        .assert_success();
    _operator.borrow_runtime_mut().cur_block.block_timestamp = SEP_8_2022 - ONE_DAY;
    _operator
        .call(
            nearlott_contract.account_id(),
            "start_lottery",
            &json!({
                "_end_time": SEP_8_2022,
                "_price_ticket_in_near": U128(to_yocto("1")),
                "_discount_divisor":U128(2000),
                "_rewards_breakdown": [125, 375, 750, 1250, 2500, 5000],
                "_treasury_fee": U128(2000),

            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS,
            1,
        )
        .assert_success();

    // check market data. There is no bid
    let current_lottery_id: u32 = alice
        .view(
            nearlott_contract.account_id(),
            "view_latest_lottery_id",
            &json!({}).to_string().into_bytes(),
        )
        .unwrap_json();
    assert_eq!(current_lottery_id, 1);

    // chans buy some tickets
    chandra
        .call(
            nearlott_contract.account_id(),
            "storage_deposit",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            minium_deposit_amount.min.0 * 5,
        )
        .assert_success();

    // Do a loop buying 2000 tickets
    for i in 0..2000 {
        // check storage available
        let chandra_storage: StorageBalance = chandra
            .view(
                nearlott_contract.account_id(),
                "debug_storage_balance_of",
                &json!({
                    "account_id": chandra.account_id(),
                })
                .to_string()
                .into_bytes(),
            )
            .unwrap_json();
        println!(
            "storage_available: {:?}: {:?}, {:?}",
            i, chandra_storage.available.0, chandra_storage.total.0
        );

        chandra
            .call(
                nearlott_contract.account_id(),
                "buy_tickets",
                &json!({
                    "_lottery_id": 1,
                    "_ticket_numbers": [1039219],
                    "_amount": U128(to_yocto("1"))

                })
                .to_string()
                .into_bytes(),
                DEFAULT_GAS,
                to_yocto("1"),
            )
            .assert_success();
    }
    // get view_user_info_for_lottery_id
    let view_user_info_for_lottery_id: LotteryUserData = chandra
        .view(
            nearlott_contract.account_id(),
            "view_user_info_for_lottery_id",
            &json!({
                "_user": chandra.account_id(),
                "_lottery_id": 1,
                "_cursor": 0,
                "_size": 25,

            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();
    println!(
        "view_user_info_for_lottery_id: {:?}",
        view_user_info_for_lottery_id
    );

    assert_eq!(view_user_info_for_lottery_id.ticket_numbers.len(), 25);
    assert_eq!(view_user_info_for_lottery_id.ticket_numbers[0], 1039219);
}
