use crate::*;

impl NearLott {
    /// Assert that the method was called by the owner.
    pub fn assert_owner_calling(&self) {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.data().owner_id,
            "Can only be called by the owner"
        )
    }

    /// Assert that the method was called by operator
    pub fn assert_operator_calling(&self) {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.data().operator_address,
            "Can only be called by the operator"
        )
    }

    /// only operator if is an owner of an operator
    pub fn assert_operator_or_owner_calling(&self) {
        assert!(
            &env::predecessor_account_id() == &self.data().owner_id
                || &env::predecessor_account_id() == &self.data().operator_address,
            "Can only be called by the operator or the owner"
        );
    }

    /// only injector or an operator calling
    pub fn assert_injector_or_owner_calling(&self) {
        assert!(
            &env::predecessor_account_id() == &self.data().owner_id
                || &env::predecessor_account_id() == &self.data().injector_address,
            "Can only be called by the injector or the owner"
        );
    }

    /// Get contract status
    pub fn assert_not_busy(&self) {
        assert_eq!(
            self.data().state,
            RunningState::Running,
            "Contract is busy. Try again later"
        );
    }

    /// Assert that 1 yoctorNEAR was attached
    pub fn assert_one_yoctor(&self) {
        assert_eq!(
            env::attached_deposit(),
            1,
            "Requires attached deposit of exactly 1 yoctorNEAR"
        )
    }
}
