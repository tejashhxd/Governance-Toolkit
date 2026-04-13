#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Address, Vec};

#[derive(Clone)]
#[contracttype]
pub struct Proposal {
    pub id: u32,
    pub description: Symbol,
    pub yes_votes: u32,
    pub no_votes: u32,
}

#[contract]
pub struct GovernanceContract;

#[contractimpl]
impl GovernanceContract {
    // Create a proposal
    pub fn create_proposal(env: Env, id: u32, description: Symbol) {
        let proposal = Proposal {
            id,
            description,
            yes_votes: 0,
            no_votes: 0,
        };

        env.storage().instance().set(&id, &proposal);
    }

    // Vote on a proposal
    pub fn vote(env: Env, voter: Address, id: u32, support: bool) {
        voter.require_auth();

        let mut proposal: Proposal = env
            .storage()
            .instance()
            .get(&id)
            .unwrap();

        if support {
            proposal.yes_votes += 1;
        } else {
            proposal.no_votes += 1;
        }

        env.storage().instance().set(&id, &proposal);
    }

    // Get proposal details
    pub fn get_proposal(env: Env, id: u32) -> Proposal {
        env.storage().instance().get(&id).unwrap()
    }
}
