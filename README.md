# AYUDA Protocol 🛡️

Decentralized identity and transparent fund distribution system for institutions, built on Stellar.

---

## Problem

In many schools and institutions in the Philippines, aid distribution is still handled manually using paper lists or spreadsheets. This leads to ghost recipients, duplication, and lack of real-time tracking of who actually received assistance.

Administrators also have no reliable way to verify whether a student physically claimed their aid, and unclaimed funds often become untracked or unused.

---

## Solution

AYUDA Protocol uses NFC-based Proof-of-Presence combined with Soroban smart contracts on Stellar to ensure only verified students can claim aid. Each transaction is recorded on-chain, making distribution fully transparent.

If aid is not claimed, the system keeps it visible in the contract state and returns unclaimed funds back to the institution’s aid pool for future redistribution.

Settlement is instant, low-cost, and verifiable within seconds.

---

## Demo Flow (2 minutes)

1. Admin registers student via NFC scan
2. Aid is assigned to student wallet on-chain
3. Student taps NFC card (Proof-of-Presence)
4. Smart contract validates identity
5. Aid is released instantly to student wallet

---

## Architecture

```txt
Frontend (Next.js)
  |-- Web NFC API (identity scan)
  |-- Stellar SDK (transaction building)
  |-- Freighter Wallet (authentication)
  |-- Convex (real-time sync)

Stellar Testnet
  |-- Ayuda Soroban Contract (aid logic)
  |-- USDC / Token Contract (fund distribution)
```

No traditional backend required. All critical state is stored on-chain with Convex used only for UI syncing.

---

## Project Structure

```txt
ayuda-protocol/
├── contracts/
│   ├── src/
│   │   ├── lib.rs
│   │   └── test.rs
│   └── Cargo.toml
├── frontend/
│   ├── src/
│   │   ├── components/
│   │   ├── lib/
│   │   ├── views/
│   │   └── styles/
│   ├── public/
│   │   └── images/
│   │       ├── deployed.png
│   │       ├── test.png
│   │       ├── explorer.png
│   │       └── dashboard.png
└── README.md
```

---

## Stellar Features Used

| Feature                 | Usage                       |
| ----------------------- | --------------------------- |
| Soroban Smart Contracts | Core aid distribution logic |
| USDC / Tokens           | Aid funding and transfers   |
| Trustlines              | Student wallet eligibility  |
| Clawback                | Admin correction of errors  |
| Stellar Network         | Fast settlement & low fees  |

---

## Smart Contract

Deployed on Stellar Testnet:

```txt
CACB6NY66CGT2YGKBYJGOLSLPHZNYMZ5WBEGSNGYNGFFE7P6QSZYWV6S
```

Explorer:

```txt
https://stellar.expert/explorer/testnet/contract/CACB6NY66CGT2YGKBYJGOLSLPHZNYMZ5WBEGSNGYNGFFE7P6QSZYWV6S
```

### Contract Functions

| Function         | Description                      |
| ---------------- | -------------------------------- |
| register_citizen | Registers student identity       |
| fund_aid         | Assigns aid to student           |
| claim_aid        | Releases funds to student wallet |
| get_balance      | Checks remaining aid             |

---

## Escrow / Aid Lifecycle

```txt
Registered → Funded → Claimed → Closed
        ↘ Unclaimed → Returned to Aid Pool
```

If aid is not claimed within a defined period, it is not lost — it is returned to the institution’s shared aid pool for redistribution.

---

## Key Features

* NFC-based identity verification
* Fully transparent on-chain aid tracking
* Instant settlement via Soroban
* Zero dependency on traditional banking delays
* Reusable institutional aid pool for unclaimed funds

---

## Prerequisites

* Rust (latest stable)
* Soroban CLI v25+
* Node.js 18+
* Freighter Wallet (testnet)

---

## Setup

### Smart Contract

```bash
soroban contract build

soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/ayuda.wasm \
  --source deployer \
  --network testnet
```

### Frontend

```bash
cd frontend
npm install
npm run dev
```

---

## CLI Example

```bash
soroban contract invoke \
  --id CACB6NY66CGT2YGKBYJGOLSLPHZNYMZ5WBEGSNGYNGFFE7P6QSZYWV6S \
  --source admin \
  --network testnet \
  -- register_citizen \
  --citizen <ADDRESS> \
  --name "Juan Dela Cruz"
```

---

## Target Users

* Public schools and universities
* Government aid programs
* NGOs distributing financial assistance
* Students from low-income households in the Philippines

---

## Why Stellar

Stellar enables fast, low-cost, and transparent transactions that are ideal for institutional aid distribution. Soroban smart contracts ensure automation, while the blockchain guarantees auditability and trust without intermediaries.

---

## License

MIT License

