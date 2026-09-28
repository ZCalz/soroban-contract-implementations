#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, token, Address, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    TokenA,
    TokenB,
    TotalShares,
    Shares(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    ZeroAmount = 1,
    InsufficientShares = 2,
    SlippageExceeded = 3,
    InvalidToken = 4,
}

/// A single-pair constant-product AMM, Uniswap-v1 style: reserves live as this
/// contract's own token balances (query them with a `token::TokenClient`,
/// there's no separate ReserveA/ReserveB to keep in sync -- one less thing that
/// can drift out of truth). This is the hardest math in the whole workspace:
/// no floats are available in a Soroban contract, so every formula below is
/// integer arithmetic, and get the order of multiply-before-divide right or
/// you'll lose precision (or overflow i128).
#[contract]
pub struct AmmSwapPool;

#[contractimpl]
impl AmmSwapPool {
    pub fn __constructor(env: &Env, token_a: Address, token_b: Address) {
        todo!("store TokenA, TokenB; initialize TotalShares to 0")
    }

    /// `provider` must authorize. Deposits `amount_a` of TokenA and `amount_b`
    /// of TokenB. On the FIRST deposit, shares minted = isqrt(amount_a * amount_b).
    /// On subsequent deposits, shares minted must be proportional to the
    /// smaller of the two ratios against current reserves, so a lopsided
    /// deposit doesn't let someone mint disproportionate shares. Returns shares
    /// minted.
    pub fn add_liquidity(env: &Env, provider: Address, amount_a: i128, amount_b: i128) -> Result<i128, Error> {
        todo!(
            "provider.require_auth(); reject zero amounts; read current \
             reserve_a/reserve_b via token::TokenClient::balance on this \
             contract's own address (BEFORE pulling the new deposit in); if \
             TotalShares == 0, shares = isqrt(amount_a * amount_b) (implement \
             isqrt as a private helper -- Newton's method on i128 works fine); \
             else shares = min(amount_a * TotalShares / reserve_a, \
             amount_b * TotalShares / reserve_b); THEN transfer amount_a of \
             TokenA and amount_b of TokenB from provider into this contract; \
             credit Shares(provider) += shares and TotalShares += shares; \
             return shares"
        )
    }

    /// `provider` must authorize and must hold at least `shares`. Burns them
    /// and returns a proportional slice of both reserves.
    pub fn remove_liquidity(env: &Env, provider: Address, shares: i128) -> Result<(i128, i128), Error> {
        todo!(
            "provider.require_auth(); Err(InsufficientShares) if \
             Shares(provider) < shares; amount_a = shares * reserve_a / \
             TotalShares, amount_b = shares * reserve_b / TotalShares (read \
             reserves BEFORE paying out); transfer both out to provider; \
             debit Shares(provider) and TotalShares by `shares`; return \
             (amount_a, amount_b)"
        )
    }

    /// `trader` must authorize. `token_in` must be either TokenA or TokenB.
    /// Applies a 0.3% fee (matching the classic Uniswap v1/v2 constant). Reverts
    /// if the resulting output would be less than `min_amount_out`.
    pub fn swap(
        env: &Env,
        trader: Address,
        token_in: Address,
        amount_in: i128,
        min_amount_out: i128,
    ) -> Result<i128, Error> {
        todo!(
            "trader.require_auth(); reject zero amount_in; figure out which \
             side is `token_in` vs. `token_out` (Err(InvalidToken) if it's \
             neither TokenA nor TokenB); read reserve_in/reserve_out; \
             amount_in_with_fee = amount_in * 997 / 1000; amount_out = \
             reserve_out * amount_in_with_fee / (reserve_in + amount_in_with_fee); \
             Err(SlippageExceeded) if amount_out < min_amount_out; transfer \
             amount_in from trader to this contract, then amount_out from this \
             contract to trader; return amount_out"
        )
    }

    /// Read-only.
    pub fn reserves(env: &Env) -> (i128, i128) {
        todo!("return (TokenA's balance of this contract, TokenB's balance of this contract)")
    }

    /// Read-only.
    pub fn shares_of(env: &Env, provider: Address) -> i128 {
        todo!("return Shares(provider), defaulting to 0")
    }

    /// Integer square root via Newton's method. Used only for the first
    /// deposit's share calculation.
    fn isqrt(value: i128) -> i128 {
        todo!(
            "handle value <= 1 trivially; otherwise iterate x_{{n+1}} = \
             (x_n + value / x_n) / 2 starting from a reasonable guess until it \
             stops decreasing"
        )
    }
}

mod test;
