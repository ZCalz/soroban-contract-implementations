# 06 - Multi-Role Access Control

**Tier 2 — Access Control & Authorization**

## Goal

Extend the single-admin pattern from `04-ownable` into three roles — `Admin`,
`Moderator`, `User` — with role-gated functions and a proper `Error` type instead
of bare panics.

## Concepts

- `#[contracttype]` on an enum (`Role`) so it can live in storage and cross the
  client boundary
- `#[contracterror]` — returning `Result<T, Error>` from contract functions instead
  of panicking, so callers can use `try_*` and match on specific failure reasons
- `Map<Address, Role>` conceptually (here modeled as `DataKey::Role(Address)` +
  instance storage, matching the pattern from earlier projects)
- Layered permission checks: some functions require exactly `Admin`, others accept
  `Admin` or `Moderator`

## Tasks

- [ ] Implement `__constructor` — deployer becomes the first `Admin`
- [ ] Implement `grant_role` / `revoke_role` — Admin-only, return `Err` on failure
      rather than panicking
- [ ] Implement `get_role`
- [ ] Implement `moderator_action` (Admin or Moderator) and `admin_action` (Admin only)
- [ ] Fill in the four `todo!()` tests

## Hints

- Prefer `Result<(), Error>` + `try_*` client methods over `panic!` here — it's a
  better fit than `04-ownable`'s bare `require_auth()` panics, and you'll want the
  practice before `07-fungible-token`, which leans on this pattern heavily
- `caller.require_auth()` proves *who* is calling; your own role lookup decides
  *whether they're allowed* — don't conflate the two
- Run just this crate: `cargo test -p multi-role-access`
