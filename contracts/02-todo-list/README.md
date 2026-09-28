# 02 - Todo List

**Tier 1 — Fundamentals**

## Goal

`add_item(text)`, `complete_item(id)`, `get_items()` — a todo list namespaced per
caller address.

## Concepts

- `Vec<T>` and a `#[contracttype]` struct (`TodoItem`) in storage
- An enum `DataKey` for structured, collision-free storage keys
- Per-user storage namespacing (`DataKey::Items(owner)`)
- Iterating and mutating a `Vec` pulled out of storage

## Tasks

- [ ] Implement `add_item` — auth, allocate an id, push into the owner's Vec
- [ ] Implement `complete_item` — auth, find by id, flip the flag, write back
- [ ] Implement `get_items` — read-only, default to empty Vec
- [ ] Fill in the four `todo!()` tests in `src/test.rs`

## Hints

- `Vec<T>` in `soroban_sdk` has `push_back`, `get`, `set`, `iter()` — it's not `std::Vec`,
  but the API is close
- Because `Vec` is immutable-by-value from storage, the pattern is: load it, mutate
  your local copy, write the whole thing back
- Keep id allocation simple: store a running counter per owner (`DataKey::NextId`)
  rather than using `items.len()`, so ids stay stable after future deletions
- Run just this crate: `cargo test -p todo-list`
