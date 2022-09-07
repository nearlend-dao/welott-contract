use near_sdk::{serde_json::json, AccountId};
use near_sdk_sim::{
    deploy, init_simulator, to_yocto, ContractAccount, UserAccount, STORAGE_AMOUNT,
};

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    NEARLOTT_WASM_BYTES => "out/nearlott.wasm",
}
use contract::NearLottContract as NearlottContract;
pub const DEFAULT_GAS: u64 = near_sdk_sim::DEFAULT_GAS;

pub fn init() -> (
    ContractAccount<NearlottContract>,
    UserAccount,
    UserAccount,
    UserAccount,
    UserAccount,
    UserAccount,
    UserAccount,
    UserAccount,
    UserAccount,
    UserAccount,
) {
    let root = init_simulator(None);

    let treasury = root.create_user(
        AccountId::new_unchecked("treasury".to_string()),
        to_yocto("100"),
    );

    let injector = root.create_user(
        AccountId::new_unchecked("injector".to_string()),
        to_yocto("100"),
    );

    let operator = root.create_user(
        AccountId::new_unchecked("operator".to_string()),
        to_yocto("100"),
    );

    let owner = root.create_user(
        AccountId::new_unchecked("owner".to_string()),
        to_yocto("100"),
    );

    root.create_user(account_from(&"g"), to_yocto("100"));

    root.create_user(account_from(&"h"), to_yocto("100"));

    root.create_user(account_from(&"i"), to_yocto("100"));

    root.create_user(account_from(&"j"), to_yocto("100"));

    root.create_user(account_from(&"k"), to_yocto("100"));

    root.create_user(account_from(&"l"), to_yocto("100"));

    root.create_user(account_from(&"m"), to_yocto("100"));

    let alice = root.create_user(account_from(&"x"), to_yocto("100"));

    let bob = root.create_user(account_from(&"y"), to_yocto("100"));

    let chandra = root.create_user(account_from(&"z"), to_yocto("100"));

    let darmaji = root.create_user(account_from(&"n"), to_yocto("100"));

    let nearlott_contract = deploy!(
        contract: NearlottContract,
        contract_id: &AccountId::new_unchecked("mk".repeat(32)),
        bytes: &NEARLOTT_WASM_BYTES,
        signer_account: root,
        init_method: new(
            owner.account_id(),
            injector.account_id(),
            operator.account_id(),
            treasury.account_id()
        )
    );

    (
        nearlott_contract,
        owner,
        operator,
        injector,
        treasury,
        alice,
        bob,
        chandra,
        darmaji,
        root,
    )
}

pub fn account_o() -> AccountId {
    account_from("o")
}

pub fn account_from(s: &str) -> AccountId {
    AccountId::new_unchecked(s.repeat(64).to_string())
}
