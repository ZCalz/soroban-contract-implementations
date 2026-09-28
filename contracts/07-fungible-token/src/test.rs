#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::{Address as _, Ledger};
use soroban_sdk::String;

fn setup(env: &Env) -> (Address, FungibleTokenClient) {
    let admin = Address::generate(env);
    let contract_id = env.register(
        FungibleToken,
        (
            admin.clone(),
            7u32,
            String::from_str(env, "Practice Token"),
            String::from_str(env, "PRAC"),
        ),
    );
    (admin, FungibleTokenClient::new(env, &contract_id))
}

#[test]
fn mint_increases_balance_and_supply() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, client) = setup(&env);
    let alice = Address::generate(&env);
    todo!(
        "mint 1000 to alice, assert client.balance(&alice) == 1000 and \
         client.total_supply() == 1000"
    )
}

#[test]
fn transfer_moves_balance() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, client) = setup(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    todo!(
        "mint 1000 to alice, transfer 400 from alice to bob, assert balances \
         are 600 / 400"
    )
}

#[test]
fn transfer_fails_on_insufficient_balance() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, client) = setup(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    todo!(
        "with alice at 0 balance, try_transfer(alice, bob, 1) should return \
         Err(InsufficientBalance)"
    )
}

#[test]
fn approve_and_transfer_from() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, client) = setup(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let spender = Address::generate(&env);
    let expiration = env.ledger().sequence() + 1000;
    todo!(
        "mint 1000 to alice; alice approves `spender` for 300 until `expiration`; \
         assert client.allowance(&alice, &spender) == 300; spender calls \
         transfer_from(spender, alice, bob, 300); assert bob's balance is 300, \
         alice's is 700, and the allowance is now 0"
    )
}

#[test]
fn transfer_from_fails_over_allowance() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, client) = setup(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let spender = Address::generate(&env);
    let expiration = env.ledger().sequence() + 1000;
    todo!(
        "mint 1000 to alice; approve spender for only 100; assert \
         try_transfer_from(spender, alice, bob, 200) returns Err(InsufficientAllowance)"
    )
}

#[test]
fn allowance_expires() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, client) = setup(&env);
    let alice = Address::generate(&env);
    let spender = Address::generate(&env);
    let expiration = env.ledger().sequence() + 5;
    todo!(
        "approve spender for 100 until `expiration`; advance the ledger past \
         `expiration` with env.ledger().set_sequence_number(...); assert \
         client.allowance(&alice, &spender) == 0"
    )
}

#[test]
fn only_admin_can_mint() {
    let env = Env::default();
    let (_admin, client) = setup(&env);
    let alice = Address::generate(&env);
    todo!(
        "without mock_all_auths, use mock_auths with alice as the caller (see \
         04-ownable/src/test.rs for the pattern) and assert try_mint fails \
         because the constructor's admin never authorized it"
    )
}
