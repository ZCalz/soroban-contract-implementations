#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, BytesN, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Deployment(BytesN<32>),
}

#[contract]
pub struct ContractFactory;

#[contractimpl]
impl ContractFactory {
    pub fn __constructor(env: &Env, admin: Address) {
        todo!("store admin")
    }

    /// Admin-only. Deploys a new instance of whatever contract `wasm_hash`
    /// points to -- that wasm must already be uploaded to the network (e.g.
    /// via `stellar contract upload`, or in tests via
    /// `env.deployer().upload_contract_wasm(...)`; see src/test.rs).
    ///
    /// `salt` makes the new contract's address deterministic: this factory's
    /// own address + `salt` together determine the resulting address, so
    /// deploying twice with the same salt fails the second time (the address
    /// is already occupied) -- that's a feature, not a bug, if you want
    /// "one deployment per user" by using the user's address as the salt.
    ///
    /// `owner` is forwarded to the new contract's constructor -- here
    /// specialized to a single `Address` argument, matching 01-counter's
    /// constructor shape. A more general factory would take `Vec<Val>`
    /// instead of a fixed `Address`; that's a good stretch goal once this
    /// works end to end.
    pub fn deploy(env: &Env, wasm_hash: BytesN<32>, salt: BytesN<32>, owner: Address) -> Address {
        todo!(
            "require_auth on the stored admin; call \
             env.deployer().with_current_contract(salt).deploy_v2(wasm_hash, (owner,)) \
             -- double check your soroban-sdk version's exact deployer API, the \
             method name/shape has shifted across releases (deploy vs. deploy_v2, \
             whether constructor args are passed here or in a separate call); \
             store the returned Address under Deployment(salt); return it"
        )
    }

    /// Read-only.
    pub fn get_deployment(env: &Env, salt: BytesN<32>) -> Option<Address> {
        todo!("return Deployment(salt)")
    }
}

mod test;
