#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::{Address as _, MockAuth, MockAuthInvoke};
use soroban_sdk::IntoVal;

fn setup(env: &Env) -> (Address, OwnableClient) {
    let admin = Address::generate(env);
    let contract_id = env.register(Ownable, (admin.clone(),));
    (admin, OwnableClient::new(env, &contract_id))
}

#[test]
fn constructed_with_zero_value() {
    let env = Env::default();
    env.mock_all_auths();
    let (admin, client) = setup(&env);
    todo!("assert client.admin() == admin and client.get_value() == 0")
}

#[test]
fn admin_can_set_value() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, client) = setup(&env);
    todo!("call set_value(&42), assert get_value() == 42")
}

#[test]
fn non_admin_cannot_set_value() {
    let env = Env::default();
    let (admin, client) = setup(&env);
    let attacker = Address::generate(&env);

    // Mock auth as `attacker` instead of the real admin, then expect the call
    // to fail because require_admin() checks the stored ADMIN, not the caller.
    client.env.mock_auths(&[MockAuth {
        address: &attacker,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "set_value",
            args: (42i128,).into_val(&env),
            sub_invokes: &[],
        },
    }]);

    todo!("assert client.try_set_value(&42) is an error")
}

#[test]
fn transfer_ownership_updates_admin() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, client) = setup(&env);
    let new_admin = Address::generate(&env);
    todo!(
        "call transfer_ownership(&new_admin), assert client.admin() == new_admin, \
         then assert the OLD admin can no longer call set_value (stretch)"
    )
}
