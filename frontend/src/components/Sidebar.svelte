<script lang="ts">
  import { auth } from '../store/auth';

  export let activeSection: string = 'upload';
  export let setActiveSection: (section: string) => void;

  let isCollapsed = false;
  let isMobileMenuOpen = false;

  $: isAuthenticated = $auth?.state === 'initialized';
  $: principal = $auth?.client?.getIdentity()?.getPrincipal();

  const menuItems = [
    { id: 'upload', icon: 'upload', label: 'Upload Dataset', section: 'main' },
    { id: 'list', icon: 'datasets', label: 'My Datasets', section: 'main' },
    { id: 'sample', icon: 'samples', label: 'Sample Data', section: 'main' },
    { id: 'marketplace', icon: 'marketplace', label: 'Marketplace', section: 'main' },
    { id: 'generate', icon: 'generate', label: 'Generate Synthetic', section: 'workspace' },
    { id: 'analytics', icon: 'analytics', label: 'Analytics', section: 'workspace' },
    { id: 'settings', icon: 'settings', label: 'Settings', section: 'workspace' },
  ];

  const getIcon = (iconName: string) => {
    const iconProps = "w-5 h-5";
    switch (iconName) {
      case 'upload':
        return `<svg class="${iconProps}" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" /></svg>`;
      case 'datasets':
        return `<svg class="${iconProps}" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" /></svg>`;
      case 'samples':
        return `<svg class="${iconProps}" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" /></svg>`;
      case 'marketplace':
        return `<svg class="${iconProps}" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" /></svg>`;
      case 'generate':
        return `<svg class="${iconProps}" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>`;
      case 'analytics':
        return `<svg class="${iconProps}" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" /></svg>`;
      case 'settings':
        return `<svg class="${iconProps}" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" /></svg>`;
      default:
        return `<div class="${iconProps} bg-gray-300 rounded"></div>`;
    }
  };

  function logout() {
    if ($auth?.client) {
      $auth.client.logout();
      window.location.reload();
    }
  }
</script>

{#if isAuthenticated}
  <!-- Mobile Menu Button -->
  <button
    on:click={() => isMobileMenuOpen = !isMobileMenuOpen}
    class="lg:hidden fixed top-4 left-4 z-50 bg-gray-900 text-white p-2 rounded-lg shadow-lg"
  >
    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
    </svg>
  </button>

  <!-- Mobile Overlay -->
  {#if isMobileMenuOpen}
    <div
      class="lg:hidden fixed inset-0 bg-black bg-opacity-50 z-40"
      on:click={() => isMobileMenuOpen = false}
    />
  {/if}

  <!-- Sidebar -->
  <div class="
    fixed lg:static inset-y-0 left-0 z-40
    {isCollapsed ? 'w-20' : 'w-64'}
    {isMobileMenuOpen ? 'translate-x-0' : '-translate-x-full lg:translate-x-0'}
    bg-gray-900 text-white
    transition-all duration-300 ease-in-out
    flex flex-col shadow-xl
  ">
    <!-- Header -->
    <div class="flex items-center justify-center p-4 border-b border-gray-800">
      <button
        on:click={() => isCollapsed = !isCollapsed}
        class="flex items-center justify-center w-full hover:opacity-80 transition-opacity duration-200"
        title={isCollapsed ? "Expand sidebar" : "Collapse sidebar"}
      >
        {#if isCollapsed}
          <img 
            src="/vram-unified-gradient.svg" 
            alt="VRAM" 
            class="w-10 h-10 object-contain flex-shrink-0"
          />
        {:else}
          <img 
            src="/logo.svg" 
            alt="VRAM Synth" 
            class="h-20 object-contain max-w-full flex-shrink-0"
          />
        {/if}
      </button>
    </div>

    <!-- User Info -->
    <div class="p-4 mx-4 mb-6 bg-gray-800/50 rounded-2xl border border-gray-700/50">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 bg-gradient-to-br from-green-400 to-green-500 rounded-full flex items-center justify-center shadow-lg">
          <span class="text-white text-sm font-semibold">
            {principal?.toString().slice(0, 2).toUpperCase() || 'U'}
          </span>
        </div>
        {#if !isCollapsed}
          <div class="flex-1 min-w-0">
            <div class="text-white text-sm font-medium">Connected</div>
            <div class="text-gray-400 text-xs truncate">
              {principal?.toString().slice(0, 12) || 'Loading'}...
            </div>
          </div>
        {/if}
        {#if !isCollapsed}
          <button 
            on:click={logout}
            class="ml-2 hover:bg-gray-700 rounded-full p-1.5 transition-all duration-200 text-gray-400 hover:text-white"
            title="Disconnect"
          >
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        {/if}
      </div>
    </div>

    <!-- Navigation -->
    <div class="flex-1 overflow-y-auto">
      <!-- Main Section -->
      <div class="px-4 space-y-1">
        {#each menuItems.filter(item => item.section === 'main') as item}
          <div
            on:click={() => setActiveSection(item.id)}
            class="flex items-center gap-3 px-3 py-2.5 rounded-lg cursor-pointer transition-all duration-200 {
              activeSection === item.id
                ? 'bg-gradient-to-r from-green-400 to-green-500 text-white shadow-lg'
                : 'text-gray-300 hover:bg-gray-800 hover:text-white'
            }"
          >
            {@html getIcon(item.icon)}
            {#if !isCollapsed}
              <span class="font-medium text-sm">{item.label}</span>
            {/if}
          </div>
        {/each}
      </div>

      <!-- Workspace Section -->
      {#if !isCollapsed}
        <div class="px-4 mt-8">
          <span class="text-gray-500 text-xs font-medium uppercase tracking-wide mb-3 block">
            Workspace
          </span>
          <div class="space-y-1">
            {#each menuItems.filter(item => item.section === 'workspace') as item}
              <div
                on:click={() => setActiveSection(item.id)}
                class="flex items-center gap-3 px-3 py-2.5 rounded-lg cursor-pointer transition-all duration-200 {
                  activeSection === item.id
                    ? 'bg-gradient-to-r from-green-400 to-green-500 text-white shadow-lg'
                    : 'text-gray-300 hover:bg-gray-800 hover:text-white'
                }"
              >
                {@html getIcon(item.icon)}
                <span class="font-medium text-sm">{item.label}</span>
              </div>
            {/each}
          </div>
        </div>
      {/if}
    </div>

    <!-- Footer -->
    <div class="p-4 border-t border-gray-800">
      <button 
        on:click={logout}
        class="flex items-center gap-3 w-full px-3 py-2.5 rounded-lg cursor-pointer transition-all duration-200 text-gray-300 hover:bg-gray-800 hover:text-white"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
        </svg>
        {#if !isCollapsed}
          <span class="font-medium text-sm">Log out</span>
        {/if}
      </button>
    </div>
  </div>
{/if}
