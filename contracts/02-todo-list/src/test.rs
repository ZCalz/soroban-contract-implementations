#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::Address as _;

fn setup(env: &Env) -> TodoListClient {
    let contract_id = env.register(TodoList, ());
    TodoListClient::new(env, &contract_id)
}

#[test]
fn add_and_list_items() {
    let env = Env::default();
    env.mock_all_auths();
    let client = setup(&env);
    let alice = Address::generate(&env);
    todo!(
        "add two items for alice, assert get_items(alice) has 2 entries \
         with the expected text and completed == false"
    )
}

#[test]
fn complete_item_flips_flag() {
    let env = Env::default();
    env.mock_all_auths();
    let client = setup(&env);
    let alice = Address::generate(&env);
    todo!(
        "add an item, complete it, assert get_items(alice)[0].completed == true"
    )
}

#[test]
fn items_are_isolated_per_owner() {
    let env = Env::default();
    env.mock_all_auths();
    let client = setup(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    todo!(
        "add items for alice and separately for bob, assert each owner only \
         sees their own items"
    )
}

#[test]
#[should_panic]
fn completing_nonexistent_item_panics() {
    let env = Env::default();
    env.mock_all_auths();
    let client = setup(&env);
    let alice = Address::generate(&env);
    todo!("call complete_item(alice, 999) with nothing added yet, expect a panic")
}
