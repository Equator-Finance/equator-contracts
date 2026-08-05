use soroban_sdk::{contract, contractimpl, Address, Env, String};
use crate::types::{ContractStatus, ForwardContract};

#[contract]
pub struct EquatorEscrowContract;

#[contractimpl]
impl EquatorEscrowContract {
    /// Initialize a new NDF forward contract and deposit Importer margin
    pub fn create_forward(
        env: Env,
        importer: Address,
        currency_pair: String,
        notional_usd: i128,
        strike_rate: u128,
        importer_margin: i128,
        maturity_timestamp: u64,
    ) -> u64 {
        importer.require_auth();
        
        let contract_id = 1u64; // Scaffold placeholder
        let current_time = env.ledger().timestamp();

        let forward = ForwardContract {
            contract_id,
            importer,
            market_maker: Address::generate(&env), // Placeholder until match
            currency_pair,
            notional_usd,
            strike_rate,
            importer_margin,
            market_maker_margin: 0,
            creation_timestamp: current_time,
            maturity_timestamp,
            status: ContractStatus::Created,
            rehypothecation_enabled: false,
        };

        env.storage().persistent().set(&contract_id, &forward);
        contract_id
    }

    /// Retrieve an existing forward contract by ID
    pub fn get_forward(env: Env, contract_id: u64) -> Option<ForwardContract> {
        env.storage().persistent().get(&contract_id)
    }
}
