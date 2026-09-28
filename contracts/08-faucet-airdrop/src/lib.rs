#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Token,
    AmountPerClaim,
    PeriodSeconds,
    LastClaim(Address),
}

#[contract]
pub struct Faucet;

#[contractimpl]
impl Faucet {
    /// `token` is the address of any deployed token contract that implements the
    /// standard token interface (your 07-fungible-token, or a test SAC -- see
    /// src/test.rs). The faucet must already hold a balance of that token; fund
    /// it by transferring to `env.current_contract_address()` after deploying.
    pub fn __constructor(env: &Env, token: Address, amount_per_claim: i128, period_seconds: u64) {
        todo!("store Token, AmountPerClaim, PeriodSeconds")
    }

    /// Anyone may call this for themselves -- `user` must authorize the call.
    /// Panics if `user` already claimed within the last `period_seconds`.
    pub fn claim(env: &Env, user: Address) {
        todo!(
            "user.require_auth(); read LastClaim(user) (default far in the past, \
             e.g. 0); panic if env.ledger().timestamp() - last_claim < period_seconds; \
             build a token::TokenClient for the stored Token address and call \
             .transfer(&env.current_contract_address(), &user, &amount_per_claim); \
             write LastClaim(user) = env.ledger().timestamp()"
        )
    }

    /// Read-only. Seconds remaining before `user` can claim again (0 if claimable now).
    pub fn time_until_claimable(env: &Env, user: Address) -> u64 {
        todo!(
            "compute elapsed = now - LastClaim(user) (0 if never claimed); \
             return period_seconds.saturating_sub(elapsed)"
        )
    }
}

mod test;
