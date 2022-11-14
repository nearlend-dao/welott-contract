use crate::*;

/********************************/
/* CONTRACT Self Identification */
/********************************/
// [NEP-129](https://github.com/nearprotocol/NEPs/pull/129)
// see also pub fn get_contract_info
pub const CONTRACT_NAME: &str = "WeLott";
pub const CONTRACT_VERSION: &str = "0.0.1";
pub const DEFAULT_WEB_APP_URL: &str = "https://www.welott.nearlenddao.com";
pub const DEFAULT_AUDITOR_ACCOUNT_ID: &str = "mitsori.near";
pub const DEVELOPERS_ACCOUNT_ID: &str = "mitsori.near";

/// NEP-129 get information about this contract
/// returns JSON string according to [NEP-129](https://github.com/nearprotocol/NEPs/pull/129)
/// Rewards fee fraction structure for the staking pool contract.
#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
#[allow(non_snake_case)]
pub struct NEP129Response {
    pub dataVersion: u16,
    pub name: String,
    pub version: String,
    pub source: String,
    pub standards: Vec<String>,
    pub webAppUrl: Option<String>,
    pub developersAccountId: String,
    pub auditorAccountId: Option<AccountId>,
}

impl Default for NEP129Response {
    fn default() -> Self {
        Self {
            dataVersion: 1,
            name: CONTRACT_NAME.into(),
            version: CONTRACT_VERSION.into(),
            source: "https://gitlab.com/nearlend/nearlott".into(),
            standards: vec!["NEP-141".into(), "NEP-145".into(), "SP".into()], //SP=>core-contracts/Staking-pool
            developersAccountId: DEVELOPERS_ACCOUNT_ID.into(),
            auditorAccountId: None,
            webAppUrl: None,
        }
    }
}

impl NearLott {
    /// NEP-129 get information about this contract
    /// returns JSON string according to [NEP-129](https://github.com/nearprotocol/NEPs/pull/129)
    pub fn get_contract_info(&self) -> NEP129Response {
        return NEP129Response {
            dataVersion: 1,
            name: CONTRACT_NAME.into(),
            version: CONTRACT_VERSION.into(),
            source: "https://gitlab.com/nearlend/nearlott".into(),
            standards: vec!["NEP-141".into(), "NEP-145".into(), "SP".into()], //SP=>core-contracts/Staking-pool
            webAppUrl: self.web_app_url.clone(),
            developersAccountId: DEVELOPERS_ACCOUNT_ID.into(),
            auditorAccountId: self.auditor_account_id.clone(),
        };
    }

    /// sets configurable contract info [NEP-129](https://github.com/nearprotocol/NEPs/pull/129)
    // Note: params are not Option<String> so the user can not inadvertently set null to data by not including the argument
    pub fn set_contract_info(&mut self, web_app_url: String, auditor_account_id: AccountId) {
        self.assert_owner_calling();
        self.web_app_url = if web_app_url.len() > 0 {
            Some(web_app_url)
        } else {
            None
        };
        self.auditor_account_id = if auditor_account_id.as_str().len() > 0 {
            Some(auditor_account_id)
        } else {
            None
        };
    }
}
