<script lang="ts">
  import { onMount } from 'svelte';
  import { auth } from '../store/auth';
  import { showError, showSuccess } from '../store/notifications';

  export let passportId: bigint;

  let memories: any[] = [];
  let loading = true;
  let addingMemory = false;
  let filterType = '';

  // Form fields for adding new memory
  let newMemoryType = 'conversation';
  let newMemoryContent = '';
  let newMemoryImportance = 50;

  const memoryTypes = [
    'conversation',
    'preference',
    'skill',
    'context',
    'goal',
    'instruction',
    'feedback',
    'error'
  ];

  onMount(async () => {
    await loadMemories();
  });

  async function loadMemories() {
    loading = true;
    try {
      const $auth = auth;
      if ($auth.state !== 'initialized') {
        showError('Authentication required');
        return;
      }

      const result = await $auth.actor.get_agent_memories(
        passportId,
        filterType ? [filterType] : []
      );
      
      memories = result;

      // Decrypt memory contents
      for (let memory of memories) {
        try {
          memory.decrypted_content = await $auth.crypto.decryptWithNoteKey(
            BigInt(memory.id),
            memory.owner,
            memory.encrypted_content
          );
        } catch (error) {
          console.warn('Could not decrypt memory:', error);
          memory.decrypted_content = '[Encrypted]';
        }
      }

      // Sort by importance and creation date
      memories.sort((a, b) => {
        if (a.importance_score !== b.importance_score) {
          return b.importance_score - a.importance_score;
        }
        return Number(b.created_at) - Number(a.created_at);
      });

    } catch (error) {
      showError(`Failed to load memories: ${error.message}`);
    } finally {
      loading = false;
    }
  }

  async function addMemory() {
    if (!newMemoryContent.trim()) {
      showError('Memory content is required');
      return;
    }

    addingMemory = true;
    try {
      const $auth = auth;
      if ($auth.state !== 'initialized') {
        showError('Authentication required');
        return;
      }

      // Encrypt the memory content
      const encryptedContent = await $auth.crypto.encryptWithNoteKey(
        BigInt(Date.now()),
        $auth.actor.getPrincipal().toString(),
        newMemoryContent
      );

      const memoryId = await $auth.actor.add_agent_memory(
        passportId,
        newMemoryType,
        encryptedContent,
        newMemoryImportance
      );

      showSuccess(`Memory added successfully! ID: ${memoryId}`);
      
      // Reset form
      newMemoryContent = '';
      newMemoryImportance = 50;
      
      // Reload memories
      await loadMemories();
    } catch (error) {
      showError(`Failed to add memory: ${error.message}`);
    } finally {
      addingMemory = false;
    }
  }

  function formatTimestamp(timestamp: bigint): string {
    const date = new Date(Number(timestamp) / 1_000_000);
    return date.toLocaleString();
  }

  function getImportanceBadge(score: number): string {
    if (score >= 80) return 'badge-error';
    if (score >= 60) return 'badge-warning';
    if (score >= 40) return 'badge-info';
    return 'badge-ghost';
  }

  function getImportanceLabel(score: number): string {
    if (score >= 80) return 'Critical';
    if (score >= 60) return 'High';
    if (score >= 40) return 'Medium';
    return 'Low';
  }

  async function applyFilter() {
    await loadMemories();
  }
</script>

<div class="card bg-base-100 shadow-xl">
  <div class="card-body">
    <h3 class="card-title">Agent Memories</h3>

    <!-- Filter Section -->
    <div class="flex gap-2 mb-4">
      <select class="select select-bordered select-sm" bind:value={filterType} on:change={applyFilter}>
        <option value="">All Types</option>
        {#each memoryTypes as type}
          <option value={type}>{type}</option>
        {/each}
      </select>
      <button class="btn btn-sm btn-outline" on:click={loadMemories}>
        Refresh
      </button>
    </div>

    <!-- Add Memory Section -->
    <div class="collapse collapse-arrow bg-base-200 mb-4">
      <input type="checkbox" />
      <div class="collapse-title text-sm font-medium">
        Add New Memory
      </div>
      <div class="collapse-content">
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mt-2">
          <div class="form-control">
            <label class="label label-text text-xs">Type</label>
            <select class="select select-bordered select-sm" bind:value={newMemoryType}>
              {#each memoryTypes as type}
                <option value={type}>{type}</option>
              {/each}
            </select>
          </div>
          
          <div class="form-control">
            <label class="label label-text text-xs">Importance (0-100)</label>
            <input
              type="range"
              min="0"
              max="100"
              class="range range-sm"
              bind:value={newMemoryImportance}
            />
            <div class="text-xs text-center">{newMemoryImportance}</div>
          </div>
          
          <div class="form-control">
            <label class="label label-text text-xs">Action</label>
            <button
              class="btn btn-primary btn-sm"
              class:loading={addingMemory}
              disabled={addingMemory || !newMemoryContent.trim()}
              on:click={addMemory}
            >
              Add Memory
            </button>
          </div>
        </div>
        
        <div class="form-control mt-2">
          <textarea
            class="textarea textarea-bordered textarea-sm"
            placeholder="Enter memory content..."
            bind:value={newMemoryContent}
            disabled={addingMemory}
          ></textarea>
        </div>
      </div>
    </div>

    <!-- Memories List -->
    {#if loading}
      <div class="flex justify-center items-center h-32">
        <span class="loading loading-spinner loading-md"></span>
      </div>
    {:else if memories.length === 0}
      <div class="text-center text-gray-500 py-8">
        No memories found. Add some memories to get started!
      </div>
    {:else}
      <div class="space-y-3 max-h-96 overflow-y-auto">
        {#each memories as memory}
          <div class="card bg-base-200 shadow-sm">
            <div class="card-body p-4">
              <div class="flex justify-between items-start mb-2">
                <div class="flex gap-2 items-center">
                  <div class="badge badge-outline text-xs">{memory.memory_type}</div>
                  <div class="badge {getImportanceBadge(memory.importance_score)} text-xs">
                    {getImportanceLabel(memory.importance_score)} ({memory.importance_score})
                  </div>
                </div>
                <div class="text-xs opacity-60">
                  {formatTimestamp(memory.created_at)}
                </div>
              </div>
              
              <div class="text-sm">
                {memory.decrypted_content}
              </div>
              
              <div class="text-xs opacity-50 mt-2">
                ID: {memory.id}
              </div>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>
