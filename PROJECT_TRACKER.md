# Bitcoin Node UI - Project Tracker

**Project Goal:** Build a React UI for managing a Bitcoin node with wallet functionality, basic blockchain info, and node management capabilities. This project serves as a learning tool for Bitcoin development and Rust programming.

**Target Skills:**
- Bitcoin protocol fundamentals
- Rust backend development (Axum, Tokio)
- Bitcoin RPC integration
- Wallet management and cryptography
- React/TypeScript frontend development

---

## Architecture Overview

```
┌─────────────────────┐
│   React Frontend    │  Next.js 16, TypeScript, Tailwind CSS
│   (Port: .env)      │
└──────────┬──────────┘
           │ HTTP/REST
           ▼
┌─────────────────────┐
│   Rust Backend      │  Axum, Tokio, Tower
│   (Port: .env)      │
└──────────┬──────────┘
           │ JSON-RPC
           ▼
┌─────────────────────┐
│   Bitcoin Node      │  Bitcoin Core (local/external)
│   (Port: 8332)      │
└─────────────────────┘
```

---

## ✅ Completed Features

### Backend Infrastructure
- [x] **Rust/Axum Server Setup** ([main.rs:18-36](backend/src/main.rs#L18-L36))
  - Tokio async runtime
  - Graceful shutdown handling
  - Environment variable configuration
  - Application state management

- [x] **Bitcoin RPC Client** ([bitcoin_rpc.rs:38-137](backend/src/services/bitcoin_rpc.rs#L38-L137))
  - HTTP client with Basic auth
  - Generic RPC request handler
  - Error handling and response parsing
  - Methods implemented:
    - `getblockchaininfo` - Get blockchain state
    - `getblockcount` - Get current block height

- [x] **API Endpoints** ([node.rs:10-26](backend/src/routes/node.rs#L10-L26))
  - `GET /node/info` - Node information (network, height, sync status)
  - `GET /node/block-count` - Current block count
  - `GET /health` - Health check endpoint

- [x] **Error Handling System**
  - Custom error types for Bitcoin RPC errors
  - Structured API responses with success/error states
  - Proper HTTP status codes

- [x] **CORS Configuration**
  - Cross-origin support for frontend integration

### Frontend Infrastructure
- [x] **Next.js Setup**
  - TypeScript configuration
  - Tailwind CSS styling system
  - React 19
  - Basic project structure

### Domain Models
- [x] **Node Information** ([domain/node.rs:4-31](backend/src/domain/node.rs#L4-L31))
  - Network type detection (mainnet, testnet, signet, regtest)
  - Block height and best block hash
  - Sync status and progress
  - Difficulty tracking
  - Pruned node detection

---

## 🚧 In Progress

### Nothing currently in active development

---

## 📋 Next Steps - Priority Order

### Phase 1: Basic UI Foundation (Immediate Next)

#### 1.1 Node Dashboard UI
**Goal:** Display basic node information in the React UI

**Tasks:**
- [ ] Create API client/service layer in frontend
  - Setup axios or fetch wrapper
  - Type definitions for API responses
  - Error handling utilities

- [ ] Build Node Info Dashboard component
  - Display network type (mainnet/testnet/regtest)
  - Show current block height
  - Display sync progress bar
  - Show best block hash
  - Display difficulty
  - Connection status indicator

- [ ] Add auto-refresh/polling
  - Update node info every 10-30 seconds
  - WebSocket support (future enhancement)

**Learning Focus:**
- Bitcoin network types and their purposes
- What block height and sync status mean
- How to read blockchain state

**Files to Create/Modify:**
- `frontend/src/lib/api/client.ts` - API client
- `frontend/src/types/node.ts` - Type definitions
- `frontend/src/components/NodeDashboard.tsx` - Main dashboard
- `frontend/src/components/NodeInfo.tsx` - Info display component

---

#### 1.2 Node Connection Management
**Goal:** Allow connecting to different Bitcoin nodes (local/external)

**Backend Tasks:**
- [ ] Create node configuration endpoints
  - `POST /node/config` - Update node connection settings
  - `GET /node/config` - Get current configuration
  - Validate connection before saving

- [ ] Store node configurations
  - In-memory storage (start simple)
  - Later: Persist to file/database

- [ ] Add connection testing endpoint
  - `POST /node/test-connection` - Test RPC connectivity

**Frontend Tasks:**
- [ ] Build Node Settings UI
  - Input fields for: RPC URL, username, password
  - "Test Connection" button
  - Save/Cancel buttons
  - Show connection status

- [ ] Default local node configuration
  - Pre-fill localhost:8332
  - Clear instructions for Bitcoin Core setup

**Learning Focus:**
- Bitcoin Core RPC authentication
- How to run Bitcoin Core in different modes (regtest for development)
- Network security considerations

**Files to Create/Modify:**
- `backend/src/routes/config.rs` - Configuration endpoints
- `backend/src/state/node_config.rs` - Node config state
- `frontend/src/components/NodeSettings.tsx` - Settings UI
- `frontend/src/components/ConnectionTest.tsx` - Connection testing

---

### Phase 2: Wallet Creation & Management

#### 2.1 Wallet Creation (HD Wallet)
**Goal:** Create new Bitcoin wallets with proper key derivation

**Backend Tasks:**
- [ ] Integrate Bitcoin wallet libraries
  - Add dependencies: `bitcoin` crate, `bip39` for mnemonics
  - Add `rust-secp256k1` for cryptographic operations

- [ ] Implement wallet creation
  - Generate mnemonic seed phrases (12/24 words)
  - Derive HD wallet keys (BIP32/BIP44)
  - Support multiple derivation paths
  - Generate receiving addresses (BIP84 - native segwit)

- [ ] Wallet storage
  - Encrypt wallet data (password-based)
  - Store in local file or database
  - Secure key management practices

- [ ] Create wallet endpoints
  - `POST /wallet/create` - Create new wallet
  - `POST /wallet/import` - Import from mnemonic
  - `GET /wallet/list` - List all wallets

**Frontend Tasks:**
- [ ] Wallet creation wizard
  - Step 1: Generate/Import mnemonic
  - Step 2: Verify mnemonic (write it down)
  - Step 3: Set wallet password
  - Step 4: Confirm creation

- [ ] Mnemonic display component
  - Show 12/24 words clearly
  - Copy to clipboard functionality
  - Security warnings about backup

- [ ] Wallet selection UI
  - List available wallets
  - Show wallet metadata (name, creation date, balance)
  - Active wallet indicator

**Learning Focus:**
- BIP39 mnemonic generation
- BIP32 hierarchical deterministic wallets
- BIP44 derivation paths
- BIP84 native segwit addresses
- Wallet security and backup best practices

**Key Rust Crates to Add:**
```toml
bitcoin = "0.31"
bip39 = "2.0"
secp256k1 = "0.28"
rand = "0.8"
aes-gcm = "0.10"  # For encryption
```

**Files to Create:**
- `backend/src/services/wallet.rs` - Wallet service
- `backend/src/domain/wallet.rs` - Wallet domain models
- `backend/src/routes/wallet.rs` - Wallet endpoints
- `backend/src/crypto/encryption.rs` - Encryption utilities
- `frontend/src/components/wallet/CreateWallet.tsx`
- `frontend/src/components/wallet/WalletList.tsx`
- `frontend/src/components/wallet/MnemonicDisplay.tsx`

---

#### 2.2 Address Management
**Goal:** Generate and manage receiving addresses

**Backend Tasks:**
- [ ] Address derivation
  - Generate addresses from HD wallet
  - Track address index (gap limit: 20)
  - Support different address types (P2WPKH, P2SH-P2WPKH, P2PKH)

- [ ] Address endpoints
  - `GET /wallet/{id}/addresses` - List all addresses
  - `POST /wallet/{id}/addresses/new` - Generate new address
  - `GET /wallet/{id}/addresses/unused` - Get unused addresses

**Frontend Tasks:**
- [ ] Address list component
  - Display all generated addresses
  - Show address type and derivation path
  - Copy address button
  - QR code generation
  - Address labels/notes

- [ ] New address button
  - Generate new receiving address
  - Show address immediately
  - Indicate which addresses have been used

**Learning Focus:**
- Address types (legacy, nested segwit, native segwit)
- HD wallet gap limit concept
- Address reuse privacy concerns

**Files to Create:**
- `backend/src/services/address.rs` - Address service
- `frontend/src/components/wallet/AddressList.tsx`
- `frontend/src/components/wallet/AddressCard.tsx`
- `frontend/src/components/wallet/QRCode.tsx`

---

#### 2.3 Balance & UTXO Tracking
**Goal:** Display wallet balance and manage UTXOs

**Backend Tasks:**
- [ ] Integrate with Bitcoin node
  - Use `scantxoutset` or `importdescriptors` for wallet tracking
  - Track UTXOs for wallet addresses
  - Calculate confirmed/unconfirmed balances

- [ ] UTXO management
  - List all UTXOs for wallet
  - Track spent/unspent status
  - Calculate total balance

- [ ] Balance endpoints
  - `GET /wallet/{id}/balance` - Get wallet balance
  - `GET /wallet/{id}/utxos` - List UTXOs
  - `GET /wallet/{id}/transactions` - Transaction history

**Frontend Tasks:**
- [ ] Balance display
  - Show confirmed balance (BTC and satoshis)
  - Show unconfirmed/pending balance
  - USD/fiat conversion (optional)

- [ ] UTXO viewer
  - List all UTXOs
  - Show amount, confirmations, address
  - UTXO selection for coin control

**Learning Focus:**
- UTXO model (vs account model)
- Transaction confirmations
- Coin selection strategies

**Files to Create:**
- `backend/src/services/balance.rs` - Balance tracking
- `backend/src/domain/utxo.rs` - UTXO models
- `frontend/src/components/wallet/Balance.tsx`
- `frontend/src/components/wallet/UTXOList.tsx`

---

### Phase 3: Send & Receive Bitcoin

#### 3.1 Receive Bitcoin
**Goal:** Receive bitcoin to wallet addresses

**Frontend Tasks:**
- [ ] Receive screen
  - Display current receiving address
  - Show QR code
  - Amount input (generate payment request)
  - Share/copy address functionality

- [ ] Transaction monitoring
  - Watch for incoming transactions
  - Show pending/confirming transactions
  - Notification when payment received

**Backend Tasks:**
- [ ] Transaction monitoring
  - Poll for new transactions
  - Update UTXO set
  - Track confirmation count

- [ ] Endpoints
  - `GET /wallet/{id}/receive` - Get receiving address with QR
  - `GET /wallet/{id}/transactions?type=incoming`

**Learning Focus:**
- Bitcoin transaction lifecycle
- Confirmation requirements
- Double-spend prevention

**Files to Create:**
- `frontend/src/components/wallet/Receive.tsx`
- `frontend/src/components/wallet/TransactionList.tsx`
- `backend/src/services/transaction_monitor.rs`

---

#### 3.2 Send Bitcoin
**Goal:** Send bitcoin from wallet to other addresses

**Backend Tasks:**
- [ ] Transaction building
  - UTXO selection (coin selection algorithms)
  - Fee estimation (using `estimatesmartfee`)
  - Transaction construction
  - Change address generation

- [ ] Transaction signing
  - Sign inputs with private keys
  - Support for different signature types (SIGHASH_ALL, etc.)
  - Validate transaction before broadcast

- [ ] Transaction broadcast
  - Use `sendrawtransaction` RPC
  - Error handling for broadcast failures
  - Transaction ID tracking

- [ ] Send endpoints
  - `POST /wallet/{id}/send` - Create and broadcast transaction
  - `POST /wallet/{id}/estimate-fee` - Estimate transaction fee
  - `POST /wallet/{id}/build-transaction` - Build unsigned transaction

**Frontend Tasks:**
- [ ] Send form
  - Recipient address input (with validation)
  - Amount input (BTC/satoshi toggle)
  - Fee selection (slow/medium/fast or custom)
  - Review screen before sending

- [ ] Transaction confirmation
  - Show transaction details
  - Require password/confirmation
  - Display transaction ID after broadcast
  - Link to block explorer

- [ ] Fee estimation UI
  - Show fee rates (sat/vB)
  - Display total fee in BTC and fiat
  - ETA for confirmation

**Learning Focus:**
- Bitcoin transaction structure (inputs, outputs)
- Fee calculation and estimation
- Transaction signing (ECDSA)
- Coin selection strategies (random, FIFO, largest-first, etc.)
- Change addresses
- Transaction malleability and SegWit

**Key Considerations:**
- Input validation (prevent sending to invalid addresses)
- Dust limit checks
- RBF (Replace-By-Fee) support (future)
- CPFP (Child-Pays-For-Parent) support (future)

**Files to Create:**
- `backend/src/services/transaction_builder.rs` - Transaction construction
- `backend/src/services/fee_estimator.rs` - Fee estimation
- `backend/src/services/signer.rs` - Transaction signing
- `backend/src/routes/send.rs` - Send endpoints
- `frontend/src/components/wallet/Send.tsx`
- `frontend/src/components/wallet/SendReview.tsx`
- `frontend/src/components/wallet/FeeSelector.tsx`

---

### Phase 4: Transaction History & Details

#### 4.1 Transaction History
**Goal:** Display complete transaction history for wallet

**Backend Tasks:**
- [ ] Transaction indexing
  - Store transaction history in database
  - Track sent/received amounts
  - Link transactions to addresses

- [ ] Transaction details
  - Parse transaction inputs and outputs
  - Calculate fees
  - Determine transaction type (sent/received/self)

- [ ] Endpoints
  - `GET /wallet/{id}/transactions` - Paginated history
  - `GET /wallet/{id}/transactions/{txid}` - Transaction details

**Frontend Tasks:**
- [ ] Transaction list
  - Show date, amount, type (sent/received)
  - Confirmation status
  - Filter by type, date range
  - Pagination

- [ ] Transaction detail view
  - Show all inputs and outputs
  - Display fee, size, confirmations
  - Link to block explorer
  - Transaction hex viewer

**Learning Focus:**
- Transaction structure deep dive
- Block explorers and their APIs
- Transaction analysis

**Files to Create:**
- `backend/src/services/transaction_history.rs`
- `frontend/src/components/wallet/TransactionHistory.tsx`
- `frontend/src/components/wallet/TransactionDetail.tsx`

---

### Phase 5: Advanced Features

#### 5.1 Multi-Wallet Support
- [ ] Switch between multiple wallets
- [ ] Wallet import/export
- [ ] Wallet deletion with confirmation

#### 5.2 Advanced Transaction Features
- [ ] Replace-By-Fee (RBF)
- [ ] Child-Pays-For-Parent (CPFP)
- [ ] Coin control (manual UTXO selection)
- [ ] Batch transactions

#### 5.3 Security Enhancements
- [ ] Two-factor authentication
- [ ] Spending limits
- [ ] Address whitelisting
- [ ] Transaction approval workflow

#### 5.4 Network & Mempool Insights
- [ ] Mempool visualization
- [ ] Fee market analysis
- [ ] Network hash rate
- [ ] Peer information

#### 5.5 Privacy Features
- [ ] CoinJoin integration (future)
- [ ] Tor support
- [ ] Address labeling
- [ ] Coin control for privacy

---

## 🎯 Fedimint Preparation Roadmap

**Skills to Develop Through This Project:**

1. **Bitcoin Fundamentals** ✓ (Covered in Phases 1-4)
   - UTXO model
   - Transaction structure
   - Signatures and cryptography
   - HD wallets and key derivation

2. **Rust Proficiency** ✓ (Ongoing)
   - Async programming (Tokio)
   - Error handling
   - Type system mastery
   - Cryptographic libraries

3. **Additional Topics for Fedimint:**
   - [ ] Lightning Network basics
   - [ ] Schnorr signatures and Taproot
   - [ ] Threshold signatures (Fedimint core)
   - [ ] Byzantine fault tolerance
   - [ ] Consensus mechanisms

**Next Steps After This Project:**
1. Study Fedimint architecture documentation
2. Run a Fedimint federation locally
3. Contribute to Fedimint docs/tests
4. Tackle good first issues
5. Engage with Fedimint community

---

## 📚 Learning Resources

### Bitcoin Fundamentals
- **Mastering Bitcoin** by Andreas Antonopoulos (free online)
- **Programming Bitcoin** by Jimmy Song
- Bitcoin Developer Guide: https://developer.bitcoin.org/
- BIPs (Bitcoin Improvement Proposals): https://github.com/bitcoin/bips

### Rust & Bitcoin Development
- **Programming Rust** by Jim Blandy (O'Reilly)
- Rust Bitcoin library docs: https://docs.rs/bitcoin/
- Bitcoin Core RPC docs: https://developer.bitcoin.org/reference/rpc/

### Cryptography
- **Understanding Cryptography** by Christof Paar
- BIP32, BIP39, BIP44, BIP84 specifications

### Fedimint Specific
- Fedimint docs: https://fedimint.org/docs/
- Fedimint GitHub: https://github.com/fedimint/fedimint
- Fedimint Discord community

---

## 🔧 Development Setup Guide

### Bitcoin Core Setup (Required)
```bash
# Install Bitcoin Core
# Download from https://bitcoin.org/en/download

# Run in regtest mode (for development)
bitcoind -regtest -server -rpcuser=youruser -rpcpassword=yourpass

# Or use bitcoin.conf
# Create ~/.bitcoin/bitcoin.conf:
regtest=1
server=1
rpcuser=youruser
rpcpassword=yourpass
rpcallowip=127.0.0.1
```

### Backend Setup
```bash
cd backend
cp .env.example .env
# Edit .env with your Bitcoin RPC credentials
cargo run
```

### Frontend Setup
```bash
cd frontend
npm install
npm run dev
```

---

## 📊 Project Metrics

**Current Progress:** ~15% Complete

**Completed:**
- ✅ Backend infrastructure (Rust/Axum)
- ✅ Bitcoin RPC client foundation
- ✅ Basic node info endpoints
- ✅ Frontend skeleton (Next.js)

**Next Milestone:** Phase 1 Complete (Basic UI Foundation)
- Estimated effort: 2-3 coding sessions
- Focus: Node dashboard and connection management

**To Bitcoin Wallet Functionality:** Phase 2 Complete
- Estimated effort: 6-8 coding sessions
- Focus: Wallet creation, addresses, balance tracking

**To Full Send/Receive:** Phase 3 Complete
- Estimated effort: 4-5 additional sessions
- Focus: Transaction building and broadcasting

---

## 🐛 Known Issues & Technical Debt

- [ ] No database layer (using in-memory state)
- [ ] No logging/monitoring
- [ ] No unit tests
- [ ] No integration tests
- [ ] Frontend API client not implemented
- [ ] No error boundary in React app
- [ ] No environment-specific configs
- [ ] Backend port conflicts with frontend (both use 3000)
- [ ] No CI/CD pipeline

---

## 🤝 Contributing to Your Own Learning

### Daily/Weekly Goals
- **Week 1-2:** Complete Phase 1 (Node UI)
- **Week 3-5:** Complete Phase 2.1 (Wallet Creation)
- **Week 6-7:** Complete Phase 2.2-2.3 (Addresses & Balance)
- **Week 8-10:** Complete Phase 3 (Send/Receive)

### Study Routine Suggestion
1. Read relevant BIP before implementing feature
2. Study existing implementations (electrum, bitcoin-cli)
3. Implement feature in Rust
4. Write tests
5. Build UI
6. Document learnings in separate notes

### Code Review Checklist
- [ ] Does this match Bitcoin best practices?
- [ ] Is the cryptography implementation secure?
- [ ] Are private keys handled safely?
- [ ] Is error handling comprehensive?
- [ ] Would this code help me understand Fedimint better?

---

## 🎓 Key Concepts to Master

### Phase 1
- [x] JSON-RPC protocol
- [x] Bitcoin Core RPC API
- [ ] React state management
- [ ] TypeScript type safety

### Phase 2
- [ ] BIP39 (Mnemonic codes)
- [ ] BIP32 (HD wallets)
- [ ] BIP44 (Multi-account hierarchy)
- [ ] BIP84 (Derivation for P2WPKH)
- [ ] Encryption and key storage

### Phase 3
- [ ] UTXO selection algorithms
- [ ] Fee estimation strategies
- [ ] Transaction structure (witness data, segregated witness)
- [ ] ECDSA signatures
- [ ] Transaction broadcasting

### Phase 4
- [ ] Block explorers and indexing
- [ ] Transaction graph analysis
- [ ] Database design for blockchain data

---

## 💡 Tips for Success

1. **Start with Regtest:** Use Bitcoin Core's regtest mode for instant block generation and testing without real money

2. **Use Bitcoin Core as Reference:** When in doubt, check how Bitcoin Core implements a feature

3. **Security First:** Even in a learning project, practice secure key management

4. **Document Everything:** Write down what you learn about Bitcoin - it'll help with Fedimint

5. **Test Incrementally:** Don't build large features without testing smaller pieces

6. **Ask Questions:** Bitcoin and Fedimint communities are helpful - engage on Discord/IRC

7. **Read BIPs Thoroughly:** Understanding the "why" behind Bitcoin design decisions is crucial for Fedimint

---

## 🚀 Quick Start Commands

```bash
# Start Bitcoin Core (regtest)
bitcoind -regtest -daemon

# Generate blocks (regtest only)
bitcoin-cli -regtest generatetoaddress 101 <address>

# Start backend
cd backend && cargo run

# Start frontend
cd frontend && npm run dev

# Run both (from root)
npm run dev
```

---

**Last Updated:** 2025-12-22
**Status:** Active Development
**Next Review:** After Phase 1 completion
