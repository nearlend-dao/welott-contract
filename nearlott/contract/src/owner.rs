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
            ERR39_ATTACHED_DEPOSIT_LESS_AMOUNT
        );

        let data = self.data_mut();
        let mut lottery: Lottery = data
            ._lotteries
            .get(&_lottery_id)
            .expect(ERR1_NOT_EXISTING_LOTTERY);

        assert!(
            lottery.status == Status::Open,
            "{}",
            ERR17_LOTTERY_IS_NOT_OPEN
        );

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
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "ERR_INVALID_PREDECESSOR"
        );
        let mut contract: NearLott = env::state_read().expect("ERR_NOT_INITIALIZED");
        contract.data = match contract.data {
            VersionedContractData::V0001(data) => VersionedContractData::V0001(data.into()),
        };
        contract
    }
}

mod upgrade {
    use near_sdk::{require, Gas};

    use super::*;
    use near_sys as sys;

    const GAS_TO_COMPLETE_UPGRADE_CALL: Gas = Gas(Gas::ONE_TERA.0 * 10);
    const GAS_FOR_GET_CONFIG_CALL: Gas = Gas(Gas::ONE_TERA.0 * 5);
    const MIN_GAS_FOR_MIGRATE_STATE_CALL: Gas = Gas(Gas::ONE_TERA.0 * 10);

    /// Self upgrade and call migrate, optimizes gas by not loading into memory the code.
    /// Takes as input non serialized set of bytes of the code.
    #[no_mangle]
    pub extern "C" fn upgrade() {
        env::setup_panic_hook();
        let contract: NearLott = env::state_read().expect("ERR_CONTRACT_IS_NOT_INITIALIZED");
        contract.assert_owner_calling();
        let current_account_id = env::current_account_id().as_bytes().to_vec();
        let migrate_method_name = b"migrate_state".to_vec();
        let get_config_method_name = b"get_config".to_vec();
        let empty_args = b"{}".to_vec();
        unsafe {
            sys::input(0);
            let promise_id = sys::promise_batch_create(
                current_account_id.len() as _,
                current_account_id.as_ptr() as _,
            );
            sys::promise_batch_action_deploy_contract(promise_id, u64::MAX as _, 0);
            // Gas required to complete this call.
            let required_gas =
                env::used_gas() + GAS_TO_COMPLETE_UPGRADE_CALL + GAS_FOR_GET_CONFIG_CALL;
            require!(
                env::prepaid_gas() >= required_gas + MIN_GAS_FOR_MIGRATE_STATE_CALL,
                "Not enough gas to complete state migration"
            );
            let migrate_state_attached_gas = env::prepaid_gas() - required_gas;
            // Scheduling state migration.
            sys::promise_batch_action_function_call(
                promise_id,
                migrate_method_name.len() as _,
                migrate_method_name.as_ptr() as _,
                empty_args.len() as _,
                empty_args.as_ptr() as _,
                0 as _,
                migrate_state_attached_gas.0,
            );
            // Scheduling to return config after the migration is completed.
            //
            // The upgrade method attaches it as an action, so the entire upgrade including deploy
            // contract action and migration can be rolled back if the config view call can't be
            // returned successfully. The view call deserializes the state and deserializes the
            // config which contains the owner_id. If the contract can deserialize the current config,
            // then it can validate the owner and execute the upgrade again (in case the previous
            // upgrade/migration went badly).
            //
            // It's an extra safety guard for the remote contract upgrades.
            sys::promise_batch_action_function_call(
                promise_id,
                get_config_method_name.len() as _,
                get_config_method_name.as_ptr() as _,
                empty_args.len() as _,
                empty_args.as_ptr() as _,
                0 as _,
                GAS_FOR_GET_CONFIG_CALL.0,
            );
            sys::promise_return(promise_id);
        }
    }
}
