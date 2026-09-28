# 04 - Ownable

**Tier 2 — Access Control & Authorization**

## Goal

`init(admin)` (via constructor), `set_value()` (admin-only), `transfer_ownership()`.
The "hello world" of access control — nearly every later contract in this workspace
builds on this admin pattern.

## Concepts

- `Address::require_auth()` — the core primitive for "did this account actually sign"
- Storing an admin `Address` and checking it before mutation
- `MockAuth` / `MockAuthInvoke` in tests to simulate a *specific, wrong* caller
  (as opposed to `mock_all_auths()`, which lets everything through)

## Tasks

- [ ] Implement `__constructor`, `set_value`, `get_value`, `admin`
- [ ] Implement `require_admin` (private helper) and use it in `set_value` and
      `transfer_ownership`
- [ ] Implement `transfer_ownership`
- [ ] Fill in the four `todo!()` tests, especially `non_admin_cannot_set_value` —
      this is the one that actually proves your auth check works, not just that the
      happy path compiles

## Hints

- `env.mock_all_auths()` makes *every* `require_auth()` call succeed regardless of
  who's calling — great for happy-path tests, useless for proving access control.
  Use `mock_auths(&[...])` with a specific `MockAuth` to prove the *wrong* caller
  gets rejected.
- `client.try_set_value(&42)` (the `try_` prefix) returns a `Result` instead of
  panicking, so you can assert on the error instead of using `#[should_panic]`
- Run just this crate: `cargo test -p ownable`
