# 07 - Fungible Token (SEP-41 style)

**Tier 3 — Tokens & Value Transfer**

## Goal

`mint`, `transfer`, `balance`, `approve` / `transfer_from`, `total_supply` — the
standard fungible token interface. `08-faucet-airdrop` will call this contract via
its generated client, so get the signatures right.

## Concepts

- The standard token interface shape (mirrors the Stellar Asset Contract / SEP-41)
- Allowances: `approve`, `allowance`, `transfer_from`, including **expiration**
  (`expiration_ledger`, checked against `env.ledger().sequence()`)
- Overflow-safe balance math — use `checked_add` / `checked_sub` and map `None` to
  an error instead of letting an operation silently wrap or panic with a confusing
  message
- Returning `Result<T, Error>` throughout, matching `06-multi-role-access`

## Tasks

- [ ] Implement `__constructor`, `mint`, `balance`, `total_supply`
- [ ] Implement `transfer`
- [ ] Implement `approve`, `allowance` (remember: expired ⇒ reads as 0)
- [ ] Implement `transfer_from`
- [ ] Fill in all seven `todo!()` tests, including the expiration and
      insufficient-allowance/-balance cases

## Hints

- `i128::checked_add` / `checked_sub` return `Option<i128>` — `.ok_or(Error::...)?`
  is the idiom
- `env.ledger().sequence()` gives the current ledger number; in tests, advance it
  with `env.ledger().set_sequence_number(new_value)` (needs
  `soroban_sdk::testutils::Ledger` imported, already in `test.rs`)
- This contract's generated `FungibleTokenClient` is exactly what you'll pass
  around as a `token: Address` parameter in later projects (faucet, escrow, AMM) —
  make sure `transfer` and `balance` behave exactly as callers would expect
- Run just this crate: `cargo test -p fungible-token`
