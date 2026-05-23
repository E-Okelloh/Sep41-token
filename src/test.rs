#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction},
    Address, Env, String,
};

use crate::contract::TokenContractClient;
use crate::TokenContract;

#[test]
fn test_transfer() {
    let env = Env::default();

    // MOCK AUTH
    env.mock_all_auths();

    let contract_id = env.register(TokenContract, ());

    let client = TokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    let user1 = Address::generate(&env);

    let user2 = Address::generate(&env);

    // initialize + constructor mint
    client.initialize(
        &admin,
        &String::from_str(&env, "MyToken"),
        &String::from_str(&env, "MTK"),
        &1000,
    );

    // transfer
    client.transfer(&admin, &user1, &100);

    assert_eq!(client.balance(&user1), 100);

    // approve
    client.approve(&user1, &user2, &50, &1000);

    // transfer_from
    client.transfer_from(&user2, &user1, &admin, &50);

    assert_eq!(client.balance(&admin), 950);

    // burn
    client.burn(&admin, &100);

    assert_eq!(client.total_supply(), 900);
}
