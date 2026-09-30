#![cfg(test)]
extern crate std;
use soroban_sdk::testutils::{Address as _, Events as _};
use soroban_sdk::Event;
use soroban_sdk::log;
use soroban_sdk::testutils::Logs;


use super::*;


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
    // todo!(
    //     "add two items for alice, assert get_items(alice) has 2 entries \
    //      with the expected text and completed == false"
    // )
    let item: String = String::from_str(&env, "Do chores");
    client.add_item(&alice, &item);
    let item2: String = String::from_str(&env, "Do homework");
    client.add_item(&alice, &item2);

    let todo = client.get_items( &alice);
    assert_eq!(todo.len(), 2);

    for i in todo {
        assert_eq!(i.completed, false);
    }

    // let event = PushedTodoItem {
    //     id: 1,
    //     text: item,
    //     completed: false,
    // };

    // let event2 = PushedTodoItem {
    //     id: 2,
    //     text: item2,
    //     completed: false,
    // };

    // assert_eq!(
    //     env.events().all(),
    //     std::vec![event.to_xdr(&env, &client.address), event2.to_xdr(&env, &client.address)],
    // );
}

#[test]
fn complete_item_flips_flag() {
    let env = Env::default();
    env.mock_all_auths();
    let client = setup(&env);
    let alice = Address::generate(&env);

    // todo!(
    //     "add an item, complete it, assert get_items(alice)[0].completed == true"
    // )

    let item: String = String::from_str(&env, "Do chores");
    client.add_item(&alice, &item);

    client.complete_item(&alice, &1);
    let todo = client.get_items( &alice);
    assert_eq!(todo.len(), 1);

    for i in todo {
        // env.logs().print(i.completed);
        log!(&env, "completed check: ", i.completed);
        assert_eq!(i.completed, true);
    }

}

#[test]
fn items_are_isolated_per_owner() {
    let env = Env::default();
    env.mock_all_auths();
    let client = setup(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    // todo!(
    //     "add items for alice and separately for bob, assert each owner only \
    //      sees their own items"
    // )

    let item: String = String::from_str(&env, "Do chores");
    client.add_item(&alice, &item);
    let item2: String = String::from_str(&env, "Do homework");
    client.add_item(&bob, &item2);

    let alice_items = client.get_items(&alice);
    let bob_items = client.get_items(&bob);

    assert_eq!(alice_items.len(), 1);
    assert_eq!(bob_items.len(), 1);
    assert_eq!(alice_items.get(0).unwrap().text, item);
    assert_eq!(bob_items.get(0).unwrap().text, item2);

    for entry in alice_items.iter() {
        assert_eq!(entry.text, item);
        assert_ne!(entry.text, item2);
    }
    for entry in bob_items.iter() {
        assert_eq!(entry.text, item2);
        assert_ne!(entry.text, item);
    }

}

#[test]
#[should_panic]
fn completing_nonexistent_item_panics() {
    let env = Env::default();
    env.mock_all_auths();
    let client = setup(&env);
    let alice = Address::generate(&env);
    // todo!("call complete_item(alice, 999) with nothing added yet, expect a panic")
    client.complete_item(&alice, &999);
}
