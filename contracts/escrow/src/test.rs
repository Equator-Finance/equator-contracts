#![cfg(test)]

use super::*;
use soroban_sdk::{Env, Address, String};

#[test]
fn test_create_forward() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EquatorEscrowContract);
    let client = EquatorEscrowContractClient::new(&env, &contract_id);

    let importer = Address::generate(&env);
    let pair = String::from_str(&env, "NGN/USD");
    
    env.mock_all_auths();

    let id = client.create_forward(
        &importer,
        &pair,
        &100_000_0000000i128,
        &15500000u128,
        &20_000_0000000i128,
        &(env.ledger().timestamp() + 86400 * 90),
    );

    assert_eq!(id, 1u64);
}
