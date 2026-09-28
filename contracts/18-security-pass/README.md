# 18 - Security Pass on an Earlier Project

**Tier 5 — Production-Grade Concerns** (do this last, and do it on every contract
you're proud of, not just once)

No new contract here either — pick one you've already built (the roadmap suggests
`12-auction` or `13-amm-swap-pool` since they move real value and have the most
edge cases) and deliberately try to break it.

## Method

Write this as failing tests first, then fix the contract, then confirm the tests
pass. Don't just read the code looking for bugs — prove each one with a test that
fails against your current implementation.

## Checklist to run against your chosen contract

- [ ] **Missing auth checks** — for every function that changes state, is there a
      `require_auth()` on the right address? Try calling each one with
      `mock_auths` set to the *wrong* caller (see the pattern in
      `../04-ownable/src/test.rs`) and confirm it's rejected.
- [ ] **Integer overflow / underflow** — with `overflow-checks = true` in the
      workspace `Cargo.toml`, a raw overflow panics rather than wrapping, which
      is good, but a panic is still a denial-of-service if an attacker can
      trigger it on demand. Look for arithmetic that isn't behind
      `checked_add`/`checked_sub`/`checked_mul` and ask whether user-supplied
      values can reach it.
- [ ] **Reentrancy-shaped issues** — does any function do
      `transfer-then-update-state` instead of `update-state-then-transfer`
      (checks-effects-interactions)? `12-auction`'s `withdraw_refund` is written
      the safe way on purpose — compare against it.
- [ ] **Unauthorized state reads driving decisions** — e.g. a live balance
      re-read at decision time instead of a snapshot taken earlier (the
      token-weighted voting stretch goal in `10-voting-dao` calls this out
      directly)
- [ ] **Zero/negative amounts** — does `transfer(0)`, `bid(0)`, `mint(-1)`, etc.
      do something sensible, or something exploitable?
- [ ] **Double-spend / double-claim** — can any "once per X" action (a vote, a
      claim, an approval) be triggered twice through a code path you didn't
      consider?
- [ ] **Front-running / ordering** — does the outcome of any function depend on
      transaction ordering in a way a bidder/trader could exploit? (The AMM's
      `min_amount_out` is the mitigation for one instance of this — is it
      actually enforced everywhere it should be?)

## Task checklist

- [ ] Pick a contract
- [ ] For each checklist item above, write a test that demonstrates the issue
      (or confirms it's already handled) before changing any contract code
- [ ] Fix anything that broke
- [ ] Re-run the full test suite for that contract to confirm nothing else
      regressed
