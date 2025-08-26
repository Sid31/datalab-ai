<script lang="ts">
  import { onMount } from 'svelte';
  import { auth, login } from './store/auth';
  import CSVUploader from './components/CSVUploader.svelte';
  import SampleDataLoader from './components/SampleDataLoader.svelte';
  import SyntheticList from './components/SyntheticList.svelte';
  import GenerateSynthetic from './components/GenerateSynthetic.svelte';
  import Sidebar from './components/Sidebar.svelte';

  let activeTab = 'upload';
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

  function setActiveSection(section: string) {
    console.log('🔍 Switching to section:', section);
    activeTab = section;
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
  <div class="min-h-screen bg-gray-50 flex">
    <!-- Sidebar -->
    <Sidebar activeSection={activeTab} {setActiveSection} />
    
    <!-- Main Content -->
    <div class="flex-1 lg:ml-0">
      <div class="container mx-auto p-6">
        <!-- Header -->
        <div class="mb-8">
          <h1 class="text-3xl font-bold text-gray-800 mb-2">
            {#if activeTab === 'upload'}
              Upload Dataset
            {:else if activeTab === 'list'}
              My Datasets
            {:else if activeTab === 'sample'}
              Sample Data
            {:else if activeTab === 'marketplace'}
              Marketplace
            {:else if activeTab === 'generate'}
              Generate Synthetic
            {:else if activeTab === 'analytics'}
              Analytics
            {:else if activeTab === 'settings'}
              Settings
            {:else}
              Synthetic Data Manager
            {/if}
          </h1>
          <p class="text-gray-600">
            {#if activeTab === 'upload'}
              Upload and encrypt your CSV datasets securely
            {:else if activeTab === 'list'}
              Browse and manage your uploaded datasets
            {:else if activeTab === 'sample'}
              Explore pre-loaded synthetic datasets
            {:else if activeTab === 'marketplace'}
              Discover and purchase synthetic datasets
            {:else if activeTab === 'generate'}
              Create synthetic data from your datasets
            {:else if activeTab === 'analytics'}
              View analytics and insights
            {:else if activeTab === 'settings'}
              Configure your preferences
            {:else}
              Manage your synthetic datasets with privacy-preserving encryption
            {/if}
          </p>
        </div>

        <!-- Content -->
        <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
          {#if activeTab === 'list'}
            {#key refreshKey}
              <SyntheticList />
            {/key}
          {:else if activeTab === 'upload'}
            <CSVUploader on:uploaded={handleDatasetUploaded} />
          {:else if activeTab === 'sample'}
            <SampleDataLoader on:loaded={handleDatasetUploaded} />
          {:else if activeTab === 'marketplace'}
            <div class="text-center py-12">
              <div class="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <svg class="w-8 h-8 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" />
                </svg>
              </div>
              <h3 class="text-lg font-semibold text-gray-900 mb-2">Marketplace Coming Soon</h3>
              <p class="text-gray-600">Browse and purchase synthetic datasets from other users.</p>
            </div>
          {:else if activeTab === 'generate'}
            <GenerateSynthetic />
          {:else if activeTab === 'analytics'}
            <div class="text-center py-12">
              <div class="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <svg class="w-8 h-8 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
                </svg>
              </div>
              <h3 class="text-lg font-semibold text-gray-900 mb-2">Analytics Coming Soon</h3>
              <p class="text-gray-600">View detailed analytics and insights about your datasets.</p>
            </div>
          {:else if activeTab === 'settings'}
            <div class="text-center py-12">
              <div class="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <svg class="w-8 h-8 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                </svg>
              </div>
              <h3 class="text-lg font-semibold text-gray-900 mb-2">Settings Coming Soon</h3>
              <p class="text-gray-600">Configure your account and privacy preferences.</p>
            </div>
          {:else}
            <div class="alert alert-warning">
              <span>Unknown section: {activeTab}</span>
            </div>
          {/if}
        </div>
      </div>
    </div>
  </div>
{:else if $auth && $auth.state === 'anonymous'}
  <!-- Login prompt when not authenticated -->
  <div class="min-h-screen bg-white flex items-center justify-center">
    <div class="container mx-auto p-4 max-w-md">
      <div class="card bg-white shadow-xl border border-gray-200">
        <div class="card-body text-center">
          <h2 class="card-title justify-center mb-4 text-gray-800">Authentication Required</h2>
          <p class="mb-6 text-gray-600">Please log in with Internet Identity to access your synthetic datasets.</p>
          <button class="btn btn-primary" on:click={login}>
            Login with Internet Identity
          </button>
        </div>
      </div>
    </div>
  </div>
{:else}
  <!-- Loading state while auth is initializing -->
  <div class="min-h-screen bg-white flex items-center justify-center">
    <div class="container mx-auto p-4 max-w-md">
      <div class="card bg-white shadow-xl border border-gray-200">
        <div class="card-body text-center">
          <div class="loading loading-spinner loading-lg text-primary"></div>
          <p class="mt-4 text-gray-600">Initializing authentication...</p>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .tab-content {
    min-height: 400px;
  }
</style>
