#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, token, Address, Env};

// NOTE ON SCAFFOLDING: unlike the other stubs in this workspace, the
// marketplace logic below is already WORKING, not `todo!()`. The exercise
// here (per Tier 5 in the roadmap) is specifically to add
// `env.events().publish(...)` calls at each state change -- see the `// TODO:
// emit ...` comments. Fill those in, then fill in the corresponding
// `todo!()` tests in src/test.rs that assert on the published events.

#[derive(Clone)]
#[contracttype]
pub struct Listing {
    pub id: u32,
    pub seller: Address,
    pub token: Address,
    pub price: i128,
    pub active: bool,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    NextId,
    Listing(u32),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    ListingNotFound = 1,
    ListingNotActive = 2,
    NotSeller = 3,
}

#[contract]
pub struct Marketplace;

#[contractimpl]
impl Marketplace {
    pub fn __constructor(env: &Env) {
        env.storage().instance().set(&DataKey::NextId, &0u32);
    }

    /// `seller` must authorize. Lists `price` (in `token`) for a nominal item.
    /// Returns the new listing's id.
    pub fn list_item(env: &Env, seller: Address, token: Address, price: i128) -> u32 {
        seller.require_auth();

        let id: u32 = env.storage().instance().get(&DataKey::NextId).unwrap_or(0);
        env.storage().instance().set(&DataKey::NextId, &(id + 1));

        let listing = Listing {
            id,
            seller: seller.clone(),
            token,
            price,
            active: true,
        };
        env.storage().instance().set(&DataKey::Listing(id), &listing);

        // TODO: emit an event here, e.g.
        //   env.events().publish((symbol_short!("listed"), seller), (id, price));
        // Topics should let an off-chain indexer filter by event type and
        // seller without decoding every event's data payload.

        id
    }

    /// Only the original seller may cancel, and only while still active.
    pub fn cancel_listing(env: &Env, seller: Address, id: u32) -> Result<(), Error> {
        seller.require_auth();

        let mut listing: Listing = env
            .storage()
            .instance()
            .get(&DataKey::Listing(id))
            .ok_or(Error::ListingNotFound)?;

        if listing.seller != seller {
            return Err(Error::NotSeller);
        }
        if !listing.active {
            return Err(Error::ListingNotActive);
        }

        listing.active = false;
        env.storage().instance().set(&DataKey::Listing(id), &listing);

        // TODO: emit a "cancelled" event (id, seller).

        Ok(())
    }

    /// `buyer` must authorize. Pays the listing's price to the seller and
    /// deactivates the listing.
    pub fn buy_item(env: &Env, buyer: Address, id: u32) -> Result<(), Error> {
        buyer.require_auth();

        let mut listing: Listing = env
            .storage()
            .instance()
            .get(&DataKey::Listing(id))
            .ok_or(Error::ListingNotFound)?;

        if !listing.active {
            return Err(Error::ListingNotActive);
        }

        token::TokenClient::new(env, &listing.token).transfer(&buyer, &listing.seller, &listing.price);

        listing.active = false;
        env.storage().instance().set(&DataKey::Listing(id), &listing);

        // TODO: emit a "sold" event (id, seller, buyer, price) -- this is the
        // one an off-chain indexer would actually care most about, since it's
        // the event that represents money moving.

        Ok(())
    }

    /// Read-only.
    pub fn get_listing(env: &Env, id: u32) -> Option<Listing> {
        env.storage().instance().get(&DataKey::Listing(id))
    }
}

mod test;
