#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::Address as _;

fn setup(env: &Env) -> (Address, MultiRoleAccessClient) {
    let admin = Address::generate(env);
    let contract_id = env.register(MultiRoleAccess, (admin.clone(),));
    (admin, MultiRoleAccessClient::new(env, &contract_id))
}

#[test]
fn admin_is_set_on_construction() {
    let env = Env::default();
    env.mock_all_auths();
    let (admin, client) = setup(&env);
    todo!("assert client.get_role(&admin) == Some(Role::Admin)")
}

#[test]
fn admin_can_grant_and_revoke_roles() {
    let env = Env::default();
    env.mock_all_auths();
    let (admin, client) = setup(&env);
    let alice = Address::generate(&env);
    todo!(
        "grant_role(admin, alice, Role::Moderator), assert get_role(alice) is Some(Moderator); \
         revoke_role(admin, alice), assert get_role(alice) is None"
    )
}

#[test]
fn non_admin_cannot_grant_roles() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, client) = setup(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    todo!(
        "call grant_role(alice, bob, Role::User) where alice has no role at all; \
         assert it returns Err(NotAuthorized) via try_grant_role"
    )
}

#[test]
fn moderator_action_allows_admin_and_moderator_but_not_user() {
    let env = Env::default();
    env.mock_all_auths();
    let (admin, client) = setup(&env);
    let mod_account = Address::generate(&env);
    let plain_user = Address::generate(&env);
    todo!(
        "grant Role::Moderator to mod_account and Role::User to plain_user; \
         assert moderator_action succeeds for `admin` and `mod_account`; \
         assert moderator_action returns Err(NotAuthorized) for `plain_user`"
    )
}
