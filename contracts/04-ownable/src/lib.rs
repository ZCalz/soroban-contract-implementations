#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};

#[contract]
pub struct Ownable;

const ADMIN: Symbol = symbol_short!("ADMIN");
const VALUE: Symbol = symbol_short!("VALUE");

#[contractimpl]
impl Ownable {
    /// Store `admin` and initialize VALUE to 0.
    pub fn __constructor(env: &Env, admin: Address) {
        todo!("store admin under ADMIN, initialize VALUE to 0i128")
    }

    /// Only the current admin may set the value.
    pub fn set_value(env: &Env, value: i128) {
        todo!("require_admin(env), then store `value` under VALUE")
    }

    /// Read-only.
    pub fn get_value(env: &Env) -> i128 {
        todo!("read and return VALUE")
    }

    /// Read-only.
    pub fn admin(env: &Env) -> Address {
        todo!("read and return ADMIN")
    }

    /// Only the current admin may hand ownership to `new_admin`.
    /// `new_admin` does NOT need to authorize -- ownership transfer is a
    /// one-sided action by the current admin (compare this to a pattern where
    /// the new admin must also accept, which avoids transferring to a typo'd
    /// address -- that's a good stretch goal).
    pub fn transfer_ownership(env: &Env, new_admin: Address) {
        todo!("require_admin(env), then overwrite ADMIN with new_admin")
    }

    /// Private helper: require auth from whoever is currently stored as admin.
    fn require_admin(env: &Env) {
        todo!("load ADMIN, call .require_auth() on it")
    }
}

mod test;
