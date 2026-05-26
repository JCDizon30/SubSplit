#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::{SubSplitContract, SubSplitContractClient};

mod tests {

    use super::*;

    #[test]
    fn test_happy_path() {
        let env = Env::default();

        let contract_id = env.register_contract(None, SubSplitContract);
        let client = SubSplitContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        let member = Address::generate(&env);

        client.create_pool(&owner, &100, &5);
        client.join_pool(&member);
        client.pay_share(&member);

        assert!(client.all_paid());
    }

    #[test]
    #[should_panic(expected = "Pool full")]
    fn test_pool_full() {
        let env = Env::default();

        let contract_id = env.register_contract(None, SubSplitContract);
        let client = SubSplitContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);

        client.create_pool(&owner, &100, &1);

        let member1 = Address::generate(&env);
        let member2 = Address::generate(&env);

        client.join_pool(&member1);
        client.join_pool(&member2);
    }

    #[test]
    fn test_storage_state() {
        let env = Env::default();

        let contract_id = env.register_contract(None, SubSplitContract);
        let client = SubSplitContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);
        let member = Address::generate(&env);

        client.create_pool(&owner, &100, &5);
        client.join_pool(&member);

        let pool = client.get_pool();

        assert_eq!(pool.active_members.len(), 1);
    }

    #[test]
    fn test_revoke_unpaid() {
        let env = Env::default();

        let contract_id = env.register_contract(None, SubSplitContract);
        let client = SubSplitContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);

        let member1 = Address::generate(&env);
        let member2 = Address::generate(&env);

        client.create_pool(&owner, &100, &5);

        client.join_pool(&member1);
        client.join_pool(&member2);

        client.pay_share(&member1);

        client.revoke_unpaid();

        let pool = client.get_pool();

        assert_eq!(pool.active_members.len(), 1);
    }

    #[test]
    fn test_not_all_paid() {
        let env = Env::default();

        let contract_id = env.register_contract(None, SubSplitContract);
        let client = SubSplitContractClient::new(&env, &contract_id);

        let owner = Address::generate(&env);

        let member1 = Address::generate(&env);
        let member2 = Address::generate(&env);

        client.create_pool(&owner, &100, &5);

        client.join_pool(&member1);
        client.join_pool(&member2);

        client.pay_share(&member1);

        assert!(!client.all_paid());
    }
}