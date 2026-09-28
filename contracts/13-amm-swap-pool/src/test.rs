#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::token::{StellarAssetClient, TokenClient};

struct Fixture<'a> {
    token_a: TokenClient<'a>,
    token_b: TokenClient<'a>,
    asset_a: StellarAssetClient<'a>,
    asset_b: StellarAssetClient<'a>,
    client: AmmSwapPoolClient<'a>,
}

fn setup(env: &Env) -> Fixture {
    let admin_a = Address::generate(env);
    let admin_b = Address::generate(env);
    let sac_a = env.register_stellar_asset_contract_v2(admin_a);
    let sac_b = env.register_stellar_asset_contract_v2(admin_b);

    let contract_id = env.register(AmmSwapPool, (sac_a.address(), sac_b.address()));

    Fixture {
        token_a: TokenClient::new(env, &sac_a.address()),
        token_b: TokenClient::new(env, &sac_b.address()),
        asset_a: StellarAssetClient::new(env, &sac_a.address()),
        asset_b: StellarAssetClient::new(env, &sac_b.address()),
        client: AmmSwapPoolClient::new(env, &contract_id),
    }
}

#[test]
fn first_deposit_mints_isqrt_shares() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    let alice = Address::generate(&env);
    f.asset_a.mint(&alice, &1_000_000);
    f.asset_b.mint(&alice, &1_000_000);
    todo!(
        "add_liquidity(alice, 10_000, 10_000); expected shares == isqrt(10_000 \
         * 10_000) == 10_000; assert client.shares_of(&alice) == 10_000 and \
         reserves() == (10_000, 10_000)"
    )
}

#[test]
fn second_deposit_is_proportional() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    f.asset_a.mint(&alice, &1_000_000);
    f.asset_b.mint(&alice, &1_000_000);
    f.asset_a.mint(&bob, &1_000_000);
    f.asset_b.mint(&bob, &1_000_000);
    todo!(
        "alice deposits 10_000/10_000 (pool now 1:1); bob deposits \
         5_000/5_000 (still 1:1 ratio) and should get exactly half of \
         alice's share count; assert TotalShares math by checking \
         shares_of(bob) == shares_of(alice) / 2"
    )
}

#[test]
fn swap_moves_price_along_the_curve() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    let alice = Address::generate(&env);
    let trader = Address::generate(&env);
    f.asset_a.mint(&alice, &1_000_000);
    f.asset_b.mint(&alice, &1_000_000);
    f.asset_a.mint(&trader, &1_000_000);
    todo!(
        "alice deposits 100_000/100_000; trader swaps 1_000 of token_a in \
         with min_amount_out 0; assert the amount_out returned is slightly \
         less than 1_000 (because of the 0.3% fee and the constant-product \
         curve, NOT exactly 1_000 -- if you get exactly 1000 back your fee or \
         curve math is wrong); assert reserves moved accordingly"
    )
}

#[test]
fn swap_respects_min_amount_out() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    let alice = Address::generate(&env);
    let trader = Address::generate(&env);
    f.asset_a.mint(&alice, &1_000_000);
    f.asset_b.mint(&alice, &1_000_000);
    f.asset_a.mint(&trader, &1_000_000);
    todo!(
        "alice deposits 100_000/100_000; trader tries to swap 1_000 of \
         token_a in but demands min_amount_out way higher than the curve can \
         possibly give; assert try_swap returns Err(SlippageExceeded)"
    )
}

#[test]
fn remove_liquidity_returns_proportional_reserves() {
    let env = Env::default();
    env.mock_all_auths();
    let f = setup(&env);
    let alice = Address::generate(&env);
    f.asset_a.mint(&alice, &1_000_000);
    f.asset_b.mint(&alice, &1_000_000);
    todo!(
        "alice deposits 10_000/10_000, then removes ALL her shares; assert she \
         gets back (approximately, modulo integer division) her original \
         10_000/10_000 and TotalShares/reserves are back near zero"
    )
}
