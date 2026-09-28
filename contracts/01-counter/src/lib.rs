#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};

#[contract]
pub struct Counter;

const COUNT: Symbol = symbol_short!("COUNT");
const ADMIN: Symbol = symbol_short!("ADMIN");

#[contractimpl]
impl Counter {
    /// Store `admin` in instance storage and initialize the count to 0.
    pub fn __constructor(env: &Env, admin: Address) {
        todo!("store `admin` under ADMIN, initialize COUNT to 0u32")
    }

    /// Anyone may call this. Increments the counter by 1 and returns the new value.
    pub fn increment(env: &Env) -> u32 {
        todo!("read COUNT, add 1, write it back, return the new value")
    }

    /// Read-only. Returns the current count without modifying it.
    pub fn get(env: &Env) -> u32 {
        todo!("read and return COUNT")
    }

    /// Only the admin may reset the counter back to 0.
    pub fn reset(env: &Env) {
        todo!("require_auth() on the stored admin, then set COUNT back to 0")
    }
}

mod test;
