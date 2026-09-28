# 01 - Counter

**Tier 1 — Fundamentals**

## Goal

A single `u32` value in contract storage, with `increment()`, `get()`, and an
admin-gated `reset()`.

## Concepts

- `#[contract]` / `#[contractimpl]` macros
- The `Env` parameter and how the SDK auto-generates a `CounterClient`
- Instance storage (`env.storage().instance()`)
- `Address::require_auth()` for a single admin-gated function

## Tasks

- [ ] Implement `__constructor` — store the admin, initialize `COUNT` to 0
- [ ] Implement `increment` — read, add 1, write back, return new value
- [ ] Implement `get` — read-only accessor
- [ ] Implement `reset` — require the admin's auth before zeroing the count
- [ ] Fill in the three `todo!()` tests in `src/test.rs`
- [ ] Stretch: add a test proving a non-admin caller can't reset (see hint in test file)

## Hints

- `env.storage().instance().set(&COUNT, &value)` / `.get(&COUNT)` / `.has(&COUNT)`
- `env.storage().instance().get(&COUNT).unwrap_or(0)` is a clean way to default an
  unset counter to zero
- Run just this crate's tests with `cargo test -p counter` from the workspace root
