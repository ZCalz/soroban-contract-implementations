#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::token::{StellarAssetClient, TokenClient};

const AMOUNT: i128 = 5_000;

struct Fixture<'a> {
    buyer: Address,
    seller: Address,
    arbiter: Address,
    token: TokenClient<'a>,
    client: EscrowArbitrationClient<'a>,
}

fn setup(env: &Env) -> Fixture {
    let token_admin = Address::generate(env);
    let sac = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token = TokenClient::new(env, &sac.address());
    let asset_client = StellarAssetClient::new(env, &sac.address());

    let buyer = Address::generate(env);
    let seller = Address::generate(env);
    let arbiter = Address::generate(env);
    asset_client.mint(&buyer, &AMOUNT);

    let contract_id = env.register(
        EscrowArbitration,
        (buyer.clone(), seller.clone(), arbiter.clone(), sac.address(), AMOUNT),
    );

    Fixture {
        buyer,
        seller,
        arbiter,
        token,
        client: EscrowArbitrationClient::new(env, &contract_id),
    }
}

#[test]
fn construction_pulls_funds_from_buyer() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    todo!(
        "assert f.token.balance(&f.buyer) == 0 and \
         f.token.balance(&f.client.address) == AMOUNT right after construction"
    )
}

#[test]
fn buyer_and_seller_agree_releases_to_seller() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    todo!(
        "buyer approves(true), seller approves(true); assert f.client.resolved() \
         == true and f.token.balance(&f.seller) == AMOUNT"
    )
}

#[test]
fn buyer_and_arbiter_agree_refunds_buyer() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    todo!(
        "buyer approves(false), arbiter approves(false); assert resolved() == \
         true and f.token.balance(&f.buyer) == AMOUNT"
    )
}

#[test]
fn split_vote_does_not_resolve() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    todo!(
        "buyer approves(true), seller approves(false); assert resolved() == \
         false and neither buyer nor seller has received funds yet -- only the \
         arbiter's vote can break the tie"
    )
}

#[test]
fn non_participant_cannot_vote() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    let stranger = Address::generate(&env);
    todo!("assert f.client.try_approve(&stranger, &true) returns Err(NotAParticipant)")
}

#[test]
fn cannot_vote_twice() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    todo!(
        "buyer approves(true) once; assert a second approve from buyer returns \
         Err(AlreadyVoted)"
    )
}

#[test]
fn cannot_vote_after_resolved() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    todo!(
        "resolve via buyer+seller both voting true, then assert arbiter's vote \
         afterward returns Err(AlreadyResolved)"
    )
}
