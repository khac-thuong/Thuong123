Community DAO & Leaderboard
Problem
Student communities and study groups often struggle to manage voting decisions, collect contributions, and track team performance in a transparent way.

Solution
We built a Soroban smart contract on Stellar that enables proposal voting, DAO fund contributions, and hackathon leaderboard management on-chain.

Why Stellar
Stellar Soroban provides fast, low-cost, and transparent smart contract execution, making community governance accessible to everyone.

Target User
Student clubs

Study groups

University hackathons

Small community organizations

Features
1. Community Voting
Create a proposal

Vote Yes / Vote No

View voting results

2. DAO Fund
Contribute funds to the community treasury

View total DAO balance

3. Hackathon Leaderboard
Submit team scores

View team scores & track hackathon performance

Live Demo
Network: Stellar Testnet

Contract ID: CDUGRR7PRCMEJ4UI3EE5NDQ6TOLJYVZCFVFERTD3A7DLDSF7ASC6NCTX

Transaction (Submit Score): https://stellar.expert/explorer/testnet/tx/30c0a12360feaf8a557064c0b1047130ad743ea7546cc9011a08bd9508707353

Smart Contract Functions
🟢 Voting
create_proposal(title: String)

vote_yes()

vote_no()

get_proposal()

🟢 DAO
contribute(amount: i128)

get_dao_balance()

🟢 Leaderboard
submit_score(team_name: String, score: u32)

get_score(team_name: String)

How to Run
1. Setup & Build
Bash
# Clone Repository
git clone https://github.com/yourname/community-dao.git

# Enter Project
cd community-dao

# Build Contract
stellar contract build

# Run Tests
cargo test
2. Deploy to Testnet
Bash
stellar contract deploy --wasm target/wasm32-unknown-unknown/release/community_dao.wasm --source-account student --network testnet
3. Interact via CLI (Single-line Commands)
Do một số Terminal của môi trường Cloud IDE không hỗ trợ ký tự xuống dòng \, hãy sử dụng các lệnh viết liền một dòng dưới đây:

Kích hoạt & Nạp tiền tài khoản admin trên Testnet:

Bash
stellar keys fund admin --network testnet
Tạo Đề xuất mới:

Bash
stellar contract invoke --id CDUGRR7PRCMEJ4UI3EE5NDQ6TOLJYVZCFVFERTD3A7DLDSF7ASC6NCTX --source admin --network testnet --send=yes -- create_proposal --title "Mua Tai Khoan ChatGPT Plus"
Gửi điểm số của Đội thi:

Bash
stellar contract invoke --id CDUGRR7PRCMEJ4UI3EE5NDQ6TOLJYVZCFVFERTD3A7DLDSF7ASC6NCTX --source admin --network testnet --send=yes -- submit_score --team_name "Team_Alpha" --score 95
Truy vấn xem điểm số:

Bash
stellar contract invoke --id CDUGRR7PRCMEJ4UI3EE5NDQ6TOLJYVZCFVFERTD3A7DLDSF7ASC6NCTX --source admin --network testnet -- get_score --team_name "Team_Alpha"
Tech Stack
Smart Contract: Rust

Framework: Soroban SDK v22

Blockchain: Stellar Testnet

Testing: Cargo Test

Team
Tran Khac Thuong

Information Technology Student | University Student

Vietnam
