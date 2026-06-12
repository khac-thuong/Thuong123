
#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    Address,
    Env,
    String,
};

#[test]
fn test_voting() {

    let env = Env::default();

    let contract_id =
        env.register(Contract, ());

    let client =
        ContractClient::new(
            &env,
            &contract_id
        );

    client.create_proposal(
        &String::from_str(
            &env,
            "Buy ChatGPT Plus"
        )
    );

    client.vote_yes();
    client.vote_yes();
    client.vote_no();

    let proposal =
        client.get_proposal();

    assert_eq!(
        proposal.yes_votes,
        2
    );

    assert_eq!(
        proposal.no_votes,
        1
    );
}

#[test]
fn test_dao() {

    let env = Env::default();

    let contract_id =
        env.register(Contract, ());

    let client =
        ContractClient::new(
            &env,
            &contract_id
        );

    let user =
        Address::generate(&env);

    client.contribute(
        &user,
        &100
    );

    let balance =
        client.get_dao_balance();

    assert_eq!(
        balance,
        100
    );
}

#[test]
fn test_leaderboard() {

    let env = Env::default();

    let contract_id =
        env.register(Contract, ());

    let client =
        ContractClient::new(
            &env,
            &contract_id
        );

    client.submit_score(
        &String::from_str(
            &env,
            "Team Alpha"
        ),
        &95
    );

    let score =
        client.get_score(
            &String::from_str(
                &env,
                "Team Alpha"
            )
        );

    assert_eq!(
        score,
        95
    );
}
