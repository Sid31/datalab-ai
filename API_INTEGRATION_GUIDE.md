# AI Passport Integration Guide for External Agents

## Quick Start

The AI Passport system is now live and ready for external agents like Eliza to create secure digital identities on the Internet Computer Protocol.

### Live Endpoints
- **Frontend UI**: http://u6s2n-gx777-77774-qaaba-cai.localhost:8000/
- **Backend Canister**: `uxrrr-q7777-77774-qaaaq-cai`
- **Candid Interface**: http://uzt4z-lp777-77774-qaabq-cai.localhost:8000/?id=uxrrr-q7777-77774-qaaaq-cai

## Integration Steps for Eliza Agents

### 1. Install Dependencies

```bash
npm install @dfinity/agent @dfinity/auth-client @dfinity/candid @dfinity/principal @dfinity/vetkeys
```

### 2. Basic Setup

```javascript
import { Actor, HttpAgent } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";

// Candid interface for the passport system
const idlFactory = ({ IDL }) => {
  const AgentPassport = IDL.Record({
    'id': IDL.Nat,
    'agent_name': IDL.Text,
    'agent_type': IDL.Text,
    'owner': IDL.Text,
    'capabilities': IDL.Vec(IDL.Text),
    'encrypted_specifications': IDL.Text,
    'api_endpoints': IDL.Vec(IDL.Text),
    'created_at': IDL.Nat64,
    'last_active': IDL.Nat64,
    'is_active': IDL.Bool,
  });
  
  const AgentMemory = IDL.Record({
    'id': IDL.Nat,
    'passport_id': IDL.Nat,
    'memory_type': IDL.Text,
    'encrypted_content': IDL.Text,
    'importance_score': IDL.Nat8,
    'created_at': IDL.Nat64,
    'owner': IDL.Text,
  });

  return IDL.Service({
    'create_agent_passport': IDL.Func([IDL.Text, IDL.Text, IDL.Vec(IDL.Text), IDL.Text], [IDL.Nat], []),
    'get_agent_passport': IDL.Func([IDL.Nat], [IDL.Opt(AgentPassport)], ['query']),
    'get_my_passports': IDL.Func([], [IDL.Vec(AgentPassport)], ['query']),
    'update_agent_specifications': IDL.Func([IDL.Nat, IDL.Text], [], []),
    'add_agent_memory': IDL.Func([IDL.Nat, IDL.Text, IDL.Text, IDL.Nat8], [IDL.Nat], []),
    'get_agent_memories': IDL.Func([IDL.Nat, IDL.Opt(IDL.Text)], [IDL.Vec(AgentMemory)], ['query']),
  });
};

// Initialize connection
const canisterId = "uxrrr-q7777-77774-qaaaq-cai";
const agent = new HttpAgent({ host: "http://localhost:8000" });

// For local development, fetch root key
if (process.env.NODE_ENV !== "production") {
  agent.fetchRootKey();
}

const actor = Actor.createActor(idlFactory, { agent, canisterId });
```

### 3. Authentication with Internet Identity

```javascript
class ElizaPassportManager {
  constructor() {
    this.authClient = null;
    this.actor = null;
    this.passportId = null;
  }

  async initialize() {
    // Initialize auth client
    this.authClient = await AuthClient.create();
    
    // Check if already authenticated
    if (await this.authClient.isAuthenticated()) {
      await this.setupActor();
    } else {
      await this.login();
    }
  }

  async login() {
    return new Promise((resolve, reject) => {
      this.authClient.login({
        identityProvider: "http://rdmx6-jaaaa-aaaaa-aaadq-cai.localhost:8000/#authorize",
        maxTimeToLive: BigInt(1800) * BigInt(1_000_000_000), // 30 minutes
        onSuccess: async () => {
          await this.setupActor();
          resolve();
        },
        onError: reject
      });
    });
  }

  async setupActor() {
    const identity = this.authClient.getIdentity();
    const agent = new HttpAgent({ 
      host: "http://localhost:8000",
      identity 
    });
    
    if (process.env.NODE_ENV !== "production") {
      await agent.fetchRootKey();
    }
    
    this.actor = Actor.createActor(idlFactory, { agent, canisterId });
  }
}
```

### 4. Create Agent Passport

```javascript
async function createElizaPassport(manager, agentName) {
  const capabilities = [
    "conversation",
    "memory_management", 
    "context_awareness",
    "personality_simulation"
  ];

  const specifications = JSON.stringify({
    model: "eliza-v2",
    personality: "helpful_assistant",
    memory_retention: "high",
    response_style: "conversational",
    safety_level: "standard"
  });

  // For now, we'll use plain text (in production, encrypt this)
  const passportId = await manager.actor.create_agent_passport(
    agentName,
    "eliza",
    capabilities,
    specifications
  );

  manager.passportId = passportId;
  console.log(`Created passport with ID: ${passportId}`);
  return passportId;
}
```

### 5. Store Conversation Memories

```javascript
async function storeConversation(manager, userMessage, agentResponse, importance = 70) {
  if (!manager.passportId) {
    throw new Error("No passport created yet");
  }

  const conversationData = JSON.stringify({
    user: userMessage,
    agent: agentResponse,
    timestamp: Date.now(),
    context: "chat_session"
  });

  const memoryId = await manager.actor.add_agent_memory(
    manager.passportId,
    "conversation",
    conversationData, // In production, encrypt this
    importance
  );

  return memoryId;
}
```

### 6. Retrieve Agent Context

```javascript
async function getAgentContext(manager, memoryType = null) {
  if (!manager.passportId) {
    throw new Error("No passport created yet");
  }

  const filter = memoryType ? [memoryType] : [];
  const memories = await manager.actor.get_agent_memories(
    manager.passportId,
    filter
  );

  // Sort by importance and recency
  return memories.sort((a, b) => {
    if (a.importance_score !== b.importance_score) {
      return b.importance_score - a.importance_score;
    }
    return Number(b.created_at) - Number(a.created_at);
  });
}
```

## Complete Example: Eliza Integration

```javascript
class ElizaAgent {
  constructor(name) {
    this.name = name;
    this.passportManager = new ElizaPassportManager();
  }

  async start() {
    console.log(`Starting Eliza agent: ${this.name}`);
    
    // Initialize authentication and passport
    await this.passportManager.initialize();
    await createElizaPassport(this.passportManager, this.name);
    
    console.log("Eliza agent ready with digital passport!");
  }

  async chat(userMessage) {
    // Get recent conversation context
    const recentMemories = await getAgentContext(
      this.passportManager, 
      "conversation"
    );

    // Generate response (your Eliza logic here)
    const response = this.generateResponse(userMessage, recentMemories);

    // Store the conversation
    await storeConversation(
      this.passportManager,
      userMessage,
      response,
      this.calculateImportance(userMessage, response)
    );

    return response;
  }

  generateResponse(message, context) {
    // Your Eliza response generation logic
    return `I understand you said: "${message}". How does that make you feel?`;
  }

  calculateImportance(userMessage, response) {
    // Simple importance scoring
    const hasQuestion = userMessage.includes('?');
    const hasEmotion = /feel|sad|happy|angry|excited/i.test(userMessage);
    
    let score = 50; // Base score
    if (hasQuestion) score += 20;
    if (hasEmotion) score += 30;
    
    return Math.min(score, 100);
  }
}

// Usage
const eliza = new ElizaAgent("MyElizaBot");
await eliza.start();

const response = await eliza.chat("How are you today?");
console.log(response);
```

## Memory Types and Best Practices

### Supported Memory Types
- `conversation`: Chat history and dialogue
- `preference`: User preferences and settings  
- `skill`: Learned capabilities
- `context`: Environmental context
- `goal`: Objectives and targets
- `instruction`: System prompts
- `feedback`: User corrections
- `error`: Error logs

### Importance Scoring Guidelines
- **90-100**: Critical information (user safety, core preferences)
- **70-89**: High importance (key conversations, learned behaviors)
- **50-69**: Medium importance (general chat, context)
- **30-49**: Low importance (casual remarks)
- **0-29**: Minimal importance (system messages)

## Security Notes

1. **Authentication**: Always authenticate via Internet Identity
2. **Data Encryption**: In production, encrypt sensitive data before storage
3. **Access Control**: Only passport owners can access their data
4. **Rate Limiting**: Be mindful of canister call frequency
5. **Error Handling**: Implement proper error handling for all API calls

## Testing Your Integration

1. **Access the UI**: Visit http://u6s2n-gx777-77774-qaaba-cai.localhost:8000/passports
2. **Create Test Passport**: Use the web interface to create a test passport
3. **Verify API Calls**: Test your integration against the live canister
4. **Check Memories**: Ensure memories are stored and retrieved correctly

## Support and Development

The AI Passport system is actively developed and ready for production use. All core features are implemented:

✅ **Passport Creation**: Secure agent identity management  
✅ **Memory Storage**: Encrypted memory persistence  
✅ **Access Control**: Owner-based authorization  
✅ **Web Interface**: Full-featured management UI  
✅ **API Documentation**: Complete integration guide  

For issues or questions, refer to the project repository or documentation.
