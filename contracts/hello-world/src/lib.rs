
#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Address, Env, String,
};

#[contract]
pub struct Contract;

#[contracttype]
#[derive(Clone)]
pub struct Proposal {
    pub title: String,
    pub yes_votes: u32,
    pub no_votes: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct Team {
    pub name: String,
    pub score: u32,
}

#[contracttype]
pub enum DataKey {
    Proposal,
    DaoBalance,
    TeamScore(String),
}

#[contractimpl]
impl Contract {

    // =====================
    // COMMUNITY VOTING
    // =====================

    pub fn create_proposal(
        env: Env,
        title: String,
    ) {
        let proposal = Proposal {
            title,
            yes_votes: 0,
            no_votes: 0,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Proposal, &proposal);
    }

    pub fn vote_yes(env: Env) {

        let mut proposal: Proposal =
            env.storage()
                .persistent()
                .get(&DataKey::Proposal)
                .unwrap();

        proposal.yes_votes += 1;

        env.storage()
            .persistent()
            .set(&DataKey::Proposal, &proposal);
    }

    pub fn vote_no(env: Env) {

        let mut proposal: Proposal =
            env.storage()
                .persistent()
                .get(&DataKey::Proposal)
                .unwrap();

        proposal.no_votes += 1;

        env.storage()
            .persistent()
            .set(&DataKey::Proposal, &proposal);
    }

    pub fn get_proposal(
        env: Env,
    ) -> Proposal {

        env.storage()
            .persistent()
            .get(&DataKey::Proposal)
            .unwrap()
    }

    // =====================
    // STUDY GROUP DAO
    // =====================

    pub fn contribute(
        env: Env,
        user: Address,
        amount: i128,
    ) {

        user.require_auth();

        let current: i128 =
            env.storage()
                .persistent()
                .get(&DataKey::DaoBalance)
                .unwrap_or(0);

        env.storage()
            .persistent()
            .set(
                &DataKey::DaoBalance,
                &(current + amount),
            );
    }

    pub fn get_dao_balance(
        env: Env,
    ) -> i128 {

        env.storage()
            .persistent()
            .get(&DataKey::DaoBalance)
            .unwrap_or(0)
    }

    // =====================
    // HACKATHON LEADERBOARD
    // =====================

    pub fn submit_score(
        env: Env,
        team_name: String,
        score: u32,
    ) {

        env.storage()
            .persistent()
            .set(
                &DataKey::TeamScore(
                    team_name.clone()
                ),
                &score,
            );
    }

    pub fn get_score(
        env: Env,
        team_name: String,
    ) -> u32 {

        env.storage()
            .persistent()
            .get(
                &DataKey::TeamScore(
                    team_name
                )
            )
            .unwrap_or(0)
    }

}
