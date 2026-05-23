use soroban_sdk::{contract, contractimpl, token::Interface, Address, Env, String};

use crate::storage::*;
use crate::types::DataKey;

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    pub fn initialize(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
        initial_supply: i128,
    ) {
        admin.require_auth();

        env.storage().persistent().set(&DataKey::Admin, &admin);
        env.storage().persistent().set(&DataKey::Name, &name);
        env.storage().persistent().set(&DataKey::Symbol, &symbol);

        // Mint during deployment
        write_balance(&env, admin.clone(), initial_supply);
        write_total_supply(&env, initial_supply);
    }

    // admin mint
    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin: Address = env.storage().persistent().get(&DataKey::Admin).unwrap();

        admin.require_auth();

        let balance = read_balance(&env, to.clone());
        write_balance(&env, to, balance + amount);

        let supply = read_total_supply(&env);
        write_total_supply(&env, supply + amount);
    }

    pub fn total_supply(env: Env) -> i128 {
        read_total_supply(&env)
    }
}

#[contractimpl]
impl Interface for TokenContract {
    fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        read_allowance(&env, from, spender)
    }

    fn approve(env: Env, from: Address, spender: Address, amount: i128, _expiration_ledger: u32) {
        from.require_auth();
        write_allowance(&env, from, spender, amount);
    }

    fn balance(env: Env, id: Address) -> i128 {
        read_balance(&env, id)
    }

    fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        let from_balance = read_balance(&env, from.clone());

        if from_balance < amount {
            panic!("insufficient balance");
        }

        let to_balance = read_balance(&env, to.clone());

        write_balance(&env, from, from_balance - amount);

        write_balance(&env, to, to_balance + amount);
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        let allowance = read_allowance(&env, from.clone(), spender.clone());

        if allowance < amount {
            panic!("allowance too low");
        }

        let from_balance = read_balance(&env, from.clone());

        if from_balance < amount {
            panic!("insufficient balance");
        }

        let to_balance = read_balance(&env, to.clone());

        write_balance(&env, from.clone(), from_balance - amount);

        write_balance(&env, to, to_balance + amount);

        write_allowance(&env, from, spender, allowance - amount);
    }

    fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();

        let balance = read_balance(&env, from.clone());

        if balance < amount {
            panic!("insufficient balance");
        }

        write_balance(&env, from, balance - amount);

        let supply = read_total_supply(&env);

        write_total_supply(&env, supply - amount);
    }

    fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        let allowance = read_allowance(&env, from.clone(), spender.clone());

        if allowance < amount {
            panic!("allowance too low");
        }

        let balance = read_balance(&env, from.clone());

        if balance < amount {
            panic!("insufficient balance");
        }

        write_balance(&env, from.clone(), balance - amount);

        write_allowance(&env, from, spender, allowance - amount);

        let supply = read_total_supply(&env);

        write_total_supply(&env, supply - amount);
    }

    fn decimals(_env: Env) -> u32 {
        18
    }

    fn name(env: Env) -> String {
        env.storage().persistent().get(&DataKey::Name).unwrap()
    }

    fn symbol(env: Env) -> String {
        env.storage().persistent().get(&DataKey::Symbol).unwrap()
    }
}
