#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, String};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    TotalSupply,
    Balance(Address),
    /// (owner, spender) -> (amount, expiration_ledger)
    Allowance(Address, Address),
}

#[derive(Clone)]
#[contracttype]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotAuthorized = 1,
    InsufficientBalance = 2,
    InsufficientAllowance = 3,
    AllowanceExpired = 4,
    NegativeAmount = 5,
}

#[contract]
pub struct FungibleToken;

#[contractimpl]
impl FungibleToken {
    pub fn __constructor(env: &Env, admin: Address, decimals: u32, name: String, symbol: String) {
        todo!(
            "store admin, decimals, name, symbol (pick your own storage keys for \
             the metadata), and TotalSupply = 0"
        )
    }

    /// Admin-only. Mints `amount` new tokens to `to`. Increases total supply.
    pub fn mint(env: &Env, to: Address, amount: i128) -> Result<(), Error> {
        todo!(
            "require admin auth; reject amount < 0 with Err(NegativeAmount); \
             add to Balance(to); add to TotalSupply"
        )
    }

    /// Read-only.
    pub fn balance(env: &Env, id: Address) -> i128 {
        todo!("return Balance(id), defaulting to 0")
    }

    /// Read-only.
    pub fn total_supply(env: &Env) -> i128 {
        todo!("return TotalSupply, defaulting to 0")
    }

    /// `from` must authorize. Moves `amount` from `from` to `to`.
    pub fn transfer(env: &Env, from: Address, to: Address, amount: i128) -> Result<(), Error> {
        todo!(
            "from.require_auth(); reject negative amount; check from's balance is \
             sufficient (Err(InsufficientBalance) if not); debit from, credit to"
        )
    }

    /// `from` must authorize. Lets `spender` transfer up to `amount` on `from`'s
    /// behalf, until `expiration_ledger`.
    pub fn approve(
        env: &Env,
        from: Address,
        spender: Address,
        amount: i128,
        expiration_ledger: u32,
    ) -> Result<(), Error> {
        todo!(
            "from.require_auth(); store AllowanceValue {{ amount, expiration_ledger }} \
             under Allowance(from, spender)"
        )
    }

    /// Read-only. Returns 0 if unset OR if the allowance has expired.
    pub fn allowance(env: &Env, from: Address, spender: Address) -> i128 {
        todo!(
            "read Allowance(from, spender); if unset or env.ledger().sequence() > \
             expiration_ledger, return 0, else return amount"
        )
    }

    /// `spender` must authorize. Moves `amount` from `from` to `to`, drawing down
    /// the allowance `from` granted to `spender`.
    pub fn transfer_from(
        env: &Env,
        spender: Address,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), Error> {
        todo!(
            "spender.require_auth(); check allowance(from, spender) >= amount \
             (Err(InsufficientAllowance) if not) and balance(from) >= amount \
             (Err(InsufficientBalance) if not); debit the allowance, debit from, credit to"
        )
    }
}

mod test;
