<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { page } from '$app/state';

  import TitleBar from '$lib/components/TitleBar.svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import TetrisLoader from '$lib/components/TetrisLoader.svelte';

  import { steam, onLoginStatus, onInventoryUpdated } from '$lib/api';
  import { sessionStatus, loginStage, loginMessage, applyLoginStatus } from '$lib/stores/auth';
  import { items } from '$lib/stores/inventory';

  let { children } = $props();
  let booted = $state(false);
  let autoLoginAttempted = $state(false);

  onMount(() => {
    let unlistenStatus: (() => void) | undefined;
    let unlistenInv: (() => void) | undefined;

    (async () => {
      try {
        const s = await steam.sessionStatus();
        sessionStatus.set(s);

        
        
        
        if (!s.loggedIn && !autoLoginAttempted && s.savedAccounts.length > 0) {
          autoLoginAttempted = true;
          const lastAccount = s.savedAccounts[0];
          try {
            await steam.loginWithToken(lastAccount);
            const refreshed = await steam.sessionStatus();
            sessionStatus.set(refreshed);
            if (refreshed.loggedIn) {
              await goto('/inventory');
            }
          } catch (e) {
            console.warn('auto-login failed', e);
          }
        }

        const finalStatus = await steam.sessionStatus();
        if (!finalStatus.loggedIn && page.url.pathname !== '/') {
          await goto('/');
        }
      } catch (e) {
        console.warn('sessionStatus failed', e);
      } finally {
        booted = true;
      }

      unlistenStatus = await onLoginStatus(async (p) => {
        applyLoginStatus(p);
        
        
        
        if (p.stage === 'ready' || p.stage === 'loading-inventory' || p.stage === 'waiting-gc') {
          try {
            const fresh = await steam.sessionStatus();
            sessionStatus.set(fresh);
          } catch {}
        }
      });
      unlistenInv = await onInventoryUpdated((next) => items.set(next));
    })();

    return () => {
      unlistenStatus?.();
      unlistenInv?.();
    };
  });

  const showShell = $derived($sessionStatus.loggedIn && booted);
</script>

<div class="app">
  <TitleBar />

  {#if !booted}
    <div class="boot mesh-bg">
      <TetrisLoader label="Booting…" />
    </div>
  {:else if showShell}
    <main class="shell">
      <Sidebar />
      <section class="content mesh-bg">
        {#if $loginStage !== 'idle' && $loginStage !== 'ready' && $loginStage !== 'error'}
          <div class="status-bar">{$loginMessage ?? $loginStage}</div>
        {/if}
        {@render children()}
      </section>
    </main>
  {:else}
    <main class="auth mesh-bg">
      {@render children()}
    </main>
  {/if}
</div>

<style>
  .app { display: flex; flex-direction: column; height: 100vh; overflow: hidden; }
  .boot { flex: 1; display: flex; align-items: center; justify-content: center; }
  .shell { flex: 1; display: flex; overflow: hidden; }
  .content { flex: 1; display: flex; flex-direction: column; overflow: hidden; }
  .auth { flex: 1; display: flex; align-items: center; justify-content: center; }
  .status-bar {
    padding: 0.25rem 0.75rem;
    font-size: 0.6875rem;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: hsl(160 84% 75%);
    background: var(--color-accent-soft);
    border-bottom: 1px solid hsla(160 84% 39% / 0.25);
  }
</style>
