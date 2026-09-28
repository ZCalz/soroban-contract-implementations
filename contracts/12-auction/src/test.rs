#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::{Address as _, Ledger};
use soroban_sdk::token::{StellarAssetClient, TokenClient};

const MIN_BID: i128 = 100;
const DURATION: u64 = 3600;

struct Fixture<'a> {
    seller: Address,
    token: TokenClient<'a>,
    asset: StellarAssetClient<'a>,
    client: AuctionClient<'a>,
    end_time: u64,
}

fn setup(env: &Env) -> Fixture {
    let token_admin = Address::generate(env);
    let sac = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token = TokenClient::new(env, &sac.address());
    let asset = StellarAssetClient::new(env, &sac.address());

    let seller = Address::generate(env);
    let end_time = env.ledger().timestamp() + DURATION;
    let contract_id = env.register(
        Auction,
        (seller.clone(), sac.address(), MIN_BID, end_time),
    );

    Fixture {
        seller,
        token,
        asset,
        client: AuctionClient::new(env, &contract_id),
        end_time,
    }
}

fn funded_bidder(env: &Env, f: &Fixture, amount: i128) -> Address {
    let bidder = Address::generate(env);
    f.asset.mint(&bidder, &amount);
    bidder
}

#[test]
fn first_bid_must_meet_min_bid() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    let alice = funded_bidder(&env, &f, 1000);
    todo!(
        "assert f.client.try_bid(&alice, &(MIN_BID - 1)) returns Err(BidTooLow); \
         assert bidding exactly MIN_BID succeeds and highest_bid() == MIN_BID"
    )
}

#[test]
fn higher_bid_replaces_and_refunds_previous_bidder() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    let alice = funded_bidder(&env, &f, 1000);
    let bob = funded_bidder(&env, &f, 1000);
    todo!(
        "alice bids 200; bob bids 300; assert highest_bidder() == bob and \
         highest_bid() == 300; assert alice's token balance dropped by 200 (now \
         held by the contract, not yet refunded); alice calls withdraw_refund(), \
         assert her balance is back to 1000"
    )
}

#[test]
fn cannot_withdraw_refund_twice() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    let alice = funded_bidder(&env, &f, 1000);
    let bob = funded_bidder(&env, &f, 1000);
    todo!(
        "alice bids 200, bob outbids with 300, alice withdraws once \
         successfully, assert a second withdraw_refund(alice) returns \
         Err(NothingToWithdraw)"
    )
}

#[test]
fn cannot_bid_after_end_time() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    let alice = funded_bidder(&env, &f, 1000);
    todo!(
        "advance time to f.end_time, assert try_bid(&alice, &MIN_BID) returns \
         Err(AuctionEnded)"
    )
}

#[test]
fn close_pays_seller_and_only_once() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    let alice = funded_bidder(&env, &f, 1000);
    todo!(
        "bid 500 as alice; assert try_close() returns Err(AuctionNotEnded) \
         while still open; advance time past end_time; close() should succeed \
         and f.token.balance(&f.seller) == 500; a second close() should return \
         Err(AlreadyClosed)"
    )
}

#[test]
fn close_with_no_bids_is_a_harmless_noop() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    todo!(
        "advance time past end_time with no bids placed; close() should \
         succeed without transferring anything (seller balance stays 0)"
    )
}
