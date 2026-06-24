# TrashToCash: Eco-Reward App

**TrashToCash** - A blockchain-powered community platform built on the **Stellar Network** using **Soroban Smart Contracts**. This project aims to increase community recycling by instantly rewarding sorted waste deposits with stablecoin assets.

---

## 📌 Problem Statement
Public participation in waste sorting remains low because recycling centers lack immediate, easy, and satisfying incentive systems.

Currently, this issue faces several hurdles:
1. **Low Public Interest:** Recycling is viewed as a chores with no direct benefit.
2. **Slow Compensations:** Waste center redemptions use paper coupons or slow bank checks.
3. **Auditing Deficits:** Hard to track verified recycling metrics in municipalities.

## 🚀 Urgency
Plastic pollution damages local ecosystems daily. Sending immediate stablecoins to recyclers' digital wallets on deposition triggers strong feedback loops.
* **Instant Reward Loops:** 3-second stablecoin settlements make recycling satisfying.
* **Verified Metrics:** Municipalities audit logged recycling volumes transparently.

## ✨ Key Features
* **Instant Stablecoin Reward:** Payments arrive at recycler wallets under 5 seconds.
* **Weight and Type Audits:** Ensures point validity via certified depot loggers.
* **Decentralized Deposit Logs:** Provides auditable recycling stats.

---

## 🛠 Technical Stack
* **Blockchain:** Stellar Network
* **Smart Contract Engine:** Soroban
* **Language:** Rust
* **Development Environment:** Soroban CLI / Rust toolchain

## 📋 Smart Contract Overview
The contract handles eco-rewards waste deposits logging and payout histories:
1. `get_deposits()`: Fetch all recycling deposits logged in the contract.
2. `record_waste_deposit(admin, user, weight_grams, reward)`: Record a verified waste deposit and execute stablecoin reward payouts.

---

## 💡 Future Roadmap
- [ ] **Reverse Vending integration:** Connect smart contract rewards directly to automated recycling bins.
- [ ] **Retailer Coupons:** Allow users to redeem recycling points for grocery store discount coupons.
- [ ] **Eco-Scores CV:** Allow students/workers to showcase environmental credentials on resumes.

---
## Screenshots
<img width="1920" height="1080" alt="image" src="screenshot.png" />

---
Stellar ID: G2UJLJG3G46SHRIRYMML2DUUNL6FCVIOOYAJQWSDP6WQ34QOHXO3G576
*Developed for the Stellar Community and the advancement of Decentralized Social Economies.*
