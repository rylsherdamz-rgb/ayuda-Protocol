#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, token, Address, Env, String,
};

#[contracttype]
#[derive(Clone, Debug)]
pub struct CitizenData {
    pub name: String,
    pub aid_balance: i128,
}

#[contracttype]
pub enum DataKey {
    Admin,
    TokenAddr,
    Citizen(Address),
}

#[contract]
pub struct AyudaContract;

#[contractimpl]
impl AyudaContract {
    pub fn init(env: Env, admin: Address, token_addr: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already init");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::TokenAddr, &token_addr);
    }

    pub fn register_citizen(env: Env, admin: Address, citizen_addr: Address, name: String) {
        admin.require_auth();
        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        assert!(admin == stored_admin, "Not admin");

        let data = CitizenData {
            name,
            aid_balance: 0,
        };
        env.storage()
            .persistent()
            .set(&DataKey::Citizen(citizen_addr.clone()), &data);
    }

    pub fn fund_aid(env: Env, admin: Address, citizen_addr: Address, amount: i128) {
        admin.require_auth();
        let mut data: CitizenData = env
            .storage()
            .persistent()
            .get(&DataKey::Citizen(citizen_addr.clone()))
            .expect("Citizen not registered");

        data.aid_balance += amount;
        env.storage()
            .persistent()
            .set(&DataKey::Citizen(citizen_addr), &data);
    }

    pub fn claim_aid(env: Env, citizen_addr: Address) {
        let mut data: CitizenData = env
            .storage()
            .persistent()
            .get(&DataKey::Citizen(citizen_addr.clone()))
            .expect("No record");

        let amount = data.aid_balance;
        assert!(amount > 0, "No aid available");

        let token_addr: Address = env.storage().instance().get(&DataKey::TokenAddr).unwrap();
        let client = token::Client::new(&env, &token_addr);
        client.transfer(&env.current_contract_address(), &citizen_addr, &amount);

        data.aid_balance = 0;
        env.storage()
            .persistent()
            .set(&DataKey::Citizen(citizen_addr.clone()), &data);

        // Using the preferred tuple format for events
        env.events()
            .publish((symbol_short!("paid"), citizen_addr), amount);
    }

    pub fn get_balance(env: Env, citizen_addr: Address) -> i128 {
        let data: CitizenData = env
            .storage()
            .persistent()
            .get(&DataKey::Citizen(citizen_addr))
            .unwrap_or(CitizenData {
                name: String::from_str(&env, "Unknown"),
                aid_balance: 0,
            });
        data.aid_balance
    }
}

#[cfg(test)]
mod test;
