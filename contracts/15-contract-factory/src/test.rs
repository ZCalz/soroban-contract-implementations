#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::{Address as _, MockAuth, MockAuthInvoke};
use soroban_sdk::IntoVal;

fn setup(env: &Env) -> (Address, ContractFactoryClient) {
    let admin = Address::generate(env);
    let contract_id = env.register(ContractFactory, (admin.clone(),));
    (admin, ContractFactoryClient::new(env, &contract_id))
}

/// These two tests don't need a real wasm hash -- they prove the auth gate on
/// `deploy` runs (and fails) before the bogus hash is ever touched, the same
/// trick used in 14-upgradeable-contract/src/test.rs.
#[test]
fn non_admin_cannot_deploy() {
    let env = Env::default();
    let (_admin, client) = setup(&env);
    let attacker = Address::generate(&env);
    let fake_hash = BytesN::from_array(&env, &[0u8; 32]);
    let salt = BytesN::from_array(&env, &[1u8; 32]);
    let owner = Address::generate(&env);

    client.env.mock_auths(&[MockAuth {
        address: &attacker,
        invoke: &MockAuthInvoke {
            contract: &client.address,
            fn_name: "deploy",
            args: (fake_hash.clone(), salt.clone(), owner.clone()).into_val(&env),
            sub_invokes: &[],
        },
    }]);

    todo!("assert client.try_deploy(&fake_hash, &salt, &owner) is an error")
}

#[test]
fn unknown_salt_has_no_deployment() {
    let env = Env::default();
    env.mock_all_auths();
    let (_admin, client) = setup(&env);
    let salt = BytesN::from_array(&env, &[9u8; 32]);
    todo!("assert client.get_deployment(&salt) is None")
}

/// The real end-to-end test: actually deploy 01-counter through the factory
/// and call it. This needs a REAL compiled wasm for 01-counter, so it's gated
/// behind a feature that's off by default. Run it with:
///
///   cargo build -p counter --release --target wasm32v1-none
///   cargo test -p contract-factory --features factory-integration-test
#[cfg(feature = "factory-integration-test")]
mod integration {
    use super::*;

    // Adjust this path if your workspace/target layout differs.
    mod counter_wasm {
        soroban_sdk::contractimport!(
            file = "../../target/wasm32v1-none/release/counter.wasm"
        );
    }

    #[test]
    fn deploy_and_call_a_real_counter() {
        let env = Env::default();
        env.mock_all_auths();
        let (_admin, client) = setup(&env);

        let wasm_hash = env.deployer().upload_contract_wasm(counter_wasm::WASM);
        let salt = BytesN::from_array(&env, &[7u8; 32]);
        let counter_owner = Address::generate(&env);

        todo!(
            "call client.deploy(&wasm_hash, &salt, &counter_owner); use the \
             returned Address to build a counter_wasm::Client (or your own \
             CounterClient if you add a path dependency) and call increment() \
             / get() on it, proving the factory deployed a real, independently \
             callable contract"
        )
    }
}
