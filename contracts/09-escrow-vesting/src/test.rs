#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::{Address as _, Ledger};
use soroban_sdk::token::{StellarAssetClient, TokenClient};

const TOTAL: i128 = 1_000_000;
const CLIFF: u64 = 30 * 86_400; // 30 days
const DURATION: u64 = 365 * 86_400; // 1 year

fn setup<'a>(env: &'a Env, start: u64) -> (Address, TokenClient<'a>, VestingEscrowClient<'a>) {
    let token_admin = Address::generate(env);
    let sac = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_client = TokenClient::new(env, &sac.address());
    let asset_client = StellarAssetClient::new(env, &sac.address());

    let beneficiary = Address::generate(env);
    let contract_id = env.register(
        VestingEscrow,
        (sac.address(), beneficiary.clone(), TOTAL, start, CLIFF, DURATION),
    );

    // Fund the escrow with the full vested amount up front.
    asset_client.mint(&contract_id, &TOTAL);

    (beneficiary, token_client, VestingEscrowClient::new(env, &contract_id))
}

#[test]
fn nothing_vested_before_cliff() {
    let env = Env::default();
    env.mock_all_auths();
    let start = env.ledger().timestamp();
    let (_beneficiary, _token, client) = setup(&env, start);
    todo!(
        "assert client.vested_amount() == 0 right after construction, and still \
         0 just before the cliff (advance time to start + CLIFF - 1)"
    )
}

#[test]
fn linear_vesting_between_cliff_and_end() {
    let env = Env::default();
    env.mock_all_auths();
    let start = env.ledger().timestamp();
    let (_beneficiary, _token, client) = setup(&env, start);
    todo!(
        "advance time to start + DURATION / 2, assert vested_amount() is \
         approximately TOTAL / 2 (integer division, so allow some slack)"
    )
}

#[test]
fn fully_vested_after_duration() {
    let env = Env::default();
    env.mock_all_auths();
    let start = env.ledger().timestamp();
    let (_beneficiary, _token, client) = setup(&env, start);
    todo!(
        "advance time to start + DURATION (or beyond), assert vested_amount() == TOTAL"
    )
}

#[test]
fn beneficiary_can_claim_unlocked_amount() {
    let env = Env::default();
    env.mock_all_auths();
    let start = env.ledger().timestamp();
    let (beneficiary, token, client) = setup(&env, start);
    todo!(
        "advance past the cliff, call client.claim(), assert token.balance(&beneficiary) \
         matches what was returned, and a second immediate claim() returns 0 \
         or panics (nothing new vested yet)"
    )
}

#[test]
fn non_beneficiary_cannot_claim() {
    let env = Env::default();
    let start = env.ledger().timestamp();
    let (_beneficiary, _token, client) = setup(&env, start);
    todo!(
        "without mock_all_auths, use mock_auths with a random address as caller \
         (see 04-ownable/src/test.rs) and assert try_claim() fails"
    )
}
