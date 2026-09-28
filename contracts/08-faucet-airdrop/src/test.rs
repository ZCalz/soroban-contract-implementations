#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::{Address as _, Ledger};
use soroban_sdk::token::{StellarAssetClient, TokenClient};

const CLAIM_AMOUNT: i128 = 100;
const PERIOD_SECONDS: u64 = 86_400; // 1 day

/// Sets up a test token (a real Stellar Asset Contract, which speaks the same
/// interface your 07-fungible-token contract does) and a funded Faucet pointed
/// at it.
fn setup<'a>(env: &'a Env) -> (TokenClient<'a>, StellarAssetClient<'a>, FaucetClient<'a>) {
    let token_admin = Address::generate(env);
    let sac = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_client = TokenClient::new(env, &sac.address());
    let asset_client = StellarAssetClient::new(env, &sac.address());

    let faucet_id = env.register(
        Faucet,
        (sac.address(), CLAIM_AMOUNT, PERIOD_SECONDS),
    );
    let faucet_client = FaucetClient::new(env, &faucet_id);

    // Fund the faucet itself so it has something to hand out.
    asset_client.mint(&faucet_id, &(CLAIM_AMOUNT * 10));

    (token_client, asset_client, faucet_client)
}

#[test]
fn first_claim_succeeds() {
    let env = Env::default();
    env.mock_all_auths();
    let (token, _asset, faucet) = setup(&env);
    let alice = Address::generate(&env);
    todo!(
        "call faucet.claim(&alice), assert token.balance(&alice) == CLAIM_AMOUNT"
    )
}

#[test]
fn second_claim_within_period_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let (_token, _asset, faucet) = setup(&env);
    let alice = Address::generate(&env);
    todo!(
        "claim once, then assert faucet.try_claim(&alice) fails because not \
         enough time has passed"
    )
}

#[test]
fn claim_succeeds_again_after_period_elapses() {
    let env = Env::default();
    env.mock_all_auths();
    let (token, _asset, faucet) = setup(&env);
    let alice = Address::generate(&env);
    todo!(
        "claim once; advance time with env.ledger().set_timestamp(now + PERIOD_SECONDS); \
         claim again; assert token.balance(&alice) == CLAIM_AMOUNT * 2"
    )
}

#[test]
fn time_until_claimable_counts_down() {
    let env = Env::default();
    env.mock_all_auths();
    let (_token, _asset, faucet) = setup(&env);
    let alice = Address::generate(&env);
    todo!(
        "before any claim, time_until_claimable should be 0; after claiming, it \
         should be roughly PERIOD_SECONDS; advance time partway and check it \
         decreased accordingly"
    )
}
