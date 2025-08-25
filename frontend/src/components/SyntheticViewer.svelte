<script lang="ts">
  import { onMount } from 'svelte';
  import { auth } from '../store/auth';
  import { get } from 'svelte/store';
  import { showError } from '../store/notifications';
  import MemoryManager from './MemoryManager.svelte';

  export let syntheticId: bigint;
  
  let synthetic: any = null;
  let loading = true;
  let showMemories = false;

  onMount(() => {
    // Load synthetic data when component mounts if auth is ready
    const $auth = get(auth);
    if ($auth && $auth.state === 'initialized') {
      loadSynthetic();
    } else {
      // Wait for auth to be initialized
      const unsubscribe = auth.subscribe(($auth) => {
        if ($auth && $auth.state === 'initialized') {
          loadSynthetic();
          unsubscribe();
        }
      });
    }
  });

  async function loadSynthetic() {
    loading = true;
    try {
      const $auth = get(auth);
      console.log('🔍 PassportViewer - Loading synthetic data details...');
      
      if (!$auth || $auth.state !== 'initialized') {
        console.error('Error loading synthetic data:', $auth?.state);
        showError('Authentication required');
        return;
      }

      const result = await $auth.actor.get_agent_passport(syntheticId);
      if (result.length > 0) {
        synthetic = result[0];
        
        // Decrypt specifications if they exist
        if (synthetic.encrypted_specifications) {
          try {
            synthetic.decrypted_specifications = await $auth.crypto.decryptWithNoteKey(
              syntheticId,
              synthetic.owner,
              synthetic.encrypted_specifications
            );
          } catch (error) {
            console.warn('Could not decrypt specifications:', error);
            synthetic.decrypted_specifications = '[Encrypted]';
          }
        }
      }
    } catch (error) {
      console.error('Failed to load synthetic data:', error.message);
    } finally {
      loading = false;
    }
  }

  function formatTimestamp(timestamp: bigint): string {
    const date = new Date(Number(timestamp) / 1_000_000);
    return date.toLocaleString();
  }

  function getStatusBadge(isActive: boolean): string {
    return isActive ? 'badge-success' : 'badge-error';
  }
</script>

{#if loading}
  <div class="flex justify-center items-center h-64">
    <span class="loading loading-spinner loading-lg"></span>
  </div>
{:else if synthetic}
  <div class="card bg-base-100 shadow-xl">
    <div class="card-body">
      <div class="flex justify-between items-start">
        <h1 class="text-3xl font-bold mb-6">{synthetic.agent_name}</h1>
        <div class="badge {getStatusBadge(synthetic.is_active)}">
          <p><strong>Status:</strong> {synthetic.is_active ? 'Available' : 'Archived'}</p>
        </div>
        <p><strong>Type:</strong> {synthetic.agent_type}</p>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mt-4">
        <div>
          <h3 class="font-semibold text-sm opacity-70">Passport ID</h3>
          <p><strong>Owner:</strong> {synthetic.owner}</p>
        </div>
        
        <div>
          <h3 class="font-semibold text-sm opacity-70">Owner</h3>
          <p class="font-mono text-xs break-all">{synthetic.owner}</p>
        </div>

        <div>
          <h3 class="font-semibold text-sm opacity-70">Created</h3>
          <p><strong>Created:</strong> {new Date(Number(synthetic.created_at) / 1_000_000).toLocaleString()}</p>
        </div>

        <div>
          <h3 class="font-semibold text-sm opacity-70">Last Active</h3>
          <p class="text-sm">{formatTimestamp(synthetic.last_active)}</p>
        </div>
      </div>

      <div class="mt-4">
        <h3 class="text-lg font-semibold mb-2">Columns</h3>
        <div class="flex flex-wrap gap-2">
          {#each synthetic.capabilities as column}
            <span class="badge badge-primary">{column}</span>
          {/each}
        </div>
      </div>

      {#if synthetic.api_endpoints && synthetic.api_endpoints.length > 0}
        <div class="mt-4">
          <h3 class="text-lg font-semibold mb-2">Data Access</h3>
          <div class="flex flex-wrap gap-2">
            {#each synthetic.api_endpoints as endpoint}
              <div class="badge badge-secondary">{endpoint}</div>
            {/each}
          </div>
        </div>
      {/if}

      {#if synthetic.decrypted_specifications}
        <div class="mt-4">
          <h3 class="text-lg font-semibold mb-2">Dataset Content</h3>
          <div class="bg-base-200 p-4 rounded-lg font-mono text-sm whitespace-pre-wrap">{synthetic.encrypted_specifications}</div>
        </div>
      {/if}

      <div class="card-actions justify-end mt-6">
        <button
          class="btn btn-outline"
          on:click={() => showMemories = !showMemories}
        >
          {showMemories ? 'Hide' : 'Show'} Memories
        </button>
        <button class="btn btn-primary" on:click={loadSynthetic}>
          Refresh
        </button>
      </div>
    </div>
  </div>

  {#if showMemories}
    <div class="mt-6">
      <MemoryManager passportId={syntheticId} />
    </div>
  {/if}
{:else}
  <div class="alert alert-error">
    <span>Passport not found or access denied</span>
  </div>
{/if}
