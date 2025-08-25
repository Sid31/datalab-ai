# Passeport - AI Agent Identity Control Plane

A decentralized identity and passport system for AI agents built on the Internet Computer Protocol (ICP) using vetKeys cryptography.

## Overview

This project transforms a vetKeys encrypted notes application into a Proof-of-Concept (POC) Control Plane for Agentic Identities. It provides secure, verifiable digital passports for AI agents with features like:

- **Decentralized Identity**: Principal-based authentication via Internet Identity
- **Verifiable Credentials**: Agent passports with encrypted capabilities and attestations
- **Secure Communication**: vetKeys threshold cryptography for agent-to-agent connections
- **Monitoring & Governance**: Audit trails and policy enforcement for agent behavior
- **Cross-App Integration**: Discord bot summoning and multi-platform agent coordination

## Architecture

### Backend (Rust Canister)
- **Identity Management**: Principal-based access control with Internet Identity integration
- **Cryptographic Security**: vetKeys BLS12-381 G2 curve for secure key derivation
- **Persistent Storage**: IC stable memory with 96GB capacity for agent data
- **Verifiable Credentials**: Encrypted passport storage with sharing capabilities

### Frontend (Svelte + TypeScript)
- **Authentication Flow**: Internet Identity integration with session management
- **Encryption Service**: Client-side AES-GCM with vetKeys key derivation
- **Agent Management UI**: Passport creation, editing, and sharing interfaces
- **Real-time Monitoring**: Dashboard for agent activity and audit logs

## Current Status

**Foundation Complete (40%)**:
- ✅ Principal-based authentication
- ✅ vetKeys cryptographic integration
- ✅ Stable memory storage architecture
- ✅ Internet Identity authentication flow

**In Development**:
- 🔲 Agent passport creation and management
- 🔲 Audit logging and monitoring system
- 🔲 Policy governance framework
- 🔲 Discord bot integration
- 🔲 Cross-app authentication protocols

## Quick Start

### Prerequisites
- [dfx](https://internetcomputer.org/docs/current/developer-docs/setup/install/) (ICP SDK)
- [Rust](https://rustup.rs/) with wasm32-unknown-unknown target
- [Node.js](https://nodejs.org/) (v16+)
- [candid-extractor](https://crates.io/crates/candid-extractor)

### Local Development

1. **Start the local replica**:
   ```bash
   dfx start --background
   ```

2. **Install frontend dependencies**:
   ```bash
   cd frontend && npm install
   ```

3. **Deploy canisters**:
   ```bash
   dfx deploy
   ```

4. **Access the application**:
   - Frontend: `http://[canister-id].localhost:8000/`
   - Backend Candid UI: Available via dfx deploy output

### Environment Setup

The deployment automatically creates `frontend/.env` with:
```
CANISTER_ID_ENCRYPTED_NOTES=[canister-id]
DFX_NETWORK=local
```

## Development Roadmap

See [todo.md](./todo.md) for detailed implementation plan and [currint.md](./currint.md) for complete architecture documentation.

### Milestones
- **Week 1**: Core Identity + Crypto (passport creation, secure connections)
- **Week 2**: Monitoring + Governance (audit logs, policy enforcement)
- **Week 3**: Integration (Discord bot, passport UI)
- **Week 4**: Testing & Deployment (production readiness)

## Technology Stack

- **Backend**: Rust, IC CDK, vetKeys, Stable Structures
- **Frontend**: Svelte, TypeScript, Tailwind CSS, DaisyUI
- **Cryptography**: vetKeys (BLS12-381 G2), AES-GCM encryption
- **Authentication**: Internet Identity, Principal-based access control
- **Storage**: IC Stable Memory (persistent across upgrades)

## Security Features

- **Threshold Cryptography**: vetKeys for distributed key generation
- **Forward Secrecy**: Unique encryption keys per agent passport
- **Access Control**: Principal-based authorization with sharing capabilities
- **Client-Side Encryption**: Content never stored unencrypted on backend
- **Session Management**: 30-minute timeout with delegation chain handling

## Contributing

This is a POC for AI agent identity management. See the transformation plan in `todo.md` for current development priorities.

## License

[Add license information]

---

**Built on Internet Computer Protocol** - Leveraging ICP's decentralized infrastructure for secure, scalable AI agent identity management.
