# Synthetic Data Manager

A secure synthetic data management platform built on the Internet Computer Protocol (ICP) using vetKeys cryptography for privacy-preserving data operations.

## Overview

This platform provides a decentralized solution for managing synthetic datasets with enterprise-grade security and privacy controls. Key features include:

- **Secure Data Upload**: CSV file upload with client-side encryption
- **Privacy-First Architecture**: vetKeys threshold cryptography for data protection
- **Sample Dataset Library**: Pre-configured synthetic datasets for testing and development
- **Access Control**: Principal-based authentication via Internet Identity
- **Data Masking**: Built-in privacy controls for sensitive synthetic data

## Architecture

### Backend (Rust Canister)
- **Identity Management**: Principal-based access control with Internet Identity integration
- **Cryptographic Security**: vetKeys BLS12-381 G2 curve for secure key derivation
- **Persistent Storage**: IC stable memory with 96GB capacity for synthetic datasets
- **Encrypted Storage**: Client-side encrypted dataset storage with sharing capabilities

### Frontend (Svelte + TypeScript)
- **Authentication Flow**: Internet Identity integration with session management
- **Encryption Service**: Client-side AES-GCM with vetKeys key derivation
- **Dataset Management UI**: CSV upload, sample data loading, and dataset browsing
- **Privacy Controls**: Data masking and access management interfaces

## Current Status

**Core Features Complete**:
- ✅ Principal-based authentication with Internet Identity
- ✅ CSV file upload with drag-and-drop interface
- ✅ Sample synthetic dataset library
- ✅ Client-side encryption using vetKeys
- ✅ Dataset browsing and management UI
- ✅ Privacy-preserving data storage

**Available Sample Datasets**:
- 📊 Customer Demographics (1,000 rows)
- 💰 Financial Transactions (2,500 rows)
- 🏥 Healthcare Records (750 rows)
- 🔧 IoT Sensor Data (5,000 rows)

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
   - Login with Internet Identity to upload and manage synthetic datasets
   - Backend Candid UI: Available via dfx deploy output

### Environment Setup

The deployment automatically creates `frontend/.env` with:
```
CANISTER_ID_ENCRYPTED_NOTES=[canister-id]
DFX_NETWORK=local
```

## Features

### Data Upload & Management
- **CSV Upload**: Drag-and-drop interface for uploading CSV files
- **Sample Data**: Pre-loaded synthetic datasets for immediate testing
- **Dataset Browser**: View and manage your uploaded datasets
- **Column Preview**: See dataset structure and column information

### Privacy & Security
- **Client-Side Encryption**: All data encrypted before upload
- **Access Controls**: Principal-based dataset ownership
- **Data Masking**: Privacy controls for sensitive information
- **Secure Storage**: vetKeys cryptography for data protection

## Technology Stack

- **Backend**: Rust, IC CDK, vetKeys, Stable Structures
- **Frontend**: Svelte, TypeScript, Tailwind CSS, DaisyUI
- **Cryptography**: vetKeys (BLS12-381 G2), AES-GCM encryption
- **Authentication**: Internet Identity, Principal-based access control
- **Storage**: IC Stable Memory (persistent across upgrades)

## Security Features

- **Threshold Cryptography**: vetKeys for distributed key generation
- **Forward Secrecy**: Unique encryption keys per dataset
- **Access Control**: Principal-based authorization with sharing capabilities
- **Client-Side Encryption**: Dataset content never stored unencrypted on backend
- **Session Management**: 30-minute timeout with delegation chain handling
- **Privacy-First**: Data masking and anonymization controls

## Use Cases

- **Data Scientists**: Securely share synthetic datasets for research
- **ML Engineers**: Test models with privacy-preserving synthetic data
- **Developers**: Access sample datasets for application development
- **Organizations**: Manage synthetic data with enterprise-grade security

## License

[Add license information]

---

**Built on Internet Computer Protocol** - Leveraging ICP's decentralized infrastructure for secure, scalable synthetic data management.
