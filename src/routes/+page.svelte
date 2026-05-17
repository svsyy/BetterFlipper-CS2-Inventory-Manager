<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import LoginForm from '$lib/components/LoginForm.svelte';
  import { sessionStatus } from '$lib/stores/auth';

  
  onMount(() => {
    if ($sessionStatus.loggedIn) goto('/inventory');
  });

  async function onSuccess() {
    await goto('/inventory');
  }
</script>

<svelte:head><title>Sign in · CS2 Inventory Manager</title></svelte:head>

<div class="hero">
  <div class="brand">
    <div class="kicker">Open source</div>
    <h1>CS2 Inventory Manager</h1>
    <p>
      Sign in with your Steam account to view your CS2 inventory and manage your
      storage units. No telemetry, no servers — everything runs on your machine.
    </p>
    <ul class="bullets">
      <li>Pure-Rust backend · Tauri 2</li>
      <li>Talks directly to Steam and the CS2 Game Coordinator</li>
      <li>Refresh tokens stored in the OS keychain</li>
    </ul>
  </div>
  <LoginForm {onSuccess} />
</div>

<style>
  .hero {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 3rem;
    align-items: center;
    max-width: 880px;
    padding: 2rem;
  }
  .brand h1 {
    font-size: 1.875rem;
    font-weight: 700;
    margin-top: 0.5rem;
    background: linear-gradient(135deg, hsl(160 84% 60%), hsl(140 60% 80%));
    -webkit-background-clip: text;
    background-clip: text;
    color: transparent;
  }
  .brand p { font-size: 0.875rem; color: var(--color-fg-muted); margin: 0.75rem 0; line-height: 1.5; }
  .bullets {
    list-style: none;
    padding: 0;
    font-size: 0.75rem;
    color: var(--color-fg-dim);
  }
  .bullets li { padding: 0.2rem 0; }
  .bullets li::before { content: '— '; color: var(--color-accent); }
</style>
