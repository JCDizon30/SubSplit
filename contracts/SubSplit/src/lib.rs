#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Symbol, Vec, Map,
};

#[contracttype]
#[derive(Clone)]
pub struct Pool {
    pub owner: Address,
    pub monthly_amount: i128,
    pub max_members: u32,
    pub active_members: Vec<Address>,
    pub paid_members: Vec<Address>,
}

#[contracttype]
pub enum DataKey {
    Pool,
}

#[contract]
pub struct SubSplitContract;

#[contractimpl]
impl SubSplitContract {

    // Create a new subscription pool
    pub fn create_pool(
        env: Env,
        owner: Address,
        monthly_amount: i128,
        max_members: u32,
    ) {
        owner.require_auth();

        let pool = Pool {
            owner,
            monthly_amount,
            max_members,
            active_members: Vec::new(&env),
            paid_members: Vec::new(&env),
        };

        env.storage().instance().set(&DataKey::Pool, &pool);
    }

    // Join the subscription pool
    pub fn join_pool(env: Env, member: Address) {
        member.require_auth();

        let mut pool: Pool = env
            .storage()
            .instance()
            .get(&DataKey::Pool)
            .unwrap();

        if pool.active_members.len() >= pool.max_members {
            panic!("Pool full");
        }

        pool.active_members.push_back(member);

        env.storage().instance().set(&DataKey::Pool, &pool);
    }

    // Pay monthly share
    pub fn pay_share(env: Env, member: Address) {
        member.require_auth();

        let mut pool: Pool = env
            .storage()
            .instance()
            .get(&DataKey::Pool)
            .unwrap();

        pool.paid_members.push_back(member);

        env.storage().instance().set(&DataKey::Pool, &pool);
    }

    // Verify if all members paid
    pub fn all_paid(env: Env) -> bool {
        let pool: Pool = env
            .storage()
            .instance()
            .get(&DataKey::Pool)
            .unwrap();

        pool.active_members.len() == pool.paid_members.len()
    }

    // Remove unpaid members
    pub fn revoke_unpaid(env: Env) {
        let mut pool: Pool = env
            .storage()
            .instance()
            .get(&DataKey::Pool)
            .unwrap();

        let mut updated = Vec::new(&env);

        for member in pool.active_members.iter() {
            if pool.paid_members.contains(member.clone()) {
                updated.push_back(member);
            }
        }

        pool.active_members = updated;

        env.storage().instance().set(&DataKey::Pool, &pool);
    }

    // Get pool info
    pub fn get_pool(env: Env) -> Pool {
        env.storage()
            .instance()
            .get(&DataKey::Pool)
            .unwrap()
    }
}