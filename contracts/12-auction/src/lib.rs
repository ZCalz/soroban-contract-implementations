#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, token, Address, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Seller,
    Token,
    MinBid,
    EndTime,
    HighestBid,
    HighestBidder,
    Closed,
    /// Outbid bidders' funds land here instead of being pushed back to them
    /// immediately -- see the module doc comment for why.
    Refundable(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AuctionEnded = 1,
    AuctionNotEnded = 2,
    BidTooLow = 3,
    AlreadyClosed = 4,
    NothingToWithdraw = 5,
}

/// A minimal English auction. What's being auctioned (an NFT, a registry entry,
/// anything) is intentionally out of scope -- this contract only handles the
/// *money* side (bidding, safe refunds, closing). Wiring up a real asset
/// transfer on `close()` -- e.g. against your own NFT-like contract -- is a good
/// stretch goal once the payment logic here is solid.
///
/// Refund pattern: when a new highest bid comes in, the PREVIOUS highest
/// bidder's funds are credited to `Refundable(previous_bidder)` rather than
/// transferred back to them immediately inside `bid()`. They then call
/// `withdraw_refund()` themselves. This "pull over push" pattern is the standard
/// defense against a bidder whose address can't receive funds (or, in EVM-land,
/// a malicious contract) blocking every future bid by reverting the refund.
#[contract]
pub struct Auction;

#[contractimpl]
impl Auction {
    pub fn __constructor(env: &Env, seller: Address, token: Address, min_bid: i128, end_time: u64) {
        todo!(
            "store Seller, Token, MinBid, EndTime, Closed = false; \
             HighestBid starts at 0, HighestBidder is unset"
        )
    }

    /// `bidder` must authorize. `amount` must exceed the current highest bid
    /// (and be >= MinBid if this is the first bid). Pulls `amount` of Token from
    /// `bidder` into the contract immediately.
    pub fn bid(env: &Env, bidder: Address, amount: i128) -> Result<(), Error> {
        todo!(
            "bidder.require_auth(); Err(AuctionEnded) if now >= EndTime; \
             Err(BidTooLow) if amount doesn't beat the current HighestBid (and \
             MinBid, if no bids yet); transfer `amount` of Token from `bidder` \
             to this contract; if there was a previous highest bidder, add the \
             OLD HighestBid amount to Refundable(previous_bidder); set \
             HighestBid = amount, HighestBidder = bidder"
        )
    }

    /// Anyone whose bid was later outbid may withdraw what they're owed.
    pub fn withdraw_refund(env: &Env, bidder: Address) -> Result<(), Error> {
        todo!(
            "bidder.require_auth(); read Refundable(bidder), Err(NothingToWithdraw) \
             if 0/unset; zero it out FIRST, then transfer that amount from this \
             contract to `bidder` (zeroing before transferring is the \
             checks-effects-interactions pattern -- avoids paying out twice if \
             anything about the transfer could re-enter)"
        )
    }

    /// Anyone may call this once EndTime has passed. Pays the winning bid to the
    /// seller and marks the auction closed. A no-op-but-valid call if there were
    /// no bids at all (nothing to pay out).
    pub fn close(env: &Env) -> Result<(), Error> {
        todo!(
            "Err(AuctionNotEnded) if now < EndTime; Err(AlreadyClosed) if Closed; \
             if HighestBid > 0, transfer HighestBid of Token from this contract \
             to Seller; set Closed = true"
        )
    }

    /// Read-only.
    pub fn highest_bid(env: &Env) -> i128 {
        todo!("return HighestBid, defaulting to 0")
    }

    /// Read-only.
    pub fn highest_bidder(env: &Env) -> Option<Address> {
        todo!("return HighestBidder, or None if there have been no bids")
    }
}

mod test;
