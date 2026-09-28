#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::{Address as _, Ledger};

const PERIOD: u64 = 7 * 86_400; // 1 week

fn setup(env: &Env) -> VotingDaoClient {
    let contract_id = env.register(VotingDao, ());
    VotingDaoClient::new(env, &contract_id)
}

#[test]
fn create_and_read_proposal() {
    let env = Env::default();
    env.mock_all_auths();
    let client = setup(&env);
    let proposer = Address::generate(&env);
    todo!(
        "create_proposal(proposer, \"Ship it\", PERIOD); assert get_proposal \
         returns Some with votes_for == 0, votes_against == 0, executed == false"
    )
}

#[test]
fn voting_updates_tally() {
    let env = Env::default();
    env.mock_all_auths();
    let client = setup(&env);
    let proposer = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    todo!(
        "create a proposal; alice votes true, bob votes false; assert \
         tally(id) == (1, 1)"
    )
}

#[test]
fn cannot_vote_twice() {
    let env = Env::default();
    env.mock_all_auths();
    let client = setup(&env);
    let proposer = Address::generate(&env);
    let alice = Address::generate(&env);
    todo!(
        "create a proposal, vote once as alice, assert try_vote() for alice \
         again returns Err(AlreadyVoted)"
    )
}

#[test]
fn cannot_vote_after_close() {
    let env = Env::default();
    env.mock_all_auths();
    let client = setup(&env);
    let proposer = Address::generate(&env);
    let alice = Address::generate(&env);
    todo!(
        "create a proposal, advance time past PERIOD with \
         env.ledger().set_timestamp(...), assert try_vote() returns Err(VotingClosed)"
    )
}

#[test]
fn execute_requires_closed_voting_and_quorum() {
    let env = Env::default();
    env.mock_all_auths();
    let client = setup(&env);
    let proposer = Address::generate(&env);
    let alice = Address::generate(&env);
    todo!(
        "create a proposal; assert try_execute() fails with VotingStillOpen \
         while voting is open; vote alice=true; advance time past the period; \
         execute() should now succeed since votes_for > votes_against; a second \
         execute() call should return Err(AlreadyExecuted)"
    )
}

#[test]
fn execute_fails_without_quorum() {
    let env = Env::default();
    env.mock_all_auths();
    let client = setup(&env);
    let proposer = Address::generate(&env);
    let alice = Address::generate(&env);
    todo!(
        "create a proposal, vote alice=false (against), advance time past the \
         period, assert try_execute() returns Err(QuorumNotMet)"
    )
}
