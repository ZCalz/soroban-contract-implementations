# 08 - Faucet / Airdrop

**Tier 3 — Tokens & Value Transfer**

## Goal

Users claim a fixed token amount once per time window. This is your first
**cross-contract call**: the faucet doesn't hold its own ledger of balances, it
moves real tokens by calling another contract (your `07-fungible-token`, or any
SEP-41-shaped token, including a real Stellar Asset Contract).

## Concepts

- Cross-contract calls via a generated client (`soroban_sdk::token::TokenClient`)
  instead of `Env::invoke_contract` directly
- Timestamp-based rate limiting with `env.ledger().timestamp()`
- A contract acting as the `from` address in a token transfer — note that when the
  *calling contract itself* is the `from`, the host authorizes that transfer
  automatically as part of the same invocation (no signature needed, unlike a real
  user account)
- Testing with a real Stellar Asset Contract via `env.register_stellar_asset_contract_v2`,
  the same pattern used in `../../guessing-game-tutorial/contracts/guess-the-number/src/xlm.rs`

## Tasks

- [ ] Implement `__constructor`
- [ ] Implement `claim` — auth, rate-limit check, cross-contract transfer, update
      `LastClaim`
- [ ] Implement `time_until_claimable`
- [ ] Fill in the four `todo!()` tests, including the "second claim too soon" and
      "claim again after period elapses" cases (use `env.ledger().set_timestamp`)

## Hints

- `TokenClient::new(env, &token_address)` gives you `.transfer()` and `.balance()`
  regardless of which concrete contract is at that address, as long as it
  implements the standard interface — this is the same trick that lets `08` work
  against either your own `07-fungible-token` or a test SAC
- Don't forget to fund the faucet's own balance in your constructor/setup — an
  empty faucet will fail on the very first `claim`
- Run just this crate: `cargo test -p faucet-airdrop`
