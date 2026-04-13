# 🗳️ Governance Toolkit (Soroban Smart Contract)

## 📌 Project Description

Governance Toolkit is a decentralized governance smart contract built on the Stellar Soroban platform. It provides a simple and efficient way for communities to create proposals and participate in decision-making through on-chain voting.

This project demonstrates how decentralized governance (DAO-like systems) can be implemented on Stellar using Soroban smart contracts.

---

## ⚙️ What It Does

* Enables users to create governance proposals
* Allows users to vote (Yes / No) on proposals
* Stores all governance data securely on-chain
* Uses wallet authentication for secure participation

---

## ✨ Features

* 📝 **Proposal Creation**
  Create proposals with a unique ID and description

* 🗳️ **Voting Mechanism**
  Vote in favor or against proposals

* 🔐 **Secure Authentication**
  Uses Soroban's built-in `require_auth()` for secure voting

* 📦 **On-Chain Storage**
  All proposals and votes are permanently stored on the blockchain

* ⚡ **Lightweight & Scalable**
  Minimal design, easy to extend with advanced governance logic

---

## 🚀 Future Improvements

* One vote per wallet
* Voting deadlines and expiration
* Token-weighted voting
* Proposal execution system
* Delegation of votes

---

## 🔗 Deployed Smart Contract Link

**Contract Address:**
`CDRUXWBY4ECUYDQ54C7W56KDTH3ZERQLKYOSPD74MFXOVVTHK7XVGURL`

You can interact with the contract using the Soroban CLI or integrate it into a frontend application.

---

## 🛠️ Tech Stack

* Rust (Soroban SDK)
* Stellar Soroban Smart Contracts

---

## 📂 Project Structure

```
governance-toolkit/
│
├── src/
│   └── lib.rs
│
├── Cargo.toml
└── README.md
```

---

## 📖 Getting Started

### 1. Install Soroban CLI

```bash
cargo install soroban-cli
```

### 2. Build Contract

```bash
cargo build --target wasm32-unknown-unknown --release
```

### 3. Deploy Contract

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/governance_toolkit.wasm
```

---

## 🤝 Contributing

Contributions are welcome! Feel free to fork the repository and submit pull requests.

---

## 📜 License

MIT License
