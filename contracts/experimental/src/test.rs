#![cfg(test)]
extern crate std;

use super::*; 

use soroban_sdk::testutils::{Address as _}; 

fn setup(env: Env) -> (Address, Address, RaceClient<'static>) {
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let contract_id = env.register(Race, ( admin.clone(),));
    (admin, user, RaceClient::new(&env, &contract_id))
}

#[test]
fn test_initalize_vehicle() {
    let env = Env::default();
    let (admin, user, client) = setup(env);
    client.initialize_vehicle(&admin, &user);
    assert_eq!(client.has_vehicle(&user), true);
}