use soroban_sdk::{contracttype, Address, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractStatus {
    Created,
    Funded,
    Active,
    Settled,
    Defaulted,
    Cancelled,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForwardContract {
    pub contract_id: u64,
    pub importer: Address,
    pub market_maker: Address,
    pub currency_pair: String,
    pub notional_usd: i128,
    pub strike_rate: u128,
    pub importer_margin: i128,
    pub market_maker_margin: i128,
    pub creation_timestamp: u64,
    pub maturity_timestamp: u64,
    pub status: ContractStatus,
    pub rehypothecation_enabled: bool,
}
