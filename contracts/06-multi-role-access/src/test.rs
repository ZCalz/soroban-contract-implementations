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
    // todo!("assert client.get_role(&admin) == Some(Role::Admin)")
    assert_eq!(client.get_role(&admin),Some(Role::Admin));
}

#[test]
fn admin_can_grant_and_revoke_roles() {
    let env = Env::default();
    env.mock_all_auths();
    let (admin, client) = setup(&env);
    let alice = Address::generate(&env);
    // todo!(
    //     "grant_role(admin, alice, Role::Moderator), assert get_role(alice) is Some(Moderator); \
    //      revoke_role(admin, alice), assert get_role(alice) is None"
    // )
    client.grant_role(&admin, &alice, &Role::Moderator);
    assert_eq!(client.get_role(&alice), Some(Role::Moderator));
    client.revoke_role(&admin, &alice); 
    assert_eq!(client.get_role(&alice), None);
}

#[test]
fn non_admin_cannot_grant_roles() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, client) = setup(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    // todo!(
    //     "call grant_role(alice, bob, Role::User) where alice has no role at all; \
    //      assert it returns Err(NotAuthorized) via try_grant_role"
    // )
    assert_eq!(client.try_grant_role(&alice, &bob, &Role::Moderator),Err(Ok(Error::NotAuthorized)));
}

#[test]
fn moderator_action_allows_admin_and_moderator_but_not_user() {
    let env = Env::default();
    env.mock_all_auths();
    let (admin, client) = setup(&env);
    let mod_account = Address::generate(&env);
    let plain_user = Address::generate(&env);
    // todo!(
    //     "grant Role::Moderator to mod_account and Role::User to plain_user; \
    //      assert moderator_action succeeds for `admin` and `mod_account`; \
    //      assert moderator_action returns Err(NotAuthorized) for `plain_user`"
    // )
    client.grant_role(&admin, &mod_account, &Role::Moderator);
    assert_eq!(client.try_moderator_action(&mod_account), Ok(Ok(())));
    assert_eq!(client.try_moderator_action(&plain_user), Err(Ok(Error::NotAuthorized)));
}
