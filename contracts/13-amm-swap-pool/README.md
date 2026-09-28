# 13 - AMM / Constant-Product Swap Pool

**Tier 4 — Composability & State Machines** (the capstone of this tier)

## Goal

`add_liquidity`, `remove_liquidity`, `swap`, implementing `x * y = k` for a single
token pair — the core mechanism behind every Uniswap-v2-style DEX.

## Concepts

- Precise fixed-point integer math with **no floats** — every formula in
  `src/lib.rs` is written as "multiply everything first, divide last" on purpose;
  reordering it will silently lose precision
- LP share accounting: first deposit sets the exchange rate via
  `isqrt(amount_a * amount_b)`, every later deposit must be proportional to the
  existing reserves or it either dilutes existing LPs or lets the depositor mint
  free value
- The constant-product swap formula with a fee baked in
  (`amount_out = reserve_out * amount_in_with_fee / (reserve_in + amount_in_with_fee)`)
- Slippage protection (`min_amount_out`) as the caller's only defense against the
  price moving between when they signed and when the transaction executed

## Tasks

- [ ] Implement `isqrt` first — a private helper, but everything else depends on it
- [ ] Implement `__constructor`, `reserves`, `shares_of`
- [ ] Implement `add_liquidity` — get the first-deposit vs. later-deposit branches
      both right
- [ ] Implement `remove_liquidity`
- [ ] Implement `swap` — this is the one to review twice before trusting your tests
- [ ] Fill in all five `todo!()` tests

## Hints

- This project has no separately-stored reserve fields on purpose — reserves are
  just `token::TokenClient::balance(&env.current_contract_address())` for each
  side. One source of truth, can't drift.
- **Read the reserves before you move any funds** in `add_liquidity` and
  `remove_liquidity` — if you read them after transferring in the new deposit,
  every ratio calculation is off by the amount you just added
- `i128` overflows silently panic in Soroban with overflow checks on (see the
  workspace `Cargo.toml`'s `overflow-checks = true`) — that's a feature here, not
  a bug: it'll catch a bad multiply-order before it becomes a real accounting bug
- Run just this crate: `cargo test -p amm-swap-pool`
