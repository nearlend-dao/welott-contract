use crate::*;

#[derive(Serialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, Deserialize))]
#[serde(crate = "near_sdk::serde")]
pub struct AccountSimpleView {
    /// A copy of an account ID. Saves one storage_read when iterating on accounts.
    pub account_id: AccountId,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Account {
    /// A copy of an account ID. Saves one storage_read when iterating on accounts.
    pub account_id: AccountId,

    /// Tracks changes in storage usage by persistent collections in this account.
    #[borsh_skip]
    #[serde(skip_serializing)]
    pub storage_tracker: StorageTracker,

    // keep track of user ticket ids for a given lotteryId
    #[serde(skip_serializing)]
    pub tickets: UnorderedMap<LotteryId, Vec<TicketId>>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VAccount {
    Current(Account),
}

impl From<VAccount> for Account {
    fn from(v: VAccount) -> Self {
        match v {
            VAccount::Current(c) => c,
        }
    }
}

impl From<Account> for VAccount {
    fn from(c: Account) -> Self {
        VAccount::Current(c)
    }
}

impl Account {
    pub fn new(account_id: &AccountId) -> Self {
        Self {
            account_id: account_id.clone(),
            storage_tracker: Default::default(),
            tickets: UnorderedMap::new(StorageKey::AccountTickets {
                account_id: account_id.clone(),
            }),
        }
    }
}

impl NearLott {
    pub fn internal_get_account(&self, account_id: &AccountId) -> Option<Account> {
        self.data().accounts.get(account_id).map(|o| o.into())
    }

    pub fn internal_unwrap_account(&self, account_id: &AccountId) -> Account {
        self.internal_get_account(account_id)
            .expect(ERR42_ACCOUNT_NO_EXISTING)
    }
    pub fn internal_set_account(&mut self, account_id: &AccountId, mut account: Account) {
        let mut storage = self.internal_unwrap_storage(account_id);
        storage
            .storage_tracker
            .consume(&mut account.storage_tracker);
        storage.storage_tracker.start();
        self.data_mut().accounts.insert(account_id, &account.into());
        storage.storage_tracker.stop();
        self.internal_set_storage(account_id, storage);
    }

    pub fn internal_set_account_contract_data(
        data: &mut ContractData,
        account_id: &AccountId,
        mut account: Account,
    ) {
        let mut storage: Storage = data
            .storage
            .get(account_id)
            .map(|o| o.into())
            .expect("Storage for account is missing");

        storage
            .storage_tracker
            .consume(&mut account.storage_tracker);
        storage.storage_tracker.start();
        data.accounts.insert(account_id, &account.into());
        storage.storage_tracker.stop();
        internal_set_storage_data(data, account_id, storage);
    }
}

#[near_bindgen]
impl NearLott {
    /// Returns limited account information for accounts from a given index up to a given limit.
    pub fn get_accounts_paged(&self, from_index: Option<u64>, limit: Option<u64>) -> Vec<Account> {
        let values = self.data().accounts.values_as_vector();
        let from_index = from_index.unwrap_or(0);
        let limit = limit.unwrap_or(values.len());
        (from_index..std::cmp::min(values.len(), from_index + limit))
            .map(|index| values.get(index).unwrap().into())
            .collect()
    }

    /// Returns the number of accounts
    pub fn get_num_accounts(&self) -> u32 {
        self.data().accounts.len() as _
    }
}

impl Account {
    pub fn internal_unwrap_ticket_ids_per_lottery(&self, _lottery_id: &LotteryId) -> Vec<TicketId> {
        self.internal_get_ticket_ids_per_lottery(_lottery_id)
            .expect(ERR19_LOTTERY_NO_TICKERS_NUMBERS)
    }

    pub fn internal_get_ticket_ids_per_lottery(
        &self,
        _lottery_id: &LotteryId,
    ) -> Option<Vec<TicketId>> {
        self.tickets.get(_lottery_id)
    }

    pub fn internal_get_ticket_id_per_lottery_or_default(
        &mut self,
        _lottery_id: &LotteryId,
    ) -> Vec<TicketId> {
        self.internal_get_ticket_ids_per_lottery(_lottery_id)
            .unwrap_or_default()
    }

    pub fn internal_set_ticket_ids_per_lottery(
        &mut self,
        _lottery_id: &LotteryId,
        ticket_id: TicketId,
    ) {
        let mut ticket_ids = self.internal_get_ticket_id_per_lottery_or_default(_lottery_id);
        ticket_ids.push(ticket_id);
        self.tickets.insert(_lottery_id, &ticket_ids);
    }
}
