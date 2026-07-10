//! # `stake-vault` — Time-Locked Stake → Multi-Tier Multiplier
//!
//! Learners stake a single **SAC staking token** (KAYSTICHS on testnet)
//! into the vault and receive one of three reward multipliers:
//!
//! | Stake amount       | Multiplier | Notes                |
//! |--------------------|-----------|----------------------|
//! | `>= 500`           | `200`     | 2.0 × boost (top)    |
//! | `>= 100`           | `120`     | 1.2 × boost (mid)    |
//! | `< 100`            | `100`     | 1.0 × (no boost)     |
//!
//! The 7-day lock (`604 800 s`) is **reset** on every stake-write; the
//! contract deliberately has no early-unlock function.
//!
//! Read [`crate::types`] for the storage shapes.
#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, token, Address, BytesN, Env};

pub mod types;
use types::{DataKey, StakeInfo};

#[contract]
pub struct StakeVault;

#[contractevent]
pub struct StakeVaultInitialized {
    #[topic]
    pub admin: Address,
    #[topic]
    pub token: Address,
}

#[contractevent]
pub struct Staked {
    #[topic]
    pub user: Address,
    pub amount: i128,
    pub total_staked: i128,
    pub lock_timestamp: u64,
}

#[contractevent]
pub struct Unstaked {
    #[topic]
    pub user: Address,
    pub amount: i128,
}

#[contractevent]
pub struct ContractUpgraded {
    #[topic]
    pub admin: Address,
    pub new_wasm_hash: BytesN<32>,
}

#[contractimpl]
impl StakeVault {
    pub fn initialize(env: Env, admin: Address, token: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }

        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);

        StakeVaultInitialized { admin, token }.publish(&env);
    }

    pub fn stake(env: Env, user: Address, amount: i128) {
        user.require_auth();

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        let token_id: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .expect("Not initialized");
        let token_client = token::Client::new(&env, &token_id);

        token_client.transfer(&user, env.current_contract_address(), &amount);

        let now = env.ledger().timestamp();

        let mut stake_info: StakeInfo = env
            .storage()
            .persistent()
            .get(&DataKey::UserStake(user.clone()))
            .unwrap_or(StakeInfo {
                amount: 0,
                lock_timestamp: now,
            });

        stake_info.amount += amount;
        stake_info.lock_timestamp = now;

        env.storage()
            .persistent()
            .set(&DataKey::UserStake(user.clone()), &stake_info);

        Staked {
            user,
            amount,
            total_staked: stake_info.amount,
            lock_timestamp: stake_info.lock_timestamp,
        }
        .publish(&env);
    }

    pub fn unstake(env: Env, user: Address) {
        user.require_auth();

        let stake_info: StakeInfo = env
            .storage()
            .persistent()
            .get(&DataKey::UserStake(user.clone()))
            .expect("No stake found");

        let lock_period: u64 = 604800;
        if env.ledger().timestamp() < stake_info.lock_timestamp + lock_period {
            panic!("Lock period active");
        }

        let token_id: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .expect("Not initialized");
        let token_client = token::Client::new(&env, &token_id);

        token_client.transfer(
            &env.current_contract_address(),
            user.clone(),
            &stake_info.amount,
        );

        env.storage()
            .persistent()
            .remove(&DataKey::UserStake(user.clone()));

        Unstaked {
            user,
            amount: stake_info.amount,
        }
        .publish(&env);
    }

    pub fn get_multiplier(env: Env, user: Address) -> u32 {
        let stake_info: StakeInfo = env
            .storage()
            .persistent()
            .get(&DataKey::UserStake(user))
            .unwrap_or(StakeInfo {
                amount: 0,
                lock_timestamp: 0,
            });

        if stake_info.amount >= 500 {
            200
        } else if stake_info.amount >= 100 {
            120
        } else {
            100
        }
    }

    pub fn upgrade_contract(env: Env, admin: Address, new_wasm_hash: BytesN<32>) {
        admin.require_auth();

        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("Not initialized");
        if admin != stored_admin {
            panic!("Unauthorized");
        }

        env.deployer()
            .update_current_contract_wasm(new_wasm_hash.clone());

        ContractUpgraded {
            admin,
            new_wasm_hash,
        }
        .publish(&env);
    }
}

mod test;
