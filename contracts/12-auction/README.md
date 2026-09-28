# 12 - On-Chain Auction (English)

**Tier 4 — Composability & State Machines**

## Goal

Bid, outbid refunds, close. The interesting part isn't the happy path — it's doing
refunds *safely*.

## Concepts

- The pull-over-push refund pattern: outbid funds sit in `Refundable(address)`
  until the loser withdraws them, rather than being pushed back automatically
  inside `bid()` — read the doc comment at the top of `src/lib.rs` for why
- Checks-effects-interactions ordering: zero out `Refundable` *before* transferring,
  so a `withdraw_refund` can't be called again before its effect lands
- Auction lifecycle timing: open → (bids) → ended → closed, gated purely by
  `env.ledger().timestamp() vs. EndTime`
- Combining your own contract state with calls into a token contract, now with
  three separate token-moving code paths (bid, refund, close) that all have to
  stay consistent

## Tasks

- [ ] Implement `__constructor`
- [ ] Implement `bid` — the core state transition, including crediting the
      previous bidder's refund
- [ ] Implement `withdraw_refund` — zero-then-transfer, not transfer-then-zero
- [ ] Implement `close` — including the "no bids at all" edge case
- [ ] Implement `highest_bid` / `highest_bidder`
- [ ] Fill in all six `todo!()` tests

## Stretch goal

Wire `close()` up to actually transfer a real asset to the winner — e.g. a token id
in a small registry contract of your own, transferred atomically alongside the
seller payout.

## Hints

- Resist the urge to `transfer()` the refund straight back to the outbid bidder
  inside `bid()` — that's the "push" pattern this project is explicitly steering
  you away from
- `Option<Address>` for `HighestBidder` (no bids yet ⇒ `None`) keeps `close()`'s
  no-bids case clean — don't use a sentinel address
- Run just this crate: `cargo test -p auction`
