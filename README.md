# AYUDA Protocol 🛡️

**A decentralized identity and transparent fund distribution protocol leveraging NFC and Soroban Smart Contracts.**

---

## 🏗 System Evolution & Demo

### 1. Smart Contract Deployment
The foundation of the Ayuda protocol is built on the Stellar Testnet. This image confirms the successful deployment of the Soroban Wasm file to the network.

![Deployed](deployed.png)

### 2. Local Protocol Testing
Before going live, the core logic including `register_citizen` and `verify_citizen` was rigorously tested in a local environment to ensure state integrity and duplicate prevention.

![Testing](testing.png)

### 3. On-Chain Verification (Explorer)
Every transaction is publicly auditable. You can track the protocol's activity via the Stellar Expert Explorer.

**Contract ID:** [`CACB6NY66CGT2YGKBYJGOLSLPHZNYMZ5WBEGSNGYNGFFE7P6QSZYWV6S`](https://stellar.expert/explorer/testnet/contract/CACB6NY66CGT2YGKBYJGOLSLPHZNYMZ5WBEGSNGYNGFFE7P6QSZYWV6S)

![Stellar Explorer](explorer.png)

### 4. Admin Management Dashboard
The final user interface for GIST administrators. This minimalist dashboard bridges the NFC sensor data with the blockchain, allowing for seamless student registration and aid claims.

![Dashboard](dashboard.png)

---

## 📌 Project Overview
**Ayuda** is a decentralized identity and resource distribution system designed for institutional environments like the **Gateways Institute of Science and Technology (GIST)**. It solves the problem of "ghost recipients" and distribution leakage by using physical NFC cards as a "Proof-of-Presence" key.

### The Problem
* **Distribution Leakage:** Manual aid distribution is prone to errors and lack of real-time auditing.
* **Technical Barriers:** Students find blockchain wallets and gas fees too complex for simple resource access.

### The Solution
* **Hardware-Bound Identity:** NFC tags act as a secure identifier for students.
- **On-Chain Audit Trail:** Every registration is handled by a Soroban Smart Contract.
- **Hybrid Bridge Architecture:** A Rust backend allows a mobile phone to act as a remote NFC sensor.

---

## 🛠 Tech Stack
| Layer | Technology |
| :--- | :--- |
| **Smart Contract** | Rust, Soroban SDK, Stellar Network |
| **Backend** | Rust (Axum), Stellar CLI, Docker |
| **Frontend** | Next.js 14, Tailwind CSS, Web NFC API |
| **Infrastructure** | Render (Backend) |

---

## 🚀 Key Features
* **Minimalist Elegant UI:** High-contrast Black & White dashboard focused on speed.
* **Mobile-as-a-Sensor:** No expensive hardware; any NFC-enabled smartphone acts as the reader.
* **Zero Gas for Users:** The institution handles transaction costs for a seamless experience.

---

## 🔧 Installation & Setup

1. **Clone the Repository**
   ```bash
   git clone [https://github.com/rylsherdamz-rgb/stellar.git](https://github.com/rylsherdamz-rgb/stellar.git)
