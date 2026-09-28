# 14 - Upgradeable Contract

**Tier 5 — Production-Grade Concerns**

## Goal

`upgrade(new_wasm_hash)`, admin-gated, using
`env.deployer().update_current_contract_wasm()` — how a deployed Soroban contract
can swap its own code without changing its address or losing its storage.

## Concepts

- `env.deployer().update_current_contract_wasm(hash)` — replaces this contract's
  code, keeps its storage and address
- Why the admin check MUST happen before touching the wasm hash — an attacker who
  can call `upgrade` at all can replace your contract's logic with anything,
  storage intact
- Migration hooks: a separate `migrate()` step for reshaping storage after a code
  swap, called deliberately rather than automatically, since it changes state and
  you generally want that traceable as its own transaction

## Tasks

- [ ] Implement `__constructor`, `version`
- [ ] Implement `upgrade` — auth first, then the wasm swap
- [ ] Implement `migrate`
- [ ] Fill in all four `todo!()` tests

## Why there's no "prove a real upgrade worked" test here

Actually swapping wasm requires a second, real compiled `.wasm` to upload — you
can't do it against native `cargo test` code alone. The tests here instead prove
the **auth gate** on `upgrade`/`migrate` is correct, which is the part most likely
to be gotten wrong (and the part that matters most: get the auth check right and a
bad upgrade at least requires a compromised admin key, not just a bug).

## Stretch goal (manual, not a unit test)

1. Build this crate, deploy it to testnet, note its contract id.
2. Bump `VERSION`'s initial value or add a new function, rebuild.
3. `stellar contract upload` the new wasm to get its hash.
4. Call `upgrade` on the deployed contract with that hash, then call your new
   function on the *same* contract id — proof the code changed under a stable
   address.

## Hints

- Run just this crate: `cargo test -p upgradeable-contract`
