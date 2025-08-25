<script lang="ts">
  import { onMount } from 'svelte';
  import { auth } from '../store/auth';
  import { get } from 'svelte/store';
  import { showError } from '../store/notifications';
  import SyntheticViewer from './SyntheticViewer.svelte';

  let datasets: any[] = [];
  let loading = true;
  let selectedDatasetId: bigint | null = null;

  onMount(() => {
    // Load datasets when component mounts if auth is ready
    const $auth = get(auth);
    if ($auth && $auth.state === 'initialized') {
      loadDatasets();
    } else {
      // Wait for auth to be initialized
      const unsubscribe = auth.subscribe(($auth) => {
        if ($auth && $auth.state === 'initialized') {
          loadDatasets();
          unsubscribe();
        }
      });
    }
  });

  async function loadDatasets() {
    loading = true;
    try {
      const $auth = get(auth);
      console.log('🔍 DatasetList - Loading datasets, auth state:', $auth?.state);
      
      if (!$auth || $auth.state !== 'initialized') {
        console.error('🚨 DatasetList - Authentication not initialized:', $auth?.state);
        showError('Authentication required');
        return;
      }

      console.log('🔍 DatasetList - Calling get_my_passports...');
      datasets = await $auth.actor.get_my_passports();
      
      // Sort by creation date (newest first)
      datasets.sort((a, b) => Number(b.created_at) - Number(a.created_at));
    } catch (error) {
      showError(`Failed to load datasets: ${error.message}`);
    } finally {
      loading = false;
    }
  }

  function selectDataset(datasetId: bigint) {
    selectedDatasetId = datasetId;
  }

  function getDatasetTypeIcon(type: string) {
    switch (type) {
      case 'csv_dataset': return '📊';
      case 'synthetic_dataset': return '🔬';
      default: return '📄';
    }
  }

  function getDatasetTypeName(type: string) {
    switch (type) {
      case 'csv_dataset': return 'CSV Dataset';
      case 'synthetic_dataset': return 'Synthetic Data';
      default: return 'Dataset';
    }
  }

  function formatTimestamp(timestamp: bigint): string {
    const date = new Date(Number(timestamp) / 1_000_000);
    return date.toLocaleDateString();
  }

  function getStatusBadge(isActive: boolean): string {
    return isActive ? 'badge-success' : 'badge-error';
  }
</script>

<div class="space-y-4">
  {#if selectedDatasetId}
    <div class="flex items-center gap-2 mb-4">
      <button class="btn btn-sm btn-outline" on:click={() => selectedDatasetId = null}>
        ← Back to List
      </button>
      <h2 class="text-lg font-semibold">Dataset Details</h2>
    </div>
     <SyntheticViewer syntheticId={selectedDatasetId} />
  {:else}
    <div class="flex justify-between items-center">
      <h2 class="text-xl font-bold">My Synthetic Datasets</h2>
      <button class="btn btn-sm btn-outline" on:click={loadDatasets}>
        Refresh
      </button>
    </div>

    {#if loading}
      <div class="flex justify-center items-center h-64">
        <span class="loading loading-spinner loading-lg"></span>
      </div>
    {:else if datasets.length === 0}
      <div class="text-center py-12">
        <div class="text-gray-500 mb-4">
          <svg class="w-16 h-16 mx-auto mb-4 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17H7A5 5 0 0 1 7 7h2m0 10v3a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1v-3m0 0V7a1 1 0 0 0-1-1H10a1 1 0 0 0-1 1v10z"></path>
          </svg>
          <p class="text-lg">No datasets found</p>
          <p class="text-sm">Upload your first CSV or load sample data to get started!</p>
        </div>
      </div>
    {:else}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {#each datasets as dataset}
          <div class="card bg-base-100 shadow-lg hover:shadow-xl transition-shadow cursor-pointer" on:click={() => selectDataset(dataset.id)}>
            <div class="card-body p-4">
              <div class="flex justify-between items-start mb-2">
                <div class="flex items-center gap-2">
                  <span class="text-2xl">{getDatasetTypeIcon(dataset.agent_type)}</span>
                  <h3 class="card-title text-lg">{dataset.agent_name}</h3>
                </div>
                <div class="badge {getStatusBadge(dataset.is_active)} text-xs">
                  {dataset.is_active ? 'Available' : 'Archived'}
                </div>
              </div>
              
              <div class="space-y-2">
                <div class="badge badge-outline">{getDatasetTypeName(dataset.agent_type)}</div>
                
                <div class="text-sm opacity-70">
                  Columns: {dataset.capabilities.length} | Created: {formatTimestamp(dataset.created_at)}
                </div>
                
                
                <div class="flex flex-wrap gap-1 mt-2">
                  {#each dataset.capabilities.slice(0, 3) as column}
                    <div class="badge badge-primary badge-xs">{column}</div>
                  {/each}
                  {#if dataset.capabilities.length > 3}
                    <div class="badge badge-ghost badge-xs">+{dataset.capabilities.length - 3} more</div>
                  {/if}
                </div>
              </div>
              
              <div class="card-actions justify-end mt-4">
                <button class="btn btn-primary btn-sm">View Dataset</button>
              </div>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  {/if}
</div>
