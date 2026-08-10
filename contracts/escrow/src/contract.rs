use soroban_sdk::{contract, contractimpl, symbol_short, token, Address, Env, String, Symbol};
use crate::types::{ContractStatus, ForwardContract};

const COUNTER_KEY: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct EquatorEscrowContract;

#[contractimpl]
impl EquatorEscrowContract {
    /// Initialize a new NDF forward contract and deposit Importer margin
    pub fn create_forward(
        env: Env,
        importer: Address,
        usdc_token: Address,
        oracle_address: Address,
        currency_pair: String,
        notional_usd: i128,
        strike_rate: u128,
        importer_margin: i128,
        maturity_timestamp: u64,
    ) -> u64 {
        importer.require_auth();

        // Parameter & Sanity Validations
        assert!(notional_usd > 0, "Notional must be positive");
        assert!(strike_rate > 0, "Strike rate must be positive");
        assert!(importer_margin > 0, "Importer margin must be positive");
        assert!(
            maturity_timestamp > env.ledger().timestamp(),
            "Maturity timestamp must be in the future"
        );

        // Increment & fetch contract ID
        let mut contract_id: u64 = env.storage().persistent().get(&COUNTER_KEY).unwrap_or(0);
        contract_id += 1;
        env.storage().persistent().set(&COUNTER_KEY, &contract_id);

        // Lock importer's USDC margin in escrow contract
        let token_client = token::Client::new(&env, &usdc_token);
        token_client.transfer(&importer, &env.current_contract_address(), &importer_margin);

        let current_time = env.ledger().timestamp();

        let forward = ForwardContract {
            contract_id,
            importer: importer.clone(),
            market_maker: importer.clone(), // Placeholder until funded by desk
            usdc_token,
            oracle_address,
            currency_pair,
            notional_usd,
            strike_rate,
            settlement_rate: 0,
            importer_margin,
            market_maker_margin: 0,
            creation_timestamp: current_time,
            maturity_timestamp,
            payout_importer: 0,
            payout_desk: 0,
            status: ContractStatus::Created,
            rehypothecation_enabled: false,
        };

        env.storage().persistent().set(&contract_id, &forward);
        contract_id
    }

    /// OTC Desk accepts quote and deposits matching USDC margin to activate the forward
    pub fn fund_desk_margin(
        env: Env,
        contract_id: u64,
        market_maker: Address,
        market_maker_margin: i128,
    ) {
        market_maker.require_auth();

        let mut forward: ForwardContract = env
            .storage()
            .persistent()
            .get(&contract_id)
            .expect("Contract not found");

        assert_eq!(forward.status, ContractStatus::Created, "Contract is not in Created state");
        assert!(market_maker_margin > 0, "Market maker margin must be positive");
        assert!(market_maker != forward.importer, "Market maker cannot be the importer");

        // Update state before token transfer (Checks-Effects-Interactions)
        forward.market_maker = market_maker.clone();
        forward.market_maker_margin = market_maker_margin;
        forward.status = ContractStatus::Active;

        env.storage().persistent().set(&contract_id, &forward);

        // Lock desk's USDC margin
        let token_client = token::Client::new(&env, &forward.usdc_token);
        token_client.transfer(&market_maker, &env.current_contract_address(), &market_maker_margin);
    }

    /// Settle forward contract at maturity using the oracle exchange rate
    pub fn settle_forward(env: Env, contract_id: u64, settlement_rate: u128) {
        let mut forward: ForwardContract = env
            .storage()
            .persistent()
            .get(&contract_id)
            .expect("Contract not found");

        // Require oracle authorization to prevent unauthorized rate manipulation
        forward.oracle_address.require_auth();

        assert_eq!(forward.status, ContractStatus::Active, "Contract is not Active");
        assert!(
            env.ledger().timestamp() >= forward.maturity_timestamp,
            "Maturity timestamp not yet reached"
        );
        assert!(forward.strike_rate > 0, "Invalid zero strike rate");

        let total_collateral = forward.importer_margin + forward.market_maker_margin;

        // Calculate NDF P&L delta based on strike vs settlement rate
        // P&L Delta = Notional * (Settlement Rate - Strike Rate) / Strike Rate
        let diff = settlement_rate as i128 - forward.strike_rate as i128;
        let payout_delta = (forward.notional_usd * diff) / (forward.strike_rate as i128);

        let importer_return = forward.importer_margin + payout_delta;
        let payout_importer = if importer_return < 0 {
            0
        } else if importer_return > total_collateral {
            total_collateral
        } else {
            importer_return
        };

        let payout_desk = total_collateral - payout_importer;

        // Update state BEFORE external transfers (Checks-Effects-Interactions)
        forward.settlement_rate = settlement_rate;
        forward.payout_importer = payout_importer;
        forward.payout_desk = payout_desk;
        forward.status = ContractStatus::Settled;

        env.storage().persistent().set(&contract_id, &forward);

        // Execute token disbursements
        let token_client = token::Client::new(&env, &forward.usdc_token);

        if payout_importer > 0 {
            token_client.transfer(&env.current_contract_address(), &forward.importer, &payout_importer);
        }
        if payout_desk > 0 {
            token_client.transfer(&env.current_contract_address(), &forward.market_maker, &payout_desk);
        }
    }

    /// Cancel un-matched contract before funding and refund Importer margin
    pub fn cancel_forward(env: Env, contract_id: u64) {
        let mut forward: ForwardContract = env
            .storage()
            .persistent()
            .get(&contract_id)
            .expect("Contract not found");

        forward.importer.require_auth();
        assert_eq!(forward.status, ContractStatus::Created, "Cannot cancel non-Created contract");

        // Update state BEFORE external transfer
        forward.status = ContractStatus::Cancelled;
        env.storage().persistent().set(&contract_id, &forward);

        let token_client = token::Client::new(&env, &forward.usdc_token);
        token_client.transfer(&env.current_contract_address(), &forward.importer, &forward.importer_margin);
    }

    /// Retrieve an existing forward contract by ID
    pub fn get_forward(env: Env, contract_id: u64) -> Option<ForwardContract> {
        env.storage().persistent().get(&contract_id)
    }
}
