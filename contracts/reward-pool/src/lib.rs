#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, token, Address, Env};

pub mod types;
use types::DataKey;

#[contract]
pub struct RewardPool;

#[contractevent]
pub struct PoolInitialized {
    #[topic]
    pub admin: Address,
    #[topic]
    pub token: Address,
}

#[contractevent]
pub struct SpenderAdded {
    #[topic]
    pub spender: Address,
}

#[contractevent]
pub struct RewardDistributed {
    #[topic]
    pub caller: Address,
    #[topic]
    pub learner: Address,
    pub amount: i128,
}

#[contractevent]
pub struct PoolFunded {
    #[topic]
    pub donor: Address,
}

use types::DataKey;

pub fn setup() -> (Env, RewardPoolClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();

    // Fixed: Passing the contract type first, and empty constructor args second
    let contract_id = env.register(RewardPool);

    let client = RewardPoolClient::new(&env, &contract_id);
    (env, client)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RewardPool, RewardPoolClient};

    #[test]
    fn test_setup() {
        let (env, client) = setup();
        // Perform test logic here
    }
}