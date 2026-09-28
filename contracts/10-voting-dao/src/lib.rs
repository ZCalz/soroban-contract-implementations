#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, String};

#[derive(Clone)]
#[contracttype]
pub struct Proposal {
    pub id: u32,
    pub description: String,
    pub votes_for: u32,
    pub votes_against: u32,
    pub voting_ends_at: u64,
    pub executed: bool,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    NextId,
    Proposal(u32),
    /// Has this address already voted on this proposal id.
    HasVoted(u32, Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    ProposalNotFound = 1,
    AlreadyVoted = 2,
    VotingClosed = 3,
    VotingStillOpen = 4,
    AlreadyExecuted = 5,
    QuorumNotMet = 6,
}

#[contract]
pub struct VotingDao;

/// This is intentionally one-address-one-vote for simplicity. A good stretch
/// goal: make it token-weighted by taking a `token: Address` in the constructor
/// and using its `balance()` (see 07-fungible-token) as vote weight -- snapshot
/// the weight in `vote()` rather than re-reading the live balance in `tally()`,
/// otherwise voters could vote, transfer tokens to a fresh address, and vote again.
#[contractimpl]
impl VotingDao {
    /// Anyone may create a proposal. Returns the new proposal's id.
    pub fn create_proposal(env: &Env, proposer: Address, description: String, voting_period_seconds: u64) -> u32 {
        todo!(
            "proposer.require_auth(); allocate the next id (DataKey::NextId); \
             store a Proposal with votes at 0, executed = false, and \
             voting_ends_at = env.ledger().timestamp() + voting_period_seconds"
        )
    }

    /// `voter` must authorize. Each address may vote once per proposal.
    pub fn vote(env: &Env, voter: Address, proposal_id: u32, support: bool) -> Result<(), Error> {
        todo!(
            "voter.require_auth(); load the Proposal (Err(ProposalNotFound) if \
             missing); Err(VotingClosed) if now >= voting_ends_at; \
             Err(AlreadyVoted) if HasVoted(proposal_id, voter) is set; otherwise \
             increment votes_for or votes_against, write the Proposal back, and \
             set HasVoted(proposal_id, voter) = true"
        )
    }

    /// Read-only.
    pub fn get_proposal(env: &Env, proposal_id: u32) -> Option<Proposal> {
        todo!("return DataKey::Proposal(proposal_id)")
    }

    /// Read-only convenience wrapper: (votes_for, votes_against).
    pub fn tally(env: &Env, proposal_id: u32) -> (u32, u32) {
        todo!("load the proposal, return (votes_for, votes_against)")
    }

    /// Anyone may call this once voting has closed. Marks the proposal executed
    /// if votes_for > votes_against; otherwise returns Err(QuorumNotMet).
    /// (A real DAO would also *do* something here -- e.g. a cross-contract call
    /// chosen by the proposal. Here, "execute" just flips the flag; treat wiring
    /// up a real action as a stretch goal.)
    pub fn execute(env: &Env, proposal_id: u32) -> Result<(), Error> {
        todo!(
            "load the proposal (Err(ProposalNotFound) if missing); \
             Err(VotingStillOpen) if now < voting_ends_at; \
             Err(AlreadyExecuted) if already executed; \
             Err(QuorumNotMet) if votes_for <= votes_against; \
             otherwise set executed = true and write it back"
        )
    }
}

mod test;
