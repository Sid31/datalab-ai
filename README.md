# Synthetic Data Manager

A secure synthetic data management platform built on the Internet Computer Protocol (ICP) with encrypted storage for privacy-preserving data operations.

## What This Does

This platform allows you to securely upload, store, and manage CSV datasets with complete privacy and decentralization. Built on ICP's blockchain infrastructure, it provides:

- **🔒 Secure CSV Upload**: Upload any CSV file with automatic encryption and storage
- **📊 Dataset Management**: Browse, view, and manage your uploaded datasets
- **🎯 Sample Data Library**: Pre-loaded synthetic datasets for immediate testing
- **🔐 Privacy-First**: All data encrypted and stored on decentralized ICP canisters
- **🌐 Internet Identity**: Secure authentication without passwords or emails
- **💾 Persistent Storage**: Data survives canister upgrades using stable memory

## Key Features

- **Drag & Drop Upload**: Simple CSV file upload interface
- **Large File Support**: Handles datasets up to 1M characters (thousands of rows)
- **Real-time Preview**: See dataset structure and row counts before upload
- **Encrypted Storage**: All data encrypted before storage on ICP blockchain
- **Principal-based Access**: Only you can access your uploaded datasets
- **Sample Datasets**: Test with pre-loaded employee, financial, and IoT data

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

## Installation & Setup

### Prerequisites

Install the following tools before setting up the project:

1. **Install dfx (Internet Computer SDK)**:
   ```bash
   sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
   ```

2. **Install Rust with WebAssembly target**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add wasm32-unknown-unknown
   ```

3. **Install Node.js (v16 or higher)**:
   ```bash
   # macOS with Homebrew
   brew install node
   
   # Or download from https://nodejs.org/
   ```

4. **Install candid-extractor**:
   ```bash
   cargo install candid-extractor
   ```

### Quick Start

1. **Clone and navigate to project**:
   ```bash
   git clone <repository-url>
   cd rust-vetkeys-rust
   ```

2. **Start local ICP network**:
   ```bash
   dfx start --background
   ```

3. **Install frontend dependencies**:
   ```bash
   cd frontend
   npm install
   cd ..
   ```

4. **Deploy all canisters**:
   ```bash
   dfx deploy
   ```

5. **Access the application**:
   - Open the frontend URL shown in deploy output (e.g., `http://u6s2n-gx777-77774-qaaba-cai.localhost:8000/`)
   - Click "Login with Internet Identity"
   - Create a new Internet Identity when prompted
   - Start uploading and managing your CSV datasets!

### Troubleshooting

- **Port conflicts**: If dfx fails to start, kill existing processes: `pkill -f dfx`
- **Build errors**: Ensure Rust wasm32 target is installed: `rustup target add wasm32-unknown-unknown`
- **Internet Identity issues**: Always create a new identity after fresh deployment

### Environment Setup

The deployment automatically creates `frontend/.env` with:
```
CANISTER_ID_ENCRYPTED_NOTES=[canister-id]
DFX_NETWORK=local
```

## How to Use

### 1. Upload Your CSV Data
- **Drag & Drop**: Simply drag your CSV file onto the upload area
- **File Preview**: See row count and column headers before upload
- **Large Files**: Supports datasets up to 1M characters (thousands of rows)
- **Automatic Processing**: File is automatically parsed and encrypted

### 2. Browse Your Datasets
- **My Datasets**: View all your uploaded CSV files
- **Dataset Details**: Click any dataset to see full structure and metadata
- **Search & Filter**: Find specific datasets quickly
- **Download**: Export your data when needed

### 3. Try Sample Data
- **One-Click Loading**: Test with pre-loaded synthetic datasets
- **Various Types**: Employee data, financial records, IoT sensor data
- **Instant Access**: No upload required, ready to explore immediately

### Privacy & Security Features
- **Encrypted Storage**: All data encrypted before storage on ICP blockchain
- **Principal-based Access**: Only you can access your uploaded datasets
- **No Passwords**: Secure authentication via Internet Identity
- **Decentralized**: Data stored on ICP network, not centralized servers

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
