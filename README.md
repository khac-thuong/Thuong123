# Community DAO & Leaderboard

## Problem

Student communities and study groups often struggle to manage voting decisions, collect contributions, and track team performance in a transparent way.

## Solution

We built a Soroban smart contract on Stellar that enables proposal voting, DAO fund contributions, and hackathon leaderboard management on-chain.

## Why Stellar

Stellar Soroban provides fast, low-cost, and transparent smart contract execution, making community governance accessible to everyone.

## Target User

* Student clubs
* Study groups
* University hackathons
* Small community organizations

## Features

### Community Voting

* Create a proposal
* Vote Yes
* Vote No
* View voting results

### DAO Fund

* Contribute funds to the community treasury
* View total DAO balance

### Hackathon Leaderboard

* Submit team scores
* View team scores
* Track hackathon performance

## Live Demo

* Network: Stellar Testnet

* Contract ID: `YOUR_CONTRACT_ID`

* Transaction: `YOUR_TRANSACTION_HASH`

## Example Usage

### Create Proposal

Input:

Buy ChatGPT Plus

### Vote

* vote_yes()
* vote_no()

Result:

Title: Buy ChatGPT Plus

Yes Votes: 2

No Votes: 1

### DAO Contribution

Input:

100

Result:

DAO Balance = 100

### Submit Team Score

Input:

Team Alpha

95

Result:

Team Alpha Score = 95

## How to Run

### Clone Repository

```bash
git clone https://github.com/yourname/community-dao.git
```

### Enter Project

```bash
cd community-dao
```

### Build Contract

```bash
stellar contract build
```

### Run Tests

```bash
cargo test
```

### Deploy to Testnet

```bash
stellar contract deploy \
--wasm target/wasm32-unknown-unknown/release/community_dao.wasm \
--source-account student \
--network testnet
```

## Smart Contract Functions

### Voting

```rust
create_proposal(title)
vote_yes()
vote_no()
get_proposal()
```

### DAO

```rust
contribute(amount)
get_dao_balance()
```

### Leaderboard

```rust
submit_score(team_name, score)
get_score(team_name)
```

## Tech Stack

* Smart Contract: Rust
* Framework: Soroban SDK v22
* Blockchain: Stellar Testnet
* Testing: Cargo Test

## Team

* Tran Khac Thuong
* Information Technology Student
* tt4060779@gmail.com
* SaiGonTech
