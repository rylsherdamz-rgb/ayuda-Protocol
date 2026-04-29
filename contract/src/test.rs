#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_full_lifecycle() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AyudaContract);
    let client = AyudaContractClient::new(&env, &contract_id);

    // 1. Setup Identities
    // Use your specific hardcoded admin address
    let admin = Address::from_string(&String::from_str(
        &env,
        "GAJPZCOVW34KTYF764X74ZRYOJIF3H2XKCRWH4CARVRZD5M4WJ2XVWLW",
    ));
    let citizen = Address::generate(&env);
    let random_user = Address::generate(&env);

    // Mock a token for aid (e.g., USDC)
    let token_admin = Address::generate(&env);
    let token_addr = env.register_stellar_asset_contract(token_admin);

    // 2. Test Initialization
    // Fails if a random user tries to init
    let init_err = client.try_init(&random_user, &token_addr);
    assert_eq!(init_err, Err(Ok(AyudaError::NotAdmin)));

    // Succeeds with your hardcoded admin
    client.init(&admin, &token_addr);

    // 3. Test Citizen Registration
    let nfc_id = String::from_str(&env, "nfc_hash_001");
    let citizen_name = String::from_str(&env, "Richie");

    // Fails if not signed by admin
    let reg_err = client.try_register_citizen(&random_user, &citizen, &nfc_id, &citizen_name);
    assert_eq!(reg_err, Err(Ok(AyudaError::NotAdmin)));

    // Succeeds with admin
    client.register_citizen(&admin, &citizen, &nfc_id, &citizen_name);

    // 4. Test Funding Aid
    let aid_amount: i128 = 5000; // e.g., 50.00 USDC
    client.fund_aid(&admin, &citizen, &aid_amount);

    assert_eq!(client.get_balance(&citizen), aid_amount);

    // 5. Test Claiming Aid
    // First, send tokens to the contract so it can pay out
    let token_client = token::Client::new(&env, &token_addr);
    env.as_contract(&token_addr, || {
        token_client.mint(&contract_id, &10000);
    });

    // Fails if the NFC ID mapping is wrong
    let wrong_nfc = String::from_str(&env, "fake_nfc");
    let claim_err = client.try_claim_aid(&citizen, &wrong_nfc);
    assert_eq!(claim_err, Err(Ok(AyudaError::InvalidIdMapping)));

    // Succeeds with correct NFC tap
    client.claim_aid(&citizen, &nfc_id);

    // Verify balances after claim
    assert_eq!(client.get_balance(&citizen), 0);
    assert_eq!(token_client.balance(&citizen), aid_amount);
}

#[test]
fn test_duplicate_nfc_prevention() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AyudaContract);
    let client = AyudaContractClient::new(&env, &contract_id);

    let admin = Address::from_string(&String::from_str(
        &env,
        "GAJPZCOVW34KTYF764X74ZRYOJIF3H2XKCRWH4CARVRZD5M4WJ2XVWLW",
    ));
    let token_addr = Address::generate(&env);
    client.init(&admin, &token_addr);

    let nfc_id = String::from_str(&env, "shared_card_123");

    // Register first citizen
    client.register_citizen(
        &admin,
        &Address::generate(&env),
        &nfc_id,
        &String::from_str(&env, "User A"),
    );

    // Attempting to register second citizen with SAME card should fail
    let second_reg = client.try_register_citizen(
        &admin,
        &Address::generate(&env),
        &nfc_id,
        &String::from_str(&env, "User B"),
    );
    assert_eq!(second_reg, Err(Ok(AyudaError::IdAlreadyLinked)));
}

