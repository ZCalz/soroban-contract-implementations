# 09 - Escrow / Vesting

**Tier 3 — Tokens & Value Transfer**

## Goal

Lock tokens for a beneficiary and release them linearly after a cliff — the classic
token-vesting shape (think: employee grants, investor unlocks).

## Concepts

- Time-based logic: cliff + linear unlock, driven off `env.ledger().timestamp()`
- Holding custody of another contract's tokens (the escrow contract itself is the
  token holder, same self-authorization idea as `08-faucet-airdrop`)
- Partial-withdrawal accounting: tracking `ClaimedAmount` separately from
  `vested_amount()` so repeated claims only ever release the delta

## Tasks

- [ ] Implement `__constructor`
- [ ] Implement `vested_amount` — the actual math: 0 before cliff, linear from
      `start` through `start + duration`, full amount after
- [ ] Implement `claimable_amount` and `claim`
- [ ] Fill in the five `todo!()` tests — pay special attention to
      `linear_vesting_between_cliff_and_end`, which is the one that actually
      exercises your math, not just the 0%/100% edges

## Hints

- The cliff only gates *when claiming starts* — don't accidentally make vesting
  restart its linear clock at `start + cliff`. It's `TOTAL * (now - start) / duration`,
  just clamped to 0 before the cliff and to `TOTAL` after `start + duration`.
- Integer division will floor — don't assert exact equality at the halfway point,
  assert it's within a few units
- Advance time in tests with `env.ledger().set_timestamp(new_value)` (needs
  `soroban_sdk::testutils::Ledger`, already imported in `test.rs`)
- Run just this crate: `cargo test -p escrow-vesting`
