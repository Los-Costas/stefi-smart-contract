use soroban_sdk::{contracttype, Address, String, Map};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Asset {
    pub title: String,
    pub monthly_fee: i128,
    pub total: i128,
    pub purchased: bool,
    pub deadline: u64,
    pub next_due_date: u64,
    pub grace_period_end: u64,
    pub client: User,
    pub asset_provider: User,
    pub token: Address, 
    // pub monthly_payouts: Map<u64, i128>,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub name: String,
    pub address: Address,
}

#[contracttype]
#[derive(Clone)]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct AllowanceDataKey {
    pub from: Address,
    pub spender: Address,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct AddressBalance {
    pub address: Address,
    pub balance: i128,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Asset,
    Admin,        
    Token,           
    ContractBalance, 
    Recieve(Address), 
    Balance(Address),
    Allowance(AllowanceDataKey),
}