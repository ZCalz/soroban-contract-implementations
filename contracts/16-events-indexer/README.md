# 16 - Events + Indexer-Friendly Contract

**Tier 5 — Production-Grade Concerns**

## Goal

Take a small, already-working marketplace contract (`list_item`, `cancel_listing`,
`buy_item`) and make it indexer-friendly by publishing structured events at every
state change. Then write a tiny off-chain script that listens for them.

## This project is scaffolded differently on purpose

Every other project in this workspace stubs out the core logic with `todo!()`.
Here, the marketplace logic in `src/lib.rs` is already implemented and its first
three tests already pass — because the actual exercise (per the roadmap) is
specifically *adding events to already-working code*, not writing the code itself.

## Concepts

- `env.events().publish((topics...), data)` — topics are for cheap, indexed
  filtering (event type, an address), data is the richer payload
- Designing events for off-chain consumption: what would a UI or indexer actually
  need to know from a "sold" event without re-fetching contract state?
- Reading events back in tests via `soroban_sdk::testutils::Events` /
  `env.events().all()`
- The RPC `getEvents` endpoint, and the basic shape of a polling indexer
  (`scripts/listen.mjs`)

## Tasks

- [ ] Read `src/lib.rs` — note the three `// TODO: emit ...` comments in
      `list_item`, `cancel_listing`, and `buy_item`
- [ ] Add an `env.events().publish(...)` call at each of those three points
- [ ] Fill in the three event-assertion `todo!()` tests in `src/test.rs`
- [ ] Fill in the two `TODO`s in `scripts/listen.mjs` (starting ledger,
      the actual `getEvents` poll) and run it against a testnet deployment

## Hints

- `symbol_short!("listed")` etc. make good first topics — keep topics small and
  cheap; put the richer stuff (price, buyer, seller) in the data argument
- If you're not sure what shape `env.events().all()` returns in a test, print it
  once (`std::println!("{:?}", env.events().all())` — note single braces here,
  it's a real format string, not the escaped one in the TODO text) and look
  before you write the assertion
- Run just this crate: `cargo test -p events-indexer`
