use crate::*;

#[near_bindgen]
impl NearLott {
    /**
     * @notice Inject funds
     * @param _lotteryId: lottery id
     * @param _amount: amount to inject in NEAR token
     * @dev Callable by owner or injector address
     */
    #[payable]
    pub fn inject_funds(&mut self, _lottery_id: LotteryId, _amount: u128) {
        self.assert_injector_or_owner_calling();

        assert!(
            env::attached_deposit() >= _amount,
            "{}",
            ERR16_ATTACHED_DEPOSIT_LESS_AMOUNT
        );

        let data = self.data_mut();
        let mut lottery: Lottery = data
            ._lotteries
            .get(&_lottery_id)
            .expect(ERR1_NOT_EXISTING_LOTTERY);

        assert_eq!(lottery.status, Status::Open, "{}", ERR17_LOTTERY_IS_NOT_OPEN);

        let new_near_fund = lottery.amount_collected_in_near + env::attached_deposit();
        lottery.amount_collected_in_near = new_near_fund;

        // save lottery
        data._lotteries.insert(&_lottery_id, &lottery);

        env::log_str(
            &json!({
                "type": "inject_funds",
                "params": {
                    "lottery_id": _lottery_id,
                    "amount": env::attached_deposit(),
                }
            })
            .to_string(),
        );
    }

    /// Get the owner of this contract
    pub fn get_owner(&self) -> AccountId {
        self.data().owner_id.clone()
    }

    /// Change owner. Only can be called by owner
    #[payable]
    pub fn set_owner(&mut self, owner_id: AccountId) {
        self.assert_one_yoctor();
        self.assert_owner_calling();
        self.data_mut().owner_id = owner_id.into();
    }

    #[payable]
    pub fn pause_contract(&mut self) {
        self.assert_one_yoctor();
        self.assert_owner_calling();

        if self.data().state == RunningState::Running {
            let msg = format!("Contract paused by {}", env::predecessor_account_id());
            env::log_str(&msg);
            self.data_mut().state = RunningState::Paused;
        } else {
            env::log_str(&"Contract state is already in Paused");
        }
    }
    #[payable]
    pub fn resume_contract(&mut self) {
        self.assert_one_yoctor();
        self.assert_owner_calling();

        if self.data().state == RunningState::Paused {
            env::log_str(&format!(
                "Contract resumed by {}",
                env::predecessor_account_id()
            ));
            self.data_mut().state = RunningState::Running;
        } else {
            env::log_str("Contract state is already in Running");
        }
    }

    /// Migration function between versions
    /// For next version upgrades, change this function
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let mut contract: NearLott = env::state_read().expect("ERR_NOT_INITIALIZED");
        contract.data = match contract.data {
            VersionedContractData::V0001(data) => VersionedContractData::V0001(data.into()),
        };
        contract
    }
}

#[cfg(target_arch = "wasm32")]
mod upgrade {
    use near_sdk::env::BLOCKCHAIN_INTERFACE;
    use near_sdk::Gas;

    use super::*;

    const BLOCKCHAIN_INTERFACE_NOT_SET_ERR: &str = "Blockchain interface not set.";

    /// Gas for calling migration call.
    pub const GAS_FOR_MIGRATE_CALL: Gas = 10_000_000_000_000;

    /// Self upgrade and call migrate, optimizes gas by not loading into memory the code.
    /// Takes as input non serialized set of bytes of the code.
    #[no_mangle]
    pub extern "C" fn upgrade() {
        env::setup_panic_hook();
        env::set_blockchain_interface(Box::new(near_blockchain::NearBlockchain {}));
        let contract: Contract = env::state_read().expect("ERR_CONTRACT_IS_NOT_INITIALIZED");
        contract.assert_owner();
        let current_id = env::current_account_id().into_bytes();
        let method_name = "migrate".as_bytes().to_vec();
        unsafe {
            BLOCKCHAIN_INTERFACE.with(|b| {
                // Load input into register 0.
                b.borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .input(0);
                let promise_id = b
                    .borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .promise_batch_create(current_id.len() as _, current_id.as_ptr() as _);
                b.borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .promise_batch_action_deploy_contract(promise_id, u64::MAX as _, 0);
                let attached_gas = env::prepaid_gas() - env::used_gas() - GAS_FOR_MIGRATE_CALL;
                b.borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .promise_batch_action_function_call(
                        promise_id,
                        method_name.len() as _,
                        method_name.as_ptr() as _,
                        0 as _,
                        0 as _,
                        0 as _,
                        attached_gas,
                    );
            });
        }
    }
}
