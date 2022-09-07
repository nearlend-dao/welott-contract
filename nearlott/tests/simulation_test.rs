use crate::utils::{account_o, init, DEFAULT_GAS};
use contract::INIT_ACCOUNT_STORAGE;
use near_sdk::{
    json_types::{U128, U64},
    serde::{Deserialize, Serialize},
    serde_json::json,
    AccountId, Gas,
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
        alice,
        bob,
        chandra,
        darmaji,
        root,
    ) = init();

    chandra
        .call(
            nearlott_contract.account_id(),
            "storage_deposit",
            &json!({}).to_string().into_bytes(),
            DEFAULT_GAS,
            (INIT_ACCOUNT_STORAGE as u64).into(),
        )
        .assert_success();
}
