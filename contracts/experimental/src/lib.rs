#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, contracterror, contractevent, Env, Address, String};


#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Role {
    Admin,
    User
}

#[contracttype]
pub struct Vehicle {
    brand: String,
    gas: u32,
    mileage: u32
}

#[contracttype]
pub enum DataKey {
    Role(Address),
    Owns(Address)
}
#[derive(Clone, Debug)]
#[contractevent]
pub struct GasAddedEvent {
    amount: u32,
}

#[derive(Clone, Debug)]
#[contractevent]
pub struct MilesDrivenEvent {
    miles: u32
}

#[contracterror]
#[repr(u32)]
pub enum VehicleError {
    NoVehicle = 0,
    NotEnoughGas = 1
}

#[contract]
pub struct Race;

impl Vehicle {
    pub fn new(env: &Env) -> Self {
        Vehicle{
            brand: String::from_str(env, "Default"),
            gas: 0u32,
            mileage: 0u32
        }
    }
}

#[contractimpl]
impl Race {
    pub fn __constructor(env: &Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Role(admin), &Role::Admin);
    }
    pub fn initialize_vehicle(env: &Env, admin: Address, account: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Owns(account), &Vehicle::new(&env));
    }

    pub fn set_admin(env: &Env, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Role(admin), &Role::Admin);
    }

    pub fn has_vehicle(env: &Env, account: Address) -> bool {
        Self::get_vehicle(env, account).is_some()
    }

    pub fn add_gas(env: &Env, account: Address) -> Result<(), VehicleError> {
        let vh = Self::get_vehicle(env, account.clone());
        if let Some(mut v) = vh {
            v.gas += 20u32;
            env.storage().instance().set(&DataKey::Owns(account), &v);
            let event = GasAddedEvent {
                amount: 20u32
            };
            event.publish(env);
            Ok(())
        } else {
            Err(VehicleError::NoVehicle)
        }
    }

    pub fn drive(env: &Env, caller: Address) -> Result<(), VehicleError> {
        caller.require_auth();
        
        let vh = Self::get_vehicle(env, caller.clone());
        if let Some(mut v) = vh {
            if v.gas < 5 {
                return Err(VehicleError::NotEnoughGas)
            }
            v.gas -= 5u32;
            v.mileage += 150u32;
            env.storage().instance().set(&DataKey::Owns(caller), &v);
            MilesDrivenEvent {
                miles: 150
            }.publish(env);
            Ok(())
        } else {
            Err(VehicleError::NoVehicle)
        }
    }

    pub fn get_vehicle(env: &Env, account: Address) -> Option<Vehicle> {
        env.storage().instance().get(&DataKey::Owns(account))
    }

    fn is_admin(env: Env, caller: Address) -> bool {
        let role: Role = env.storage().instance().get(&DataKey::Role(caller)).expect("No role set!");
        role == Role::Admin
    }

}

mod test;