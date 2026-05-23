use soroban_sdk::{Address, Env};

use crate::types::DataKey;

pub fn read_balance(env: &Env, user: Address) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::Balance(user))
        .unwrap_or(0)
}

pub fn write_balance(env: &Env, user: Address, amount: i128) {
    env.storage()
        .persistent()
        .set(&DataKey::Balance(user), &amount);
}

pub fn read_allowance(env: &Env, from: Address, spender: Address) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::Allowance(from, spender))
        .unwrap_or(0)
}

pub fn write_allowance(env: &Env, from: Address, spender: Address, amount: i128) {
    env.storage()
        .persistent()
        .set(&DataKey::Allowance(from, spender), &amount);
}

pub fn read_total_supply(env: &Env) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::TotalSupply)
        .unwrap_or(0)
}

pub fn write_total_supply(env: &Env, amount: i128) {
    env.storage()
        .persistent()
        .set(&DataKey::TotalSupply, &amount);
}
