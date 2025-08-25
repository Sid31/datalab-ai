<script lang="ts">
  import { onMount } from 'svelte';
  import { auth, login } from './store/auth';
  import CSVUploader from './components/CSVUploader.svelte';
  import SampleDataLoader from './components/SampleDataLoader.svelte';
  import SyntheticList from './components/SyntheticList.svelte';

  let activeTab = 'list';
  let refreshKey = 0;

  // Debug authentication state
  $: {
    console.log('🔍 Synthetic.svelte - Auth State Debug:', {
      state: $auth?.state,
      hasActor: !!$auth?.actor,
      hasClient: !!$auth?.client,
      hasCrypto: !!$auth?.crypto,
      timestamp: new Date().toISOString()
    });
  }

  function handleDatasetUploaded() {
    activeTab = 'list';
    refreshKey++; // Force refresh of dataset list
  }

  function switchTab(tab: string) {
    console.log('🔍 Switching to tab:', tab);
    activeTab = tab;
    console.log('🔍 Active tab is now:', activeTab);
  }
</script>

<!-- Debug Info Panel -->
<div class="bg-yellow-100 border border-yellow-400 text-yellow-700 px-4 py-3 rounded mb-4">
  <strong>🔍 Debug Info:</strong> Auth State = "{$auth?.state || 'undefined'}" | 
  Actor: {$auth?.actor ? '✅' : '❌'} | 
  Client: {$auth?.client ? '✅' : '❌'} | 
  Crypto: {$auth?.crypto ? '✅' : '❌'} | 
  Active Tab: "{activeTab}"
</div>

{#if $auth && $auth.state === 'initialized'}
  <div class="container mx-auto p-4 max-w-6xl">
    <div class="mb-6">
      <h1 class="text-3xl font-bold mb-2">Synthetic Data Manager</h1>
      <p class="text-gray-600">Upload, store, and manage your synthetic datasets with privacy controls</p>
    </div>

    <!-- Tab Navigation -->
    <div class="tabs tabs-boxed mb-6">
      <button 
        class="tab {activeTab === 'list' ? 'tab-active' : ''}"
        on:click={() => switchTab('list')}
      >
        My Datasets
      </button>
      <button 
        class="tab {activeTab === 'upload' ? 'tab-active' : ''}"
        on:click={() => {
          console.log('🔍 Upload CSV button clicked');
          switchTab('upload');
        }}
      >
        Upload CSV
      </button>
      <button 
        class="tab {activeTab === 'sample' ? 'tab-active' : ''}"
        on:click={() => switchTab('sample')}
      >
        Sample Data
      </button>
    </div>

    <!-- Tab Content -->
    <div class="tab-content">
      {#if activeTab === 'list'}
        {#key refreshKey}
          <SyntheticList />
        {/key}
      {:else if activeTab === 'upload'}
        <CSVUploader on:uploaded={handleDatasetUploaded} />
      {:else if activeTab === 'sample'}
        <SampleDataLoader on:loaded={handleDatasetUploaded} />
      {:else}
        <div class="alert alert-warning">
          <span>Unknown tab: {activeTab}</span>
        </div>
      {/if}
    </div>
  </div>
{:else if $auth && $auth.state === 'anonymous'}
  <!-- Login prompt when not authenticated -->
  <div class="container mx-auto p-4 max-w-md">
    <div class="card bg-base-100 shadow-xl">
      <div class="card-body text-center">
        <h2 class="card-title justify-center mb-4">Authentication Required</h2>
        <p class="mb-6">Please log in with Internet Identity to access your synthetic datasets.</p>
        <button class="btn btn-primary" on:click={login}>
          Login with Internet Identity
        </button>
      </div>
    </div>
  </div>
{:else}
  <!-- Loading state while auth is initializing -->
  <div class="container mx-auto p-4 max-w-md">
    <div class="card bg-base-100 shadow-xl">
      <div class="card-body text-center">
        <div class="loading loading-spinner loading-lg"></div>
        <p class="mt-4">Initializing authentication...</p>
      </div>
    </div>
  </div>
{/if}

<style>
  .tab-content {
    min-height: 400px;
  }
</style>
