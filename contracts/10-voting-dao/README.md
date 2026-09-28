# 10 - Voting / DAO-lite

**Tier 4 — Composability & State Machines**

## Goal

`create_proposal`, `vote`, `tally`, `execute` on quorum — a minimal one-address-one-vote
governance contract.

## Concepts

- A proper multi-stage state machine: open → closed → (executed | failed quorum)
- Preventing double-voting with a `HasVoted(proposal_id, voter)` storage key
- Snapshotting vs. live-reading — see the `contractimpl` doc comment on why a
  token-weighted version needs to snapshot voting weight at vote time, not re-read
  live balances at tally time (classic "flash loan" style governance attack if you
  don't)
- Time-gated state transitions (`voting_ends_at`) layered on top of the auth checks
  from earlier tiers

## Tasks

- [ ] Implement `create_proposal`, `get_proposal`, `tally`
- [ ] Implement `vote` — auth, existence check, closed check, double-vote check
- [ ] Implement `execute` — closed check, already-executed check, quorum check
- [ ] Fill in the six `todo!()` tests

## Stretch goal

Make it token-weighted: take a `token: Address` in the constructor, and in `vote`,
read the voter's balance from a `07-fungible-token`-style contract and record *that*
as their vote weight (not a live balance re-read in `tally`). Think through why a
live re-read is exploitable before you build it.

## Hints

- `#[contracterror]` + `Result` again (same shape as `06-multi-role-access` and
  `07-fungible-token`) — by now this should feel familiar
- Run just this crate: `cargo test -p voting-dao`
