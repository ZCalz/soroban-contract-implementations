# 11 - Escrow with Arbitration (2-of-3)

**Tier 4 — Composability & State Machines**

## Goal

Buyer, seller, arbiter; funds release to the seller or refund to the buyer once
any two of the three agree.

## Concepts

- Multi-party auth: more than one distinct `Address::require_auth()` caller across
  the contract's lifetime, each independently authenticated
- Dispute resolution as a voting tally over a fixed, tiny set of participants
  (contrast with `10-voting-dao`'s open-ended proposal system)
- Funding the escrow atomically inside the constructor by pulling from the buyer,
  rather than requiring a separate pre-funding step like `09-escrow-vesting` does

## Tasks

- [ ] Implement `__constructor` — pull funds from the buyer via a `token::TokenClient`
- [ ] Implement `approve` — auth, participant check, double-vote check, tally,
      and the actual payout once 2 votes agree
- [ ] Implement `resolved`
- [ ] Fill in all seven `todo!()` tests

## Hints

- There are only 3 possible voters, so "count votes matching mine" is just: look
  up `Vote(buyer)`, `Vote(seller)`, `Vote(arbiter)` (each `Option<bool>`) and count
  how many equal `Some(release_to_seller)` — no need for a general-purpose tally
  structure
- A split 1-1 vote (e.g. buyer says release, seller says refund) should NOT resolve
  anything — only the arbiter breaking the tie (or either original party changing
  their mind, if you allow that — you don't have to) should move funds
- Run just this crate: `cargo test -p escrow-arbitration`
