<script lang="ts">
  import Hero from './components/Hero.svelte';
  import Synthetic from './Synthetic.svelte';
  import Notifications from './components/Notifications.svelte';
  import { auth } from './store/auth';
</script>

{#if $auth.state === 'initialized'}
  <Synthetic />
{:else}
  <Hero auth={$auth} />
{/if}
<Notifications />

<style lang="postcss" global>
  @tailwind base;
  @tailwind components;
  @tailwind utilities;

  /* Base styling - white background theme */
  body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
    background-color: #ffffff;
    color: #2d3748;
    min-height: 100vh;
    line-height: 1.6;
  }

  /* Custom button styles */
  .btn-primary {
    background-image: linear-gradient(to right, #2d3748, #4a5568);
    color: white;
    box-shadow: 0 8px 25px rgba(72, 187, 120, 0.25);
    border: none;
  }

  .btn-primary:hover {
    transform: translateY(-0.25rem);
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
    background-image: linear-gradient(to right, #4a5568, #718096);
  }

  .btn-secondary {
    color: #48bb78;
    border: 2px solid #48bb78;
    background-color: white;
  }

  .btn-secondary:hover {
    transform: translateY(-0.125rem);
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
    color: white;
    background-color: #48bb78;
    border-color: #48bb78;
  }

  /* Card styles */
  .card {
    background-color: white;
    border: 1px solid #e2e8f0;
    transition: all 300ms ease;
  }

  .card:hover {
    border-color: #48bb78;
    transform: translateY(-0.125rem);
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
  }

  .card.selected {
    border-color: #48bb78;
    background-color: #f0fff4;
    box-shadow: 0 4px 15px rgba(72, 187, 120, 0.1);
  }

  /* Header styles */
  .header {
    background-color: white;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  /* Navigation styles */
  .nav-item {
    color: #a0aec0;
    font-weight: 500;
    cursor: pointer;
    transition: color 200ms ease;
  }

  .nav-item.active, 
  .nav-item:hover {
    color: #2d3748;
  }
</style>
