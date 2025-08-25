# Current Architecture: VetKeys Encrypted Notes Application

## Overview
This is an ICP (Internet Computer Protocol) Rust-based application that implements encrypted note storage using vetKeys technology. The application demonstrates secure, decentralized note management with Internet Identity authentication.

## Architecture Components

### 1. Backend Canister (Rust)
**Location**: `backend/src/lib.rs`

#### Core Data Structures
```rust
pub struct EncryptedNote {
    id: NoteId,                    // u128 unique identifier
    encrypted_text: String,        // AES-GCM encrypted content
    owner: PrincipalName,          // Principal ID as string
    users: Vec<PrincipalName>,     // Shared users (excluding owner)
}
```

#### Storage Architecture
- **Stable Memory**: Uses IC stable structures for persistence
- **Memory Management**: 4 separate memory regions:
  - `MemoryId(0)`: Next note ID counter
  - `MemoryId(1)`: Notes storage (BTreeMap<NoteId, EncryptedNote>)
  - `MemoryId(2)`: Owner to note IDs mapping
  - `MemoryId(3)`: Shared user to note IDs mapping

#### Key Functions
- `create_note()` → Creates empty note, returns ID
- `get_notes()` → Returns all notes for caller (owned + shared)
- `update_note(id, encrypted_text)` → Updates note content
- `add_user(note_id, user)` → Shares note with user
- `remove_user(note_id, user)` → Unshares note
- `delete_note(note_id)` → Deletes note (owner only)

#### VetKeys Integration
- `symmetric_key_verification_key_for_note()` → Returns public key for verification
- `encrypted_symmetric_key_for_note(note_id, transport_key)` → Returns encrypted key for specific note
- Uses BLS12-381 G2 curve with test key "test_key_1"

#### Security Model
- **Authentication**: Rejects anonymous principals
- **Authorization**: Owner-based permissions with sharing capabilities
- **Limits**: Max 1,000 users, 500 notes per user, 1,000 chars per note, 50 shares per note

### 2. Frontend Application (Svelte + TypeScript)
**Location**: `frontend/src/`

#### Authentication Flow (`store/auth.ts`)
```typescript
type AuthState = 
  | "initializing-auth"
  | "anonymous" 
  | "initializing-crypto"
  | "synchronizing"
  | "initialized"
  | "error"
```

**Process**:
1. Initialize AuthClient (Internet Identity)
2. Check existing authentication
3. Create backend actor with identity
4. Initialize CryptoService
5. Set up session timeout handling

#### Encryption Service (`lib/crypto.ts`)
- **Algorithm**: AES-GCM with 12-byte IV
- **Key Derivation**: Uses vetKeys for note-specific keys
- **Storage**: IndexedDB via idb-keyval for client-side key caching
- **Process**:
  1. Fetch transport secret key
  2. Request encrypted vetKey from backend
  3. Decrypt and verify with derived public key
  4. Derive AES-GCM key for note encryption

#### Data Flow (`store/notes.ts`)
1. **Loading**: Polls backend every 3 seconds for notes
2. **Decryption**: Filters empty notes, decrypts content
3. **Creation**: Creates note → gets ID → encrypts → updates backend
4. **Updates**: Encrypts content → calls backend update

#### UI Components
- **Hero.svelte**: Landing page with login
- **LayoutAuthenticated.svelte**: Main app layout
- **Notes.svelte**: Note list display
- **NoteEditor.svelte**: Note editing interface
- **SharingEditor.svelte**: User sharing management

### 3. Identity & Authentication System

#### Internet Identity Integration
- **Provider**: Local development uses `rdmx6-jaaaa-aaaaa-aaadq-cai.localhost:8000`
- **Session**: 30-minute timeout (1800 seconds)
- **Delegation**: Handles delegation chain expiry
- **Storage**: Uses localStorage for session persistence

#### Principal Management
- **Format**: IC Principal IDs (e.g., "mpuup-jpwyr-ib3b5-i4asn-wc3qd-ftsxv-qcol2-bqqdr-qofpd-7ogeb-yae")
- **Usage**: String representation for storage and comparison
- **Authorization**: Principal-based access control

### 4. Cryptographic Architecture

#### VetKeys Implementation
- **Purpose**: Threshold cryptography for secure key derivation
- **Curve**: BLS12-381 G2
- **Context**: "note_symmetric_key" for all operations
- **Key Derivation Input**: `note_id (16 bytes) + owner_principal (UTF-8)`

#### Encryption Flow
```
1. User creates note → Backend assigns ID
2. Frontend requests vetKey for note_id + owner
3. Backend derives encrypted key using vetKeys
4. Frontend decrypts vetKey using transport key
5. Frontend derives AES-GCM key from vetKey
6. Content encrypted with note-specific key
7. Encrypted content stored in backend
```

#### Security Properties
- **Forward Secrecy**: Each note has unique encryption key
- **Access Control**: Only authorized users can decrypt
- **Threshold Security**: vetKeys provides distributed key generation
- **Client-Side Encryption**: Content never stored unencrypted on backend

### 5. Development Environment

#### Build System
- **Backend**: Cargo workspace with WASM target
- **Frontend**: Rollup + TypeScript + Svelte
- **Deployment**: dfx for local development
- **Dependencies**: 
  - `@dfinity/vetkeys` for cryptography
  - `@dfinity/auth-client` for authentication
  - `@dfinity/agent` for IC communication

#### Configuration
- **dfx.json**: Defines canisters and build process
- **Cargo.toml**: Rust dependencies and workspace
- **package.json**: Frontend dependencies and scripts

## Current Limitations

### Scalability
- Fixed limits on users and notes
- Single canister architecture
- No horizontal scaling mechanism

### User Experience
- Manual sharing by principal ID
- No user discovery mechanism
- Limited note organization features

### Security
- Test keys in development
- No key rotation mechanism
- Limited audit trail

### Identity Management
- Relies solely on Internet Identity
- No profile or metadata storage
- No identity verification beyond IC authentication

## Technical Debt

### Code Quality
- TypeScript compilation warnings
- Missing error handling in some paths
- No comprehensive testing suite

### Architecture
- Tight coupling between UI and crypto operations
- No abstraction layer for different identity providers
- Hard-coded configuration values

### Documentation
- Limited inline documentation
- No API documentation
- Missing deployment guides for production

## Transformation Opportunities for AI Agent Passport Identity MVP

### Core Identity Infrastructure
The current principal-based authentication and vetKeys encryption provide a solid foundation for AI agent identity management.

### Verifiable Credentials
The note-sharing mechanism could be extended to credential issuance and verification.

### Decentralized Storage
The stable memory architecture could store agent profiles, capabilities, and attestations.

### Cryptographic Primitives
The vetKeys integration provides the cryptographic foundation for secure agent-to-agent communication and credential verification.
