use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk::api::msg_caller;
use ic_cdk::update;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{
    storable::Bound, DefaultMemoryImpl, StableBTreeMap, StableCell, Storable,
};
use std::borrow::Cow;
use std::cell::RefCell;

type PrincipalName = String;
type Memory = VirtualMemory<DefaultMemoryImpl>;
type NoteId = u128;
type PassportId = u128;
type AgentMemoryId = u128;
type ApiTokenId = u128;

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct EncryptedNote {
    id: NoteId,
    encrypted_text: String,
    owner: PrincipalName,
    /// Principals with whom this note is shared. Does not include the owner.
    /// Needed to be able to efficiently show in the UI with whom this note is shared.
    users: Vec<PrincipalName>,
}

impl EncryptedNote {
    pub fn is_authorized(&self, user: &PrincipalName) -> bool {
        user == &self.owner || self.users.contains(user)
    }
}

impl Storable for EncryptedNote {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct AgentPassport {
    id: PassportId,
    agent_name: String,
    agent_type: String, // "eliza", "custom", etc.
    owner: PrincipalName,
    capabilities: Vec<String>,
    encrypted_specifications: String, // Encrypted agent config/specs
    api_endpoints: Vec<String>, // Allowed API access points
    created_at: u64, // Timestamp in nanoseconds
    last_active: u64,
    is_active: bool,
}

impl AgentPassport {
    pub fn is_authorized(&self, user: &PrincipalName) -> bool {
        user == &self.owner
    }
}

impl Storable for AgentPassport {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct AgentMemory {
    id: AgentMemoryId,
    passport_id: PassportId,
    memory_type: String, // "conversation", "preference", "skill", etc.
    encrypted_content: String,
    importance_score: u8, // 0-100 relevance score
    created_at: u64,
    owner: PrincipalName,
}

impl AgentMemory {
    pub fn is_authorized(&self, user: &PrincipalName) -> bool {
        user == &self.owner
    }
}

impl Storable for AgentMemory {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

#[derive(Clone, Debug, CandidType, Deserialize, Eq, PartialEq)]
pub struct ApiToken {
    id: ApiTokenId,
    passport_id: PassportId,
    token_hash: String, // SHA-256 hash of the actual token
    name: String, // Human-readable name for the token
    permissions: Vec<String>, // List of allowed operations
    expires_at: Option<u64>, // Optional expiration timestamp
    created_at: u64,
    last_used: Option<u64>,
    is_active: bool,
    owner: PrincipalName,
}

impl ApiToken {
    pub fn is_authorized(&self, user: &PrincipalName) -> bool {
        user == &self.owner
    }

    pub fn is_valid(&self) -> bool {
        if !self.is_active {
            return false;
        }
        
        if let Some(expires_at) = self.expires_at {
            let current_time = ic_cdk::api::time();
            if current_time > expires_at {
                return false;
            }
        }
        
        true
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&permission.to_string()) || 
        self.permissions.contains(&"*".to_string()) // Wildcard permission
    }
}

impl Storable for ApiToken {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize, Default)]
pub struct NoteIds {
    ids: Vec<NoteId>,
}

impl NoteIds {
    pub fn iter(&self) -> impl std::iter::Iterator<Item = &NoteId> {
        self.ids.iter()
    }
}

#[derive(CandidType, Deserialize, Default)]
pub struct PassportIds {
    ids: Vec<PassportId>,
}

#[derive(CandidType, Deserialize, Default)]
pub struct TokenIds {
    ids: Vec<ApiTokenId>,
}

impl PassportIds {
    pub fn iter(&self) -> impl std::iter::Iterator<Item = &PassportId> {
        self.ids.iter()
    }
}

impl TokenIds {
    pub fn iter(&self) -> impl std::iter::Iterator<Item = &ApiTokenId> {
        self.ids.iter()
    }
}

impl Storable for PassportIds {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for TokenIds {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for NoteIds {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

// We use a canister's stable memory as storage. This simplifies the code and makes the appliation
// more robust because no (potentially failing) pre_upgrade/post_upgrade hooks are needed.
// Note that stable memory is less performant than heap memory, however.
// Currently, a single canister smart contract is limited to 96 GB of stable memory.
// For the current limits see https://internetcomputer.org/docs/current/developer-docs/production/resource-limits.
// To ensure that our canister does not exceed the limit, we put various restrictions (e.g., number of users) in place.
static MAX_USERS: u64 = 1_000;
static MAX_NOTES_PER_USER: usize = 500;
static MAX_NOTE_CHARS: usize = 1_000_000;
static MAX_SHARES_PER_NOTE: usize = 50;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static NEXT_NOTE_ID: RefCell<StableCell<NoteId, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(0))),
            1
        ).expect("failed to init NEXT_NOTE_ID")
    );

    static NOTES: RefCell<StableBTreeMap<NoteId, EncryptedNote, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(1))),
        )
    );

    static NOTE_OWNERS: RefCell<StableBTreeMap<PrincipalName, NoteIds, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(2))),
        )
    );

    static NOTE_SHARES: RefCell<StableBTreeMap<PrincipalName, NoteIds, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(3))),
        )
    );

    // New storage for agent passports and memories
    static NEXT_PASSPORT_ID: RefCell<StableCell<PassportId, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(4))),
            1
        ).expect("failed to init NEXT_PASSPORT_ID")
    );

    static PASSPORTS: RefCell<StableBTreeMap<PassportId, AgentPassport, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(5))),
        )
    );

    static NEXT_MEMORY_ID: RefCell<StableCell<AgentMemoryId, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(6))),
            1
        ).expect("failed to init NEXT_MEMORY_ID")
    );

    static AGENT_MEMORIES: RefCell<StableBTreeMap<AgentMemoryId, AgentMemory, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(7))),
        )
    );

    static PASSPORT_OWNERS: RefCell<StableBTreeMap<PrincipalName, PassportIds, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(8))),
        )
    );

    // API Token storage
    static NEXT_TOKEN_ID: RefCell<StableCell<ApiTokenId, Memory>> = RefCell::new(
        StableCell::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(9))),
            1
        ).expect("failed to init NEXT_TOKEN_ID")
    );

    static API_TOKENS: RefCell<StableBTreeMap<ApiTokenId, ApiToken, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(10))),
        )
    );

    static TOKEN_OWNERS: RefCell<StableBTreeMap<PrincipalName, TokenIds, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with_borrow(|m| m.get(MemoryId::new(11))),
        )
    );
}

/// Unlike Motoko, the caller identity is not built into Rust.
/// Thus, we use the ic_cdk::caller() method inside this wrapper function.
/// The wrapper prevents the use of the anonymous identity. Forbidding anonymous
/// interactions is the recommended default behavior for IC canisters.
fn caller() -> Principal {
    let caller = msg_caller();
    // The anonymous principal is not allowed to interact with the
    // encrypted notes canister.
    if caller == Principal::anonymous() {
        panic!("Anonymous principal not allowed to make calls.")
    }
    caller
}

// --- Queries vs. Updates ---
//
// Note that our public methods are declared as an *updates* rather than *queries*, e.g.:
// #[update(name = "notesCnt")] ...
// rather than
// #[query(name = "notesCnt")] ...
//
// While queries are significantly faster than updates, they are not certified by the IC.
// Thus, we avoid using queries throughout this dapp, ensuring that the result of our
// methods gets through consensus. Otherwise, this method could e.g. omit some notes
// if it got executed by a malicious node. (To make the dapp more efficient, one could
// use an approach in which both queries and updates are combined.)
//
// See https://internetcomputer.org/docs/current/concepts/canisters-code#query-and-update-methods

/// Reflects the [caller]'s identity by returning (a future of) its principal.
/// Useful for debugging.
#[update]
fn whoami() -> String {
    msg_caller().to_string()
}

// General assumptions
// -------------------
// All the functions of this canister's public API should be available only to
// registered users, with the exception of [whoami].

/// Returns (a future of) this [caller]'s notes.
/// Panics:
///     [caller] is the anonymous identity
#[update]
fn get_notes() -> Vec<EncryptedNote> {
    let user_str = caller().to_string();
    NOTES.with_borrow(|notes| {
        let owned = NOTE_OWNERS.with_borrow(|ids| {
            ids.get(&user_str)
                .unwrap_or_default()
                .iter()
                .map(|id| notes.get(id).ok_or(format!("missing note with ID {id}")))
                .collect::<Result<Vec<_>, _>>()
                .unwrap_or_else(|err| ic_cdk::trap(&err))
        });
        let shared = NOTE_SHARES.with_borrow(|ids| {
            ids.get(&user_str)
                .unwrap_or_default()
                .iter()
                .map(|id| notes.get(id).ok_or(format!("missing note with ID {id}")))
                .collect::<Result<Vec<_>, _>>()
                .unwrap_or_else(|err| ic_cdk::trap(&err))
        });
        let mut result = Vec::with_capacity(owned.len() + shared.len());
        result.extend(owned);
        result.extend(shared);
        result
    })
}

/// Delete this [caller]'s note with given id. If none of the
/// existing notes have this id, do nothing.
/// [id]: the id of the note to be deleted
///
/// Returns:
///      Future of unit
/// Panics:
///      [caller] is the anonymous identity
///      [caller] is not the owner of note with id `note_id`
#[update]
fn delete_note(note_id: u128) {
    let user_str = caller().to_string();
    NOTES.with_borrow_mut(|notes| {
        if let Some(note_to_delete) = notes.get(&note_id) {
            let owner = &note_to_delete.owner;
            if owner != &user_str {
                ic_cdk::trap("only the owner can delete notes");
            }
            NOTE_OWNERS.with_borrow_mut(|owner_to_nids| {
                if let Some(mut owner_ids) = owner_to_nids.get(owner) {
                    owner_ids.ids.retain(|&id| id != note_id);
                    if !owner_ids.ids.is_empty() {
                        owner_to_nids.insert(owner.clone(), owner_ids);
                    } else {
                        owner_to_nids.remove(owner);
                    }
                }
            });
            NOTE_SHARES.with_borrow_mut(|share_to_nids| {
                for share in note_to_delete.users {
                    if let Some(mut share_ids) = share_to_nids.get(&share) {
                        share_ids.ids.retain(|&id| id != note_id);
                        if !share_ids.ids.is_empty() {
                            share_to_nids.insert(share, share_ids);
                        } else {
                            share_to_nids.remove(&share);
                        }
                    }
                }
            });
            notes.remove(&note_id);
        }
    });
}

/// Replaces the encrypted text of note with ID [id] with [encrypted_text].
///
/// Panics:
///     [caller] is the anonymous identity
///     [caller] is not the note's owner and not a user with whom the note is shared
///     [encrypted_text] exceeds [MAX_NOTE_CHARS]
#[update]
fn update_note(id: NoteId, encrypted_text: String) {
    let user_str = caller().to_string();

    NOTES.with_borrow_mut(|notes| {
        if let Some(mut note_to_update) = notes.get(&id) {
            if !note_to_update.is_authorized(&user_str) {
                ic_cdk::trap("unauthorized update");
            }
            assert!(encrypted_text.chars().count() <= MAX_NOTE_CHARS);
            note_to_update.encrypted_text = encrypted_text;
            notes.insert(id, note_to_update);
        }
    })
}

/// Add new empty note for this [caller].
///
/// Returns:
///      Future of ID of new empty note
/// Panics:
///      [caller] is the anonymous identity
///      User already has [MAX_NOTES_PER_USER] notes
///      This is the first note for [caller] and [MAX_USERS] is exceeded
#[update]
fn create_note() -> NoteId {
    let owner = caller().to_string();

    NOTES.with_borrow_mut(|id_to_note| {
        NOTE_OWNERS.with_borrow_mut(|owner_to_nids| {
            let next_note_id = NEXT_NOTE_ID.with_borrow(|id| *id.get());
            let new_note = EncryptedNote {
                id: next_note_id,
                owner: owner.clone(),
                users: vec![],
                encrypted_text: String::new(),
            };

            if let Some(mut owner_nids) = owner_to_nids.get(&owner) {
                assert!(owner_nids.ids.len() < MAX_NOTES_PER_USER);
                owner_nids.ids.push(new_note.id);
                owner_to_nids.insert(owner, owner_nids);
            } else {
                assert!(owner_to_nids.len() < MAX_USERS);
                owner_to_nids.insert(
                    owner,
                    NoteIds {
                        ids: vec![new_note.id],
                    },
                );
            }
            assert_eq!(id_to_note.insert(new_note.id, new_note), None);

            NEXT_NOTE_ID.with_borrow_mut(|next_note_id| {
                let id_plus_one = next_note_id
                    .get()
                    .checked_add(1)
                    .expect("failed to increase NEXT_NOTE_ID: reached the maximum");
                next_note_id
                    .set(id_plus_one)
                    .unwrap_or_else(|_e| ic_cdk::trap("failed to set NEXT_NOTE_ID"))
            });
            next_note_id
        })
    })
}

/// Shares the note with ID `note_id`` with the `user`.
/// Has no effect if the note is already shared with that user.
///
/// Panics:
///      [caller] is the anonymous identity
///      [caller] is not the owner of note with id `note_id`
#[update]
fn add_user(note_id: NoteId, user: PrincipalName) {
    let caller_str = caller().to_string();
    NOTES.with_borrow_mut(|notes| {
        NOTE_SHARES.with_borrow_mut(|user_to_nids| {
            if let Some(mut note) = notes.get(&note_id) {
                let owner = &note.owner;
                if owner != &caller_str {
                    ic_cdk::trap("only the owner can share the note");
                }
                assert!(note.users.len() < MAX_SHARES_PER_NOTE);
                if !note.users.contains(&user) {
                    note.users.push(user.clone());
                    notes.insert(note_id, note);
                }
                if let Some(mut user_ids) = user_to_nids.get(&user) {
                    if !user_ids.ids.contains(&note_id) {
                        user_ids.ids.push(note_id);
                        user_to_nids.insert(user, user_ids);
                    }
                } else {
                    user_to_nids.insert(user, NoteIds { ids: vec![note_id] });
                }
            }
        })
    });
}

/// Unshares the note with ID `note_id`` with the `user`.
/// Has no effect if the note is not shared with that user.
///
/// Panics:
///      [caller] is the anonymous identity
///      [caller] is not the owner of note with id `note_id`
#[update]
fn remove_user(note_id: NoteId, user: PrincipalName) {
    let caller_str = caller().to_string();
    NOTES.with_borrow_mut(|notes| {
        NOTE_SHARES.with_borrow_mut(|user_to_nids| {
            if let Some(mut note) = notes.get(&note_id) {
                let owner = &note.owner;
                if owner != &caller_str {
                    ic_cdk::trap("only the owner can share the note");
                }
                note.users.retain(|u| u != &user);
                notes.insert(note_id, note);

                if let Some(mut user_ids) = user_to_nids.get(&user) {
                    user_ids.ids.retain(|&id| id != note_id);
                    if !user_ids.ids.is_empty() {
                        user_to_nids.insert(user, user_ids);
                    } else {
                        user_to_nids.remove(&user);
                    }
                }
            }
        })
    });
}

use ic_cdk::management_canister::{
    VetKDCurve, VetKDDeriveKeyArgs, VetKDDeriveKeyResult, VetKDKeyId, VetKDPublicKeyArgs,
    VetKDPublicKeyResult,
};

#[update]
async fn symmetric_key_verification_key_for_note() -> String {
    let request = VetKDPublicKeyArgs {
        canister_id: None,
        context: b"note_symmetric_key".to_vec(),
        key_id: bls12_381_g2_test_key_1(),
    };

    let response: VetKDPublicKeyResult = ic_cdk::management_canister::vetkd_public_key(&request)
        .await
        .expect("call to vetkd_public_key failed");

    hex::encode(response.public_key)
}

#[update]
async fn encrypted_symmetric_key_for_note(
    note_id: NoteId,
    transport_public_key: Vec<u8>,
) -> String {
    let user_str = caller().to_string();
    let request = NOTES.with_borrow(|notes| {
        if let Some(note) = notes.get(&note_id) {
            if !note.is_authorized(&user_str) {
                ic_cdk::trap(format!("unauthorized key request by user {user_str}"));
            }
            VetKDDeriveKeyArgs {
                input: {
                    let mut buf = vec![];
                    buf.extend_from_slice(&note_id.to_be_bytes()); // fixed-size encoding
                    buf.extend_from_slice(note.owner.as_bytes());
                    buf // prefix-free
                },
                context: b"note_symmetric_key".to_vec(),
                key_id: bls12_381_g2_test_key_1(),
                transport_public_key,
            }
        } else {
            ic_cdk::trap(format!("note with ID {note_id} does not exist"));
        }
    });

    let response: VetKDDeriveKeyResult = ic_cdk::management_canister::vetkd_derive_key(&request)
        .await
        .expect("call to vetkd_derive_key failed");

    hex::encode(response.encrypted_key)
}

fn bls12_381_g2_test_key_1() -> VetKDKeyId {
    VetKDKeyId {
        curve: VetKDCurve::Bls12_381_G2,
        name: "test_key_1".to_string(),
    }
}

// ===== AGENT PASSPORT FUNCTIONS =====

/// Creates a new agent passport with the given parameters
/// Returns the passport ID
#[update]
fn create_agent_passport(
    agent_name: String,
    agent_type: String,
    capabilities: Vec<String>,
    encrypted_specifications: String,
) -> PassportId {
    let owner = caller().to_string();
    let current_time = ic_cdk::api::time();

    PASSPORTS.with_borrow_mut(|passports| {
        PASSPORT_OWNERS.with_borrow_mut(|owners| {
            let next_passport_id = NEXT_PASSPORT_ID.with_borrow(|id| *id.get());
            
            let new_passport = AgentPassport {
                id: next_passport_id,
                agent_name,
                agent_type,
                owner: owner.clone(),
                capabilities,
                encrypted_specifications,
                api_endpoints: vec![], // Empty initially, can be updated later
                created_at: current_time,
                last_active: current_time,
                is_active: true,
            };

            // Add to passport owners mapping
            if let Some(mut owner_passports) = owners.get(&owner) {
                owner_passports.ids.push(next_passport_id);
                owners.insert(owner.clone(), owner_passports);
            } else {
                owners.insert(
                    owner,
                    PassportIds {
                        ids: vec![next_passport_id],
                    },
                );
            }

            // Store the passport
            passports.insert(next_passport_id, new_passport);

            // Increment the next passport ID
            NEXT_PASSPORT_ID.with_borrow_mut(|next_id| {
                let id_plus_one = next_id
                    .get()
                    .checked_add(1)
                    .expect("failed to increase NEXT_PASSPORT_ID: reached the maximum");
                next_id
                    .set(id_plus_one)
                    .unwrap_or_else(|_e| ic_cdk::trap("failed to set NEXT_PASSPORT_ID"))
            });

            next_passport_id
        })
    })
}

/// Retrieves an agent passport by ID
#[update]
fn get_agent_passport(passport_id: PassportId) -> Option<AgentPassport> {
    let user_str = caller().to_string();
    PASSPORTS.with_borrow(|passports| {
        if let Some(passport) = passports.get(&passport_id) {
            if passport.is_authorized(&user_str) {
                Some(passport)
            } else {
                ic_cdk::trap("unauthorized access to passport");
            }
        } else {
            None
        }
    })
}

/// Returns all passports owned by the caller
#[update]
fn get_my_passports() -> Vec<AgentPassport> {
    let user_str = caller().to_string();
    PASSPORTS.with_borrow(|passports| {
        PASSPORT_OWNERS.with_borrow(|owners| {
            if let Some(passport_ids) = owners.get(&user_str) {
                passport_ids
                    .iter()
                    .filter_map(|id| passports.get(id))
                    .collect()
            } else {
                vec![]
            }
        })
    })
}

/// Updates agent specifications
#[update]
fn update_agent_specifications(passport_id: PassportId, encrypted_specifications: String) {
    let user_str = caller().to_string();
    PASSPORTS.with_borrow_mut(|passports| {
        if let Some(mut passport) = passports.get(&passport_id) {
            if !passport.is_authorized(&user_str) {
                ic_cdk::trap("unauthorized update to passport");
            }
            passport.encrypted_specifications = encrypted_specifications;
            passport.last_active = ic_cdk::api::time();
            passports.insert(passport_id, passport);
        } else {
            ic_cdk::trap("passport not found");
        }
    });
}

/// Adds a memory entry for an agent
#[update]
fn add_agent_memory(
    passport_id: PassportId,
    memory_type: String,
    encrypted_content: String,
    importance_score: u8,
) -> AgentMemoryId {
    let owner = caller().to_string();
    
    // Verify passport ownership
    PASSPORTS.with_borrow(|passports| {
        if let Some(passport) = passports.get(&passport_id) {
            if !passport.is_authorized(&owner) {
                ic_cdk::trap("unauthorized access to passport");
            }
        } else {
            ic_cdk::trap("passport not found");
        }
    });

    AGENT_MEMORIES.with_borrow_mut(|memories| {
        let next_memory_id = NEXT_MEMORY_ID.with_borrow(|id| *id.get());
        
        let new_memory = AgentMemory {
            id: next_memory_id,
            passport_id,
            memory_type,
            encrypted_content,
            importance_score: importance_score.min(100), // Cap at 100
            created_at: ic_cdk::api::time(),
            owner,
        };

        memories.insert(next_memory_id, new_memory);

        // Increment the next memory ID
        NEXT_MEMORY_ID.with_borrow_mut(|next_id| {
            let id_plus_one = next_id
                .get()
                .checked_add(1)
                .expect("failed to increase NEXT_MEMORY_ID: reached the maximum");
            next_id
                .set(id_plus_one)
                .unwrap_or_else(|_e| ic_cdk::trap("failed to set NEXT_MEMORY_ID"))
        });

        next_memory_id
    })
}

/// Retrieves memories for an agent passport
#[update]
fn get_agent_memories(passport_id: PassportId, memory_type: Option<String>) -> Vec<AgentMemory> {
    let user_str = caller().to_string();
    
    // Verify passport ownership
    PASSPORTS.with_borrow(|passports| {
        if let Some(passport) = passports.get(&passport_id) {
            if !passport.is_authorized(&user_str) {
                ic_cdk::trap("unauthorized access to passport");
            }
        } else {
            ic_cdk::trap("passport not found");
        }
    });

    AGENT_MEMORIES.with_borrow(|memories| {
        memories
            .iter()
            .filter_map(|(_, memory)| {
                if memory.passport_id == passport_id && memory.is_authorized(&user_str) {
                    if let Some(ref filter_type) = memory_type {
                        if &memory.memory_type == filter_type {
                            Some(memory)
                        } else {
                            None
                        }
                    } else {
                        Some(memory)
                    }
                } else {
                    None
                }
            })
            .collect()
    })
}
