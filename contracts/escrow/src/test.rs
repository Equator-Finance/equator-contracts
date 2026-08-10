#![cfg(test)]

use super::*;
use soroban_sdk::testutils::{Address as _, Ledger as _};
use soroban_sdk::{token, Address, Env, String};
use crate::types::ContractStatus;

#[test]
fn test_full_ndf_lifecycle() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, EquatorEscrowContract);
    let client = EquatorEscrowContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let importer = Address::generate(&env);
    let desk = Address::generate(&env);
    let oracle = Address::generate(&env);
    
    // Register mock SAC token
    let sac = env.register_stellar_asset_contract(admin.clone());
    let token_admin_client = token::StellarAssetClient::new(&env, &sac);
    let token_client = token::Client::new(&env, &sac);

    // Mint initial balances (10,000 USDC each with 7 decimals)
    let initial_balance = 10_000_0000000i128;
    token_admin_client.mint(&importer, &initial_balance);
    token_admin_client.mint(&desk, &initial_balance);

    let currency_pair = String::from_str(&env, "NGN/USD");
    let notional = 100_000_0000000i128; // $100,000 notional
    let strike_rate = 1500_0000u128;   // 1500 NGN/USD
    let importer_margin = 1_000_0000000i128; // $1,000 collateral
    let desk_margin = 1_000_0000000i128;     // $1,000 collateral
    let maturity_time = env.ledger().timestamp() + 86400 * 90; // 90 days

    // 1. Importer creates forward contract
    let id = client.create_forward(
        &importer,
        &sac,
        &oracle,
        &currency_pair,
        &notional,
        &strike_rate,
        &importer_margin,
        &maturity_time,
    );

    assert_eq!(id, 1u64);
    assert_eq!(token_client.balance(&importer), initial_balance - importer_margin);
    assert_eq!(token_client.balance(&contract_id), importer_margin);

    // 2. OTC Desk funds margin
    client.fund_desk_margin(&id, &desk, &desk_margin);
    assert_eq!(token_client.balance(&desk), initial_balance - desk_margin);
    assert_eq!(token_client.balance(&contract_id), importer_margin + desk_margin);

    let fwd = client.get_forward(&id).unwrap();
    assert_eq!(fwd.status, ContractStatus::Active);

    // 3. Fast-forward ledger time to maturity
    env.ledger().with_mut(|li| li.timestamp = maturity_time + 1);

    // 4. Settle forward at maturity rate of 1650 NGN/USD (Naira depreciated -> Importer payout)
    let settlement_rate = 1650_0000u128;
    client.settle_forward(&id, &settlement_rate);

    let settled_fwd = client.get_forward(&id).unwrap();
    assert_eq!(settled_fwd.status, ContractStatus::Settled);
    assert_eq!(settled_fwd.settlement_rate, settlement_rate);

    // Assert that contract balance is drained back to parties
    assert_eq!(token_client.balance(&contract_id), 0);
    assert!(token_client.balance(&importer) > initial_balance - importer_margin);
}
