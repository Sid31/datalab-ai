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
</style>
