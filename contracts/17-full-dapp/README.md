# 17 - Full dApp: Frontend + Contract Integration

**Tier 5 — Production-Grade Concerns**

No new contract here — this is an exercise applied to a contract you've already
built. Take any Tier 3–4 contract (the fungible token, the auction, and the
voting DAO are all good candidates — they have enough surface area to be
interesting in a UI) and wire up a real frontend.

## Two ways to do it

1. **Scaffold Stellar** (recommended for your first pass) — mirror the setup in
   [`../../../guessing-game-tutorial`](../../../guessing-game-tutorial). It gives
   you generated TypeScript bindings, a wallet-connect flow, and a dev server
   out of the box, so you're focused on wiring UI to contract calls rather than
   plumbing.
2. **Plain `stellar-sdk` (JS/TS)** — more manual, more educational the first time:
   you build the transaction, simulate it, sign with a wallet, and submit it
   yourself. Good to do at least once so Scaffold Stellar's generated code isn't
   a black box.

## Concepts

- Building and parsing XDR (the wire format Soroban transactions use)
- Simulating a transaction before submitting (`simulateTransaction`) to get
  accurate resource fees and catch errors before spending a real submission
- Signing with the Freighter browser wallet
- Surfacing contract errors (your `#[contracterror]` enums) as readable UI states
  instead of raw XDR/host errors

## Task checklist

- [ ] Pick a contract (fungible token, auction, or voting DAO are good choices)
      and make sure its Tier 3/4 `todo!()`s are filled in and tests pass first
- [ ] Deploy it to testnet
- [ ] Build a minimal UI: connect wallet, read contract state, submit at least
      one state-changing call (mint/transfer, or bid, or vote)
- [ ] Handle at least one error case in the UI (e.g. insufficient balance, voting
      closed) by matching on the contract's `Error` enum, not just showing a raw
      failure
- [ ] Stretch: add a live-updating view using the events you may have added in
      `../16-events-indexer` if you built on the same contract
