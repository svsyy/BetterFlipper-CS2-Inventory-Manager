<script lang="ts">
  import { Backpack, Boxes, LogOut, RefreshCw } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import { page } from '$app/state';
  import { goto } from '$app/navigation';
  import { invoke } from '@tauri-apps/api/core';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import { sessionStatus } from '$lib/stores/auth';
  import { items } from '$lib/stores/inventory';
  import { steam, inventory } from '$lib/api';

  type Nav = { href: string; label: string; icon: any };
  const links: Nav[] = [
    { href: '/inventory', label: 'Inventory', icon: Backpack },
    { href: '/storage',   label: 'Storage',   icon: Boxes },
  ];

  let personaName = $state<string | null>(null);
  let avatarUrl = $state<string | null>(null);

  onMount(async () => {
    if ($sessionStatus.loggedIn) {
      try {
        const p = await invoke<{ personaName: string; avatarUrl: string }>('persona_get');
        personaName = p.personaName;
        avatarUrl = p.avatarUrl;
      } catch (e) { console.warn('persona failed', e); }
    }
  });

  async function refresh() {
    try { await inventory.refresh(); } catch (e) { console.warn(e); }
  }
  async function logout() {
    await steam.logout();
    await goto('/');
  }
  async function openBfSite() {
    try { await openUrl('https://betterflipper.com'); } catch (e) { console.warn('open url failed', e); }
  }
</script>

<aside class="w-[200px] shrink-0 border-r border-border bg-bg-elevated/60 flex flex-col">
  
  <div class="p-3 border-b border-border">
    <div class="flex items-center gap-2.5">
      {#if avatarUrl}
        <img src={avatarUrl} alt="" class="w-11 h-11 rounded-md border border-accent/30 object-cover" />
      {:else}
        <div class="w-11 h-11 rounded-md bg-gradient-to-br from-accent/30 to-accent/10 border border-accent/30 flex items-center justify-center">
          <span class="text-accent text-sm font-semibold">
            {($sessionStatus.username ?? '?').slice(0, 2).toUpperCase()}
          </span>
        </div>
      {/if}
      <div class="min-w-0">
        <div class="text-sm font-semibold truncate">{personaName ?? $sessionStatus.username ?? 'Not signed in'}</div>
        <div class="text-[11px] text-fg-dim truncate">{$sessionStatus.username ?? ''}</div>
        <div class="text-[11px] text-fg-dim flex items-center gap-1.5 mt-0.5">
          <span class="status-dot {$sessionStatus.gcReady ? 'on' : 'off'}"></span>
          {$sessionStatus.gcReady ? 'Connected' : 'Connecting…'}
        </div>
      </div>
    </div>
  </div>

  
  <div class="px-3 py-2 border-b border-border kicker flex justify-between">
    <span>Items</span>
    <span class="text-fg tabular">{$items.length}</span>
  </div>

  
  <nav class="flex-1 p-2 space-y-1">
    {#each links as l}
      {@const active = page.url.pathname.startsWith(l.href)}
      <a
        href={l.href}
        class="nav-link"
        class:active
      >
        <l.icon size={16} />
        <span>{l.label}</span>
      </a>
    {/each}
  </nav>

  
  <button class="promo" onclick={openBfSite} type="button">
    <span class="promo-tag">FREE</span>
    <span class="promo-text">More features at <span class="promo-link">betterflipper.com</span></span>
  </button>

  
  <div class="p-2 border-t border-border space-y-1">
    <button class="nav-link w-full text-left" onclick={refresh}>
      <RefreshCw size={16} />
      <span>Refresh</span>
    </button>
    <button class="nav-link w-full text-left hover:!text-danger" onclick={logout}>
      <LogOut size={16} />
      <span>Sign out</span>
    </button>
  </div>
</aside>

<style>
  .nav-link {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    padding: 0.4rem 0.6rem;
    border-radius: 0.375rem;
    font-size: 0.8125rem;
    color: var(--color-fg-muted);
    transition: background 0.12s, color 0.12s;
  }
  .nav-link:hover { background: rgba(255, 255, 255, 0.04); color: var(--color-fg); }
  .nav-link.active {
    background: var(--color-accent-soft);
    color: hsl(160 84% 75%);
    box-shadow: inset 0 0 0 1px hsla(160 84% 39% / 0.25);
  }
  .status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    display: inline-block;
  }
  .status-dot.on {
    background: var(--color-accent);
    box-shadow: 0 0 6px hsla(160 84% 39% / 0.6);
  }
  .status-dot.off {
    background: var(--color-warn);
    animation: pulse 1.4s ease-in-out infinite;
  }
  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.35; }
  }
  .promo {
    display: flex;
    align-items: center;
    gap: 8px;
    margin: 8px;
    padding: 7px 9px;
    background: transparent;
    border: 1px solid rgb(38, 42, 50);
    border-radius: 2px;
    cursor: pointer;
    text-align: left;
    transition: border-color 0.12s;
  }
  .promo:hover { border-color: var(--color-accent); }
  .promo-tag {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.1em;
    padding: 2px 5px;
    color: var(--color-accent);
    border: 1px solid var(--color-accent);
    border-radius: 2px;
    line-height: 1;
    flex-shrink: 0;
  }
  .promo-text {
    font-size: 11px;
    color: var(--color-fg-dim);
    line-height: 1.3;
  }
  .promo-link { color: var(--color-fg); }
  .promo:hover .promo-link { color: var(--color-accent); }
</style>
