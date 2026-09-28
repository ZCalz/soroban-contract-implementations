#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[contracttype]
pub enum Role {
    Admin,
    Moderator,
    User,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Role(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotAuthorized = 1,
    AlreadyHasRole = 2,
}

#[contract]
pub struct MultiRoleAccess;

#[contractimpl]
impl MultiRoleAccess {
    /// The deployer becomes the first Admin.
    pub fn __constructor(env: &Env, admin: Address) {
        todo!("admin.require_auth(); store Role::Admin under DataKey::Role(admin)")
    }

    /// Only an existing Admin may grant a role to `account`.
    pub fn grant_role(env: &Env, caller: Address, account: Address, role: Role) -> Result<(), Error> {
        todo!(
            "require caller has Role::Admin (return Err(NotAuthorized) if not), \
             then store `role` under DataKey::Role(account)"
        )
    }

    /// Only an existing Admin may revoke a role.
    pub fn revoke_role(env: &Env, caller: Address, account: Address) -> Result<(), Error> {
        todo!("require caller is Admin, then remove DataKey::Role(account)")
    }

    /// Read-only.
    pub fn get_role(env: &Env, account: Address) -> Option<Role> {
        todo!("return the stored role for `account`, or None")
    }

    /// Callable by Admin OR Moderator. A stand-in for some real moderation action.
    pub fn moderator_action(env: &Env, caller: Address) -> Result<(), Error> {
        todo!(
            "caller.require_auth(); check caller's role is Admin or Moderator, \
             else return Err(NotAuthorized)"
        )
    }

    /// Callable only by Admin. A stand-in for some real admin action.
    pub fn admin_action(env: &Env, caller: Address) -> Result<(), Error> {
        todo!("caller.require_auth(); check caller's role is Admin, else Err(NotAuthorized)")
    }
}

mod test;
