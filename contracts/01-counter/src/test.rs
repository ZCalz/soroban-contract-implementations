#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::Address as _;

fn setup(env: &Env) -> (Address, CounterClient) {
    let admin = Address::generate(env);
    let contract_id = env.register(Counter, (admin.clone(),));
    (admin, CounterClient::new(env, &contract_id))
}

#[test]
fn starts_at_zero() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, client) = setup(&env);
    assert_eq!(client.get(),0);
}

#[test]
fn increment_increases_count() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, client) = setup(&env);
    // todo!("call client.increment() a few times, assert client.get() reflects it")
    client.increment();
    client.increment();
    client.increment();
    assert_eq!(client.get(), 3);
}

#[test]
fn only_admin_can_reset() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, client) = setup(&env);
    // todo!(
    //     "increment a few times, call reset(), assert get() == 0. \
    //      Stretch: use mock_auths (see the guess-the-number example in \
    //      ../../guessing-game-tutorial) to prove a non-admin caller is rejected \
    //      instead of relying on mock_all_auths."
    // )
    client.increment();
    client.increment();
    assert_eq!(client.get(), 2);
    client.reset();
 
    assert_eq!(client.get(), 0);
    client.increment();
    assert_eq!(client.get(), 1);
}
