#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::token::{StellarAssetClient, TokenClient};

struct Fixture<'a> {
    seller: Address,
    buyer: Address,
    token: TokenClient<'a>,
    client: MarketplaceClient<'a>,
}

fn setup(env: &Env) -> Fixture {
    let token_admin = Address::generate(env);
    let sac = env.register_stellar_asset_contract_v2(token_admin);
    let token = TokenClient::new(env, &sac.address());
    let asset = StellarAssetClient::new(env, &sac.address());

    let seller = Address::generate(env);
    let buyer = Address::generate(env);
    asset.mint(&buyer, &10_000);

    let contract_id = env.register(Marketplace, ());

    Fixture {
        seller,
        buyer,
        token,
        client: MarketplaceClient::new(env, &contract_id),
    }
}

// --- These already pass with the marketplace logic as provided -- no
// changes needed. They're here so you can see the mechanics work before you
// start layering events on top. ---

#[test]
fn list_and_read_back() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    let id = f.client.list_item(&f.seller, &f.token.address, &500);
    let listing = f.client.get_listing(&id).unwrap();
    assert_eq!(listing.seller, f.seller);
    assert_eq!(listing.price, 500);
    assert!(listing.active);
}

#[test]
fn buy_transfers_funds_and_deactivates() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    let id = f.client.list_item(&f.seller, &f.token.address, &500);
    f.client.buy_item(&f.buyer, &id);
    assert_eq!(f.token.balance(&f.seller), 500);
    assert_eq!(f.token.balance(&f.buyer), 9_500);
    assert!(!f.client.get_listing(&id).unwrap().active);
}

#[test]
fn cannot_buy_cancelled_listing() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    let id = f.client.list_item(&f.seller, &f.token.address, &500);
    f.client.cancel_listing(&f.seller, &id);
    assert!(f.client.try_buy_item(&f.buyer, &id).is_err());
}

// --- These are the actual exercise: once you've added `env.events().publish`
// calls in src/lib.rs, fill these in to verify they fire with the right data. ---

#[test]
fn listing_emits_an_event() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    todo!(
        "call list_item(...), then inspect env.events().all() (needs \
         `soroban_sdk::testutils::Events`) and assert an event was published \
         for this contract with the id/price you expect. Print \
         env.events().all() with `std::println!(\"{{:?}}\", ...)` once if \
         you're not sure what the topic/data shape looks like."
    )
}

#[test]
fn sale_emits_an_event_with_buyer_and_seller() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    todo!(
        "list an item, buy it, assert a 'sold' event was published containing \
         both f.buyer and f.seller"
    )
}

#[test]
fn cancellation_emits_an_event() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    todo!("list an item, cancel it, assert a 'cancelled' event was published")
}
