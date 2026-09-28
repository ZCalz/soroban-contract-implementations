#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, token, Address, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Buyer,
    Seller,
    Arbiter,
    Token,
    Amount,
    Resolved,
    /// true = voted to release to seller, false = voted to refund the buyer
    Vote(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotAParticipant = 1,
    AlreadyResolved = 2,
    AlreadyVoted = 3,
}

#[contract]
pub struct EscrowArbitration;

#[contractimpl]
impl EscrowArbitration {
    /// `buyer` must authorize -- the constructor pulls `amount` of `token` from
    /// `buyer` into this contract in the same call, so the escrow is funded
    /// atomically with creation.
    pub fn __constructor(
        env: &Env,
        buyer: Address,
        seller: Address,
        arbiter: Address,
        token: Address,
        amount: i128,
    ) {
        todo!(
            "buyer.require_auth(); store Buyer, Seller, Arbiter, Token, Amount, \
             Resolved = false; then use a token::TokenClient to transfer \
             `amount` from `buyer` to env.current_contract_address()"
        )
    }

    /// `caller` must be the buyer, seller, or arbiter, and must authorize.
    /// `release_to_seller` is this participant's vote: true = pay the seller,
    /// false = refund the buyer. Once any outcome has 2 of the 3 possible votes,
    /// the funds move immediately and Resolved is set.
    pub fn approve(env: &Env, caller: Address, release_to_seller: bool) -> Result<(), Error> {
        todo!(
            "caller.require_auth(); Err(AlreadyResolved) if Resolved; \
             Err(NotAParticipant) if caller isn't Buyer/Seller/Arbiter; \
             Err(AlreadyVoted) if Vote(caller) is already set; otherwise store \
             Vote(caller) = release_to_seller, then count how many of the (up \
             to 3) recorded votes match `release_to_seller`. If the count \
             reaches 2, transfer Amount of Token from this contract to Seller \
             (if release_to_seller) or Buyer (if not), and set Resolved = true."
        )
    }

    /// Read-only.
    pub fn resolved(env: &Env) -> bool {
        todo!("return Resolved, defaulting to false")
    }
}

mod test;
