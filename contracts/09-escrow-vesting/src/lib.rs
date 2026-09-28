#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Token,
    Beneficiary,
    TotalAmount,
    ClaimedAmount,
    StartTimestamp,
    CliffSeconds,
    DurationSeconds,
}

#[contract]
pub struct VestingEscrow;

#[contractimpl]
impl VestingEscrow {
    /// The deployer must transfer `total_amount` of `token` to this contract's
    /// address BEFORE (or immediately after, in the same submitted transaction
    /// group) construction -- the contract does not mint or pull funds itself.
    /// Nothing is claimable before `start_timestamp + cliff_seconds`; the amount
    /// unlocks linearly from there through `start_timestamp + duration_seconds`,
    /// at which point the full `total_amount` is claimable.
    pub fn __constructor(
        env: &Env,
        token: Address,
        beneficiary: Address,
        total_amount: i128,
        start_timestamp: u64,
        cliff_seconds: u64,
        duration_seconds: u64,
    ) {
        todo!(
            "store Token, Beneficiary, TotalAmount, StartTimestamp, CliffSeconds, \
             DurationSeconds; initialize ClaimedAmount to 0"
        )
    }

    /// Read-only. How much of TotalAmount has unlocked so far (not yet
    /// necessarily claimed), given the current ledger timestamp.
    pub fn vested_amount(env: &Env) -> i128 {
        todo!(
            "let now = env.ledger().timestamp(); before start+cliff -> 0; at or \
             after start+duration -> TotalAmount; in between -> \
             TotalAmount * (now - start) / duration (linear from `start`, not \
             from the end of the cliff -- the cliff only gates *when* claiming \
             starts, it doesn't reset the vesting clock)"
        )
    }

    /// Read-only. `vested_amount() - ClaimedAmount`.
    pub fn claimable_amount(env: &Env) -> i128 {
        todo!("vested_amount(env) - ClaimedAmount")
    }

    /// Only the beneficiary may claim. Transfers whatever is currently
    /// claimable and updates ClaimedAmount.
    pub fn claim(env: &Env) -> i128 {
        todo!(
            "beneficiary.require_auth(); compute claimable_amount(env); panic if \
             it's 0 (nothing to claim); transfer it from \
             env.current_contract_address() to Beneficiary via a token::TokenClient; \
             add it to ClaimedAmount; return the amount claimed"
        )
    }
}

mod test;
