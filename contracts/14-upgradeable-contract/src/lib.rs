#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, BytesN, Env, Symbol};

#[contract]
pub struct UpgradeableContract;

const ADMIN: Symbol = symbol_short!("ADMIN");
const VERSION: Symbol = symbol_short!("VERSION");

#[contractimpl]
impl UpgradeableContract {
    pub fn __constructor(env: &Env, admin: Address) {
        todo!("store admin under ADMIN; set VERSION to 1u32")
    }

    /// Read-only.
    pub fn version(env: &Env) -> u32 {
        todo!("read and return VERSION")
    }

    /// Admin-only. Swaps this contract's own executable code for
    /// `new_wasm_hash`, which must already be uploaded to the network (e.g.
    /// via `stellar contract upload`, or in tests via
    /// `env.deployer().upload_contract_wasm(...)`). Storage is untouched by
    /// the swap -- only the code changes. Note this doesn't bump VERSION by
    /// itself; call `migrate` afterward if the new code needs that.
    pub fn upgrade(env: &Env, new_wasm_hash: BytesN<32>) {
        todo!(
            "load ADMIN and call .require_auth() on it, THEN call \
             env.deployer().update_current_contract_wasm(new_wasm_hash). Auth \
             must be checked first -- see the only_admin_can_call_upgrade test, \
             which relies on the auth failure happening before the wasm hash \
             is ever touched"
        )
    }

    /// Admin-only. Call this once, right after `upgrade`, if the new code
    /// needs to reshape existing storage (add a field with a default, rename
    /// a key, etc). A real migration would read old-shaped data and rewrite
    /// it; this stub just bumps VERSION so a test can prove the hook ran.
    pub fn migrate(env: &Env, new_version: u32) {
        todo!("require_auth on the stored admin, then set VERSION = new_version")
    }
}

mod test;
