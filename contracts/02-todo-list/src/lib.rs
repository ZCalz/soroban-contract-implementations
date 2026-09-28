#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

#[derive(Clone)]
#[contracttype]
pub struct TodoItem {
    pub id: u32,
    pub text: String,
    pub completed: bool,
}

/// Storage key: each owner gets their own Vec<TodoItem>, keyed on their Address.
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Items(Address),
    NextId(Address),
}

#[contract]
pub struct TodoList;

#[contractimpl]
impl TodoList {
    /// Add a new item for `owner`. Returns the new item's id.
    /// `owner` must authorize the call.
    pub fn add_item(env: &Env, owner: Address, text: String) -> u32 {
        todo!(
            "owner.require_auth(); allocate the next id for this owner \
             (DataKey::NextId(owner)), push a new TodoItem into \
             DataKey::Items(owner), return the id"
        )
    }

    /// Mark an item completed. `owner` must authorize the call and must own the item.
    pub fn complete_item(env: &Env, owner: Address, id: u32) {
        todo!(
            "owner.require_auth(); load the owner's Vec<TodoItem>, find the item \
             with this id, set completed = true, write the Vec back. Panic if not found."
        )
    }

    /// Read-only. Returns all items for `owner`, in insertion order.
    pub fn get_items(env: &Env, owner: Address) -> Vec<TodoItem> {
        todo!("return DataKey::Items(owner), defaulting to an empty Vec if unset")
    }
}

mod test;
