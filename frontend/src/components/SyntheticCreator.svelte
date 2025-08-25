<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { auth } from '../store/auth';
  import { get } from 'svelte/store';
  import { showError, showSuccess } from '../store/notifications';

  const dispatch = createEventDispatcher();

  let agentName = '';
  let agentType = 'eliza';
  let capabilities = ['conversation', 'memory_management'];
  let specifications = '';
  let isCreating = false;

  const agentTypes = [
    { value: 'eliza', label: 'Eliza Agent' },
    { value: 'custom', label: 'Custom Agent' },
    { value: 'assistant', label: 'Assistant Agent' },
    { value: 'analyzer', label: 'Analyzer Agent' }
  ];

  const availableCapabilities = [
    'conversation',
    'memory_management',
    'api_calls',
    'data_analysis',
    'content_generation',
    'task_automation',
    'monitoring',
    'integration'
  ];

  function toggleCapability(capability: string) {
    if (capabilities.includes(capability)) {
      capabilities = capabilities.filter(c => c !== capability);
    } else {
      capabilities = [...capabilities, capability];
    }
  }

  async function createPassport() {
    console.log('🔍 PassportCreator - Starting passport creation');
    
    if (!agentName.trim()) {
      showError('Agent name is required');
      return;
    }

    if (capabilities.length === 0) {
      showError('At least one capability must be selected');
      return;
    }

    isCreating = true;
    try {
      const $auth = get(auth);
      console.log('🔍 PassportCreator - Auth state check:', {
        state: $auth.state,
        hasActor: !!$auth.actor,
        hasClient: !!$auth.client,
        hasCrypto: !!$auth.crypto
      });
      
      if (!$auth || $auth.state !== 'initialized') {
        console.error('🚨 PassportCreator - Authentication not initialized:', $auth?.state);
        showError('Authentication required');
        return;
      }

      // Encrypt specifications using the crypto service
      const encryptedSpecs = specifications.trim() ? 
        await $auth.crypto.encryptWithNoteKey(BigInt(Date.now()), $auth.actor.getPrincipal().toString(), specifications) :
        '';

      const passportId = await $auth.actor.create_agent_passport(
        agentName.trim(),
        agentType,
        capabilities,
        encryptedSpecs
      );

      showSuccess(`Agent passport created successfully! ID: ${passportId}`);
      dispatch('created', { passportId });
      
      // Reset form
      agentName = '';
      agentType = 'eliza';
      capabilities = ['conversation', 'memory_management'];
      specifications = '';
    } catch (error) {
      showError(`Failed to create passport: ${error.message}`);
    } finally {
      isCreating = false;
    }
  }
</script>

<div class="card bg-base-100 shadow-xl">
  <div class="card-body">
    <h2 class="card-title">Create Agent Passport</h2>
    
    <div class="form-control">
      <label class="label" for="agent-name">
        <span class="label-text">Agent Name</span>
      </label>
      <input
        id="agent-name"
        type="text"
        placeholder="Enter agent name"
        class="input input-bordered"
        bind:value={agentName}
        disabled={isCreating}
      />
    </div>

    <div class="form-control">
      <label class="label" for="agent-type">
        <span class="label-text">Agent Type</span>
      </label>
      <select
        id="agent-type"
        class="select select-bordered"
        bind:value={agentType}
        disabled={isCreating}
      >
        {#each agentTypes as type}
          <option value={type.value}>{type.label}</option>
        {/each}
      </select>
    </div>

    <div class="form-control">
      <label class="label">
        <span class="label-text">Capabilities</span>
      </label>
      <div class="grid grid-cols-2 gap-2">
        {#each availableCapabilities as capability}
          <label class="label cursor-pointer">
            <span class="label-text">{capability}</span>
            <input
              type="checkbox"
              class="checkbox checkbox-primary"
              checked={capabilities.includes(capability)}
              on:change={() => toggleCapability(capability)}
              disabled={isCreating}
            />
          </label>
        {/each}
      </div>
    </div>

    <div class="form-control">
      <label class="label" for="specifications">
        <span class="label-text">Specifications (Optional)</span>
      </label>
      <textarea
        id="specifications"
        class="textarea textarea-bordered h-24"
        placeholder="Enter agent specifications, configuration, or special instructions..."
        bind:value={specifications}
        disabled={isCreating}
      ></textarea>
    </div>

    <div class="card-actions justify-end">
      <button
        class="btn btn-primary"
        class:loading={isCreating}
        disabled={isCreating || !agentName.trim()}
        on:click={createPassport}
      >
        {isCreating ? 'Creating...' : 'Create Passport'}
      </button>
    </div>
  </div>
</div>

<style>
  .grid {
    max-height: 200px;
    overflow-y: auto;
  }
</style>
