#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::{Address as _, MockAuth, MockAuthInvoke};
use soroban_sdk::IntoVal;

fn setup(env: &Env) -> (Address, UpgradeableContractClient) {
    let admin = Address::generate(env);
    let contract_id = env.register(UpgradeableContract, (admin.clone(),));
    (admin, UpgradeableContractClient::new(env, &contract_id))
}

#[test]
fn starts_at_version_one() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, client) = setup(&env);
    todo!("assert client.version() == 1")
}

#[test]
fn admin_can_migrate_version() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, client) = setup(&env);
    todo!("call client.migrate(&2), assert client.version() == 2")
}

#[test]
fn non_admin_cannot_migrate() {
    let env = Env::default();
    let (_admin, client) = setup(&env);
    let attacker = Address::generate(&env);
    client.env.mock_auths(&[MockAuth {
        address: &attacker,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "migrate",
            args: (2u32,).into_val(&env),
            sub_invokes: &[],
        },
    }]);
    todo!("assert client.try_migrate(&2) is an error")
}

#[test]
fn non_admin_cannot_upgrade() {
    let env = Env::default();
    let (_admin, client) = setup(&env);
    let attacker = Address::generate(&env);
    // A deliberately bogus hash -- this test only proves the AUTH check runs
    // (and fails) before the contract ever tries to look up this hash on the
    // network, so the hash's validity doesn't matter here.
    let fake_hash = BytesN::from_array(&env, &[0u8; 32]);

    client.env.mock_auths(&[MockAuth {
        address: &attacker,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "upgrade",
            args: (fake_hash.clone(),).into_val(&env),
            sub_invokes: &[],
        },
    }]);
    todo!(
        "assert client.try_upgrade(&fake_hash) is an error, and that it fails \
         because of the auth check, not because the hash is bogus -- if your \
         implementation checks the hash before require_auth, this test won't \
         tell the difference, but a real upload_contract_wasm call further \
         down the line would"
    )
}
