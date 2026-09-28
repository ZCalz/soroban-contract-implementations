# 03 - Guessing Game

**Tier 1 — Fundamentals** (already built)

You already have this one at [`../../guessing-game-tutorial`](../../guessing-game-tutorial/contracts/guess-the-number).

No new scaffolding here — this folder is a placeholder so the numbering in
[../../README.md](../../README.md) and [../../../soroban-practice-projects.md](../../../soroban-practice-projects.md)
stays consistent.

## If you want to revisit it

- [ ] Re-read `guessing-game-tutorial/contracts/guess-the-number/src/lib.rs` cold,
      before looking at the tests — can you predict what each function does?
- [ ] Re-run its test suite: `cargo test -p guess-the-number` from that project's root
- [ ] Concepts it demonstrates worth internalizing: persistent/instance storage keyed
      by `Address`, a simple game state machine, and a custom `Error` enum used with
      `try_*` client methods
