# TODO: Transforming VetKeys Encrypted Notes App into POC Control Plane for Agentic Identities on ICP

This TODO list outlines the current state of implementation based on the existing VetKeys Encrypted Notes Application architecture. It identifies what's already implemented (leveraging the principal-based auth, vetKeys crypto, stable storage, and frontend flows) and what needs to be added or modified to build a Proof-of-Concept (POC) Control Plane for Agentic Identities.

This POC aims to mimic features like agent discovery, verifiable passports (identities), secure connections, monitoring, governance, and cross-app integration (e.g., Discord summoning), as discussed in prior Cyata copycat plans.

**The transformation focuses on:**
- Extending notes to store agent "passports" (verifiable credentials with principals, capabilities, and encrypted data)
- Adding Cyata-inspired controls: Identification, monitoring (audit logs), governance (policies), and secure agent-to-passport binding
- Maintaining ICP strengths: Decentralized persistence, vetKeys for secure key derivation, and Internet Identity (II) for auth

## 1. Core Identity Infrastructure ✅ (Partially Complete)

*This builds on the existing principal management and II integration to create "digital passports" for AI agents.*

### Already Implemented ✅
- [x] Principal-based authentication: Rejects anonymous callers; uses `caller()` for access control
- [x] Internet Identity (II) integration: Handles delegation chains, session timeouts (30 mins), and localStorage persistence
- [x] Principal string representation: Stored and compared for ownership (e.g., in `EncryptedNote.owner`)

### To Implement 🔲
- [ ] **Implement passport creation**: Extend `create_note` to generate a "passport note" type with agent-specific fields (e.g., capabilities, encrypted creds for tools like YouTube)
- [ ] **Secure agent-to-passport binding**: Add verification in canister methods to ensure `caller()` matches passport's principal before actions (prevents rogue access)
- [ ] **Add multi-agent support**: Allow passports to reference shared "swarm" notes for agent coordination

## 2. Verifiable Credentials ✅ (Partially Complete)

*Repurpose the note-sharing mechanism for issuing/verifying agent credentials (e.g., passports as verifiable, encrypted notes).*

### Already Implemented ✅
- [x] Note sharing: `add_user`/`remove_user` allows principal-based sharing (up to 50 shares per note)
- [x] Owner permissions: Only owners can delete/update; shared users have read access
- [x] Limits enforced: Max 1,000 users, 500 notes per user, etc., for basic governance

### To Implement 🔲
- [ ] **Extend notes to credentials**: Add a `CredentialNote` variant with fields for attestations (e.g., agent reputation, capabilities)
- [ ] **Credential verification flow**: New method to verify passport signatures using vetKeys (e.g., check if credential note is signed by trusted principal)
- [ ] **Integrate ZK-proofs (optional for MVP)**: Use ICP's crypto libs for simple proofs of credential ownership without revealing details

## 3. Decentralized Storage ✅ (Partially Complete)

*Leverage stable memory for persistent agent data (memory, passports, logs).*

### Already Implemented ✅
- [x] Stable memory setup: 4 memory regions for counters, notes, owner mappings, shared mappings
- [x] Persistent storage: BTreeMap for notes; survives upgrades
- [x] Data limits: 1,000 chars per note; scalable within canister

### To Implement 🔲
- [ ] **Add agent memory storage**: Implement vector embeddings in new memory region (e.g., `MemoryId(4)` for `StableVec<VectorEntry>` with ID, embedding, encrypted content)
- [ ] **Passport storage**: Store passports as special notes with encrypted creds (e.g., API keys for cross-tools)
- [ ] **Audit log storage**: New stable vec for action logs (principal, action, timestamp, intention) in `MemoryId(5)`

## 4. Cryptographic Primitives ✅ (Mostly Complete)

*Build on vetKeys for secure key derivation in agent contexts.*

### Already Implemented ✅
- [x] VetKeys integration: Symmetric key derivation with BLS12-381 G2 curve; `encrypted_symmetric_key_for_note` returns encrypted key
- [x] AES-GCM encryption: Client-side with 12-byte IV; keys derived per note
- [x] Key caching: Frontend uses IndexedDB for transport keys
- [x] Forward secrecy: Unique keys per note

### To Implement 🔲
- [ ] **Agent-specific keys**: Modify derivation input to include agent principal + passport ID for secure connections
- [ ] **Secure connection enforcement**: In canister, decrypt/verify only if caller principal matches passport owner
- [ ] **Add signing for passports**: Use vetKeys to sign passport credentials for verifiability across apps

## 5. Monitoring and Observability (Cyata-like) 🔲 (Not Started)

*Add real-time tracking and audit trails for agent actions.*

### Already Implemented ✅
- [x] Basic logging: Implicit via canister traps and prints (e.g., for errors)

### To Implement 🔲
- [ ] **Implement action logging**: New `log_action` function to record every method call (e.g., store, retrieve) with intention param
- [ ] **Query audit trails**: New `get_audit_logs(principal)` to return logs for forensics
- [ ] **Real-time monitoring**: Add polling in frontend (every 3s like notes) for log updates in a dashboard view

## 6. Governance and Control (Cyata-like) 🔲 (Not Started)

*Enforce policies to manage agent behavior.*

### Already Implemented ✅
- [x] Basic authorization: Owner-only deletes; shared access controls
- [x] Limits as soft governance: Enforced in code (e.g., max notes)

### To Implement 🔲
- [ ] **Policy storage**: New stable map for per-principal policies (allowed_actions, shutdown_flag, rate_limit)
- [ ] **Enforce policies**: Wrapper function `enforce_policy` checked before every action
- [ ] **Admin controls**: Methods like `shutdown_agent(principal)` to block rogue agents

## 7. Frontend and Cross-App Integration ✅ (Partially Complete)

*Extend UI for agent management; add Discord bot for summoning.*

### Already Implemented ✅
- [x] Auth flow: States from "initializing" to "initialized"; handles errors
- [x] Note management UI: List, editor, sharing components
- [x] Encryption in frontend: CryptoService handles derivation and AES-GCM

### To Implement 🔲
- [ ] **Passport UI**: New components for creating/viewing passports (e.g., `PassportEditor.svelte`)
- [ ] **Cross-app**: Build Node.js Discord bot to summon agents (call `register_agent`), proxy canister calls with II delegation
- [ ] **Secure connect in integrations**: Bot/client must auth with II and bind to passport before interactions

## 8. Development and Testing ✅ (Partially Complete)

*Ensure the POC is deployable and testable.*

### Already Implemented ✅
- [x] Build system: dfx for deployment; Cargo for Rust; Rollup for frontend
- [x] Dependencies: @dfinity/vetkeys, auth-client, agent

### To Implement 🔲
- [ ] **Add tests**: Unit tests for new methods (e.g., policy enforcement) using ic-cdk-testers
- [ ] **Fix technical debt**: Add error handling, inline docs, API docs (Candid comments)
- [ ] **Deployment guide**: Write production notes (e.g., replace test keys, mainnet II provider)

## 9. Scalability and Enhancements 🔲 (Future Work)

*Address limitations for MVP viability.*

### Current State ✅
- [x] Single canister: Basic architecture in place

### To Implement 🔲
- [ ] **Sharding**: Prep for multi-canister (e.g., separate for logs vs. passports)
- [ ] **UX improvements**: Add principal discovery (e.g., via shared notes); key rotation
- [ ] **Security audits**: Manual review for new crypto flows

## Milestones for MVP Completion

### Week 1: Core Identity + Crypto
- [ ] Implement passport creation/binding and secure connections
- [ ] Agent-specific key derivation
- [ ] Secure connection enforcement

### Week 2: Monitoring + Governance
- [ ] Add monitoring logs and governance policies
- [ ] Action logging system
- [ ] Policy enforcement framework

### Week 3: Integration
- [ ] Integrate Discord bot and frontend updates
- [ ] Passport UI components
- [ ] Cross-app authentication

### Week 4: Testing & Deployment
- [ ] Testing, docs, and demo deployment
- [ ] Unit tests and integration tests
- [ ] Production deployment guide

---

**Progress Tracking**: Check off items as completed. This transforms the notes app into a Cyata-like control plane, focusing on secure, decentralized agent identities on ICP.

**Current Status**: 
- ✅ **Foundation Complete**: 40% (Identity, Storage, Crypto basics)
- 🔲 **Core Features**: 0% (Passports, Monitoring, Governance)
- 🔲 **Integration**: 0% (UI, Discord bot)
- 🔲 **Production Ready**: 0% (Testing, Docs)
