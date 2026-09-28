# 15 - Contract Factory

**Tier 5 — Production-Grade Concerns**

## Goal

A "factory" contract that deploys new instances of another contract on demand —
e.g. spin up a fresh escrow or token per user, all from one admin-controlled
dispatcher contract.

## Concepts

- `env.deployer().with_current_contract(salt).deploy_v2(wasm_hash, constructor_args)`
  (method name varies by SDK version — check yours) to deploy a new contract
  instance from inside another contract
- Deterministic addresses via `salt`: the new contract's address is derived from
  *this factory's own address* plus the salt, so the same salt can never be used
  twice from the same factory
- The split between "code" (a wasm hash, uploaded once) and "instance" (an address,
  deployed many times pointing at the same code) — this is the same split that
  makes `14-upgradeable-contract` possible

## Tasks

- [ ] Implement `__constructor`, `get_deployment`
- [ ] Implement `deploy` — admin auth, then the actual deployer call
- [ ] Fill in `non_admin_cannot_deploy` and `unknown_salt_has_no_deployment`
      (no real wasm needed for these)
- [ ] Once `01-counter` builds cleanly, do the real integration test (see below)

## Running the real end-to-end test

This one needs an actual compiled `.wasm` to deploy, so it's behind a Cargo feature
that's off by default — your normal `cargo test` / `cargo test --workspace` won't
try to build it:

```bash
cargo build -p counter --release --target wasm32v1-none
cargo test -p contract-factory --features factory-integration-test
```

Then fill in the `todo!()` inside `src/test.rs`'s `integration` module.

## Hints

- Double-check the exact deployer API for your `soroban-sdk` version — this has
  changed across releases (`deploy` vs `deploy_v2`, whether constructor args are
  passed inline or via a separate call). The doc comment in `src/lib.rs` flags
  this on purpose; don't just copy an example you find without checking the
  version it targets.
- Run just this crate's non-integration tests: `cargo test -p contract-factory`
