# Smart Contract Tutorials (Soroban)

A hands-on Cargo workspace of 18 Soroban contracts, ordered simple → complex.
Full rationale and tier breakdown: [../soroban-practice-projects.md](../soroban-practice-projects.md).

Every real contract crate under `contracts/` ships as a **stub**: the struct, storage
keys, and function signatures are in place, function bodies are `todo!()`, and each
has a `README.md` with the goal, concepts, a task checklist, and hints. Your job is to
fill in the bodies and make the tests in `src/test.rs` pass (also stubbed with `todo!()`).

## Layout

```
contracts/
  01-counter/              # Tier 1 - fundamentals
  02-todo-list/
  03-guessing-game/        # pointer -> ../../guessing-game-tutorial (already built)
  04-ownable/               # Tier 2 - access control
  05-asset-manager/        # pointer -> ../../asset-manager (already built)
  06-multi-role-access/
  07-fungible-token/        # Tier 3 - tokens & value transfer
  08-faucet-airdrop/
  09-escrow-vesting/
  10-voting-dao/            # Tier 4 - composability & state machines
  11-escrow-arbitration/
  12-auction/
  13-amm-swap-pool/
  14-upgradeable-contract/  # Tier 5 - production-grade concerns
  15-contract-factory/
  16-events-indexer/
  17-full-dapp/             # pointer/guide, applied to an earlier contract
  18-security-pass/         # pointer/guide, applied to an earlier contract
```

`03`, `05`, `17`, and `18` have no `Cargo.toml` — they're guidance-only folders
(see `contracts/README.md`) since they reuse projects you already have or apply to a
contract you pick later.

## Workflow for each contract

```bash
cd contracts/01-counter

# implement the todo!() bodies in src/lib.rs, then:
cargo test                       # run src/test.rs against your implementation

# once it builds and tests pass:
stellar contract build
stellar contract deploy \
  --wasm ../../target/wasm32v1-none/release/<crate_name>.wasm \
  --source <your-identity> \
  --network testnet
```

Or from the workspace root, target any crate directly:

```bash
cargo test -p counter
cargo build -p counter --release --target wasm32v1-none
```

## Order of attack

Go in numeric order — each tier assumes the SDK concepts from the previous one.
Don't skip writing the tests; the task checklist in each project's `README.md`
tells you exactly what to assert (happy path, unauthorized caller, boundary values,
re-invocation), matching the testing habit in the roadmap doc.
