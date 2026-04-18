#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, token, Address, Env, String};

fn setup_test_env(env: &Env) -> (AyudaContractClient<'_>, Address, Address, String, Address) {
    let admin = Address::generate(env);
    let citizen = Address::generate(env);
    let contract_id = env.register(AyudaContract, ());
    let client = AyudaContractClient::new(env, &contract_id);

    let token_admin = Address::generate(env);
    let token_addr = env.register_stellar_asset_contract(token_admin);
    let token_client = token::StellarAssetClient::new(env, &token_addr);

    client.init(&admin, &token_addr);
    token_client.mint(&contract_id, &100000);

    let name = String::from_str(env, "Richie");

    client.register_citizen(&admin, &citizen, &name);

    (client, admin, citizen, name, token_addr)
}

#[test]
fn test_1_success_claim() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin, citizen, _, _) = setup_test_env(&env);

    client.fund_aid(&admin, &citizen, &5000);
    client.claim_aid(&citizen);
    assert_eq!(client.get_balance(&citizen), 0);
}

#[test]
#[should_panic(expected = "No record")]
fn test_2_unregistered_citizen_cannot_claim() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _, _, _, _) = setup_test_env(&env);
    let stranger = Address::generate(&env);

    client.claim_aid(&stranger);
}

#[test]
#[should_panic(expected = "Not admin")]
fn test_3_security_non_admin_registration_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _, citizen, _, _) = setup_test_env(&env);

    let fake_admin = Address::generate(&env);
    let name = String::from_str(&env, "Stranger");

    // This should panic because fake_admin != actual admin
    client.register_citizen(&fake_admin, &citizen, &name);
}

#[test]
#[should_panic(expected = "No aid available")]
fn test_4_zero_balance_claim_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _, citizen, _, _) = setup_test_env(&env);

    // Citizen is registered but balance is 0
    client.claim_aid(&citizen);
}

#[test]
fn test_5_multiple_citizens_claim_independently() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, admin, citizen_1, _, _) = setup_test_env(&env);

    // Setup Citizen 2
    let citizen_2 = Address::generate(&env);
    client.register_citizen(&admin, &citizen_2, &String::from_str(&env, "User2"));

    client.fund_aid(&admin, &citizen_1, &1000);
    client.fund_aid(&admin, &citizen_2, &2500);

    client.claim_aid(&citizen_2);

    assert_eq!(client.get_balance(&citizen_1), 1000);
    assert_eq!(client.get_balance(&citizen_2), 0);
}
