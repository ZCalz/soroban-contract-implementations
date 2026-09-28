# Contracts index

Numbered folders match the tiers in [../../soroban-practice-projects.md](../../soroban-practice-projects.md).
Go in order — each tier leans on SDK concepts from the previous one.

| # | Folder | Has a Cargo crate? | Notes |
|---|--------|:---:|---|
| 01 | `01-counter` | yes | |
| 02 | `02-todo-list` | yes | |
| 03 | `03-guessing-game` | no | pointer to `../../guessing-game-tutorial` |
| 04 | `04-ownable` | yes | |
| 05 | `05-asset-manager` | no | pointer to `../../asset-manager` |
| 06 | `06-multi-role-access` | yes | |
| 07 | `07-fungible-token` | yes | |
| 08 | `08-faucet-airdrop` | yes | cross-contract calls into 07's interface |
| 09 | `09-escrow-vesting` | yes | |
| 10 | `10-voting-dao` | yes | |
| 11 | `11-escrow-arbitration` | yes | |
| 12 | `12-auction` | yes | |
| 13 | `13-amm-swap-pool` | yes | capstone of Tier 4 |
| 14 | `14-upgradeable-contract` | yes | |
| 15 | `15-contract-factory` | yes | integration test needs 01 pre-built, see its README |
| 16 | `16-events-indexer` | yes | logic pre-implemented; the exercise is adding events |
| 17 | `17-full-dapp` | no | guide only, apply to an earlier contract |
| 18 | `18-security-pass` | no | guide only, apply to an earlier contract |

Folders without a crate have no `Cargo.toml` and are excluded from the workspace
`members` list in [../Cargo.toml](../Cargo.toml) — `cargo build --workspace` /
`cargo test --workspace` skip them automatically.
