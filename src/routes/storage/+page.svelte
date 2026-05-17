<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { caskets } from '$lib/api';
  import { sessionStatus } from '$lib/stores/auth';
  import { items as inventoryItems } from '$lib/stores/inventory';
  import StoragePanel from '$lib/components/StoragePanel.svelte';
  import TetrisLoader from '$lib/components/TetrisLoader.svelte';
  import { Boxes, FolderOpen, Pencil, Search } from 'lucide-svelte';
  import type { Item } from '$lib/types';

  let casketsList = $state<Item[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let query = $state('');
  let active = $state<Item | null>(null);

  let renaming = $state<string | null>(null);
  let newName = $state('');

  onMount(async () => {
    if (!$sessionStatus.loggedIn) { await goto('/'); return; }
    await load();
  });

  async function load() {
    loading = true;
    error = null;
    try {
      casketsList = await caskets.list();
    } catch (e: any) {
      error = e?.message ?? String(e);
    } finally {
      loading = false;
    }
  }

  async function saveRename(c: Item) {
    if (!newName.trim() || newName === c.customName) {
      renaming = null;
      return;
    }
    try {
      await caskets.rename(c.id, newName.trim());
      await load();
    } catch (e: any) {
      alert(e?.message ?? String(e));
    } finally {
      renaming = null;
    }
  }

  const filteredCaskets = $derived(
    casketsList.filter((c) => {
      const q = query.trim().toLowerCase();
      if (!q) return true;
      return (
        (c.customName ?? '').toLowerCase().includes(q) ||
        c.name.toLowerCase().includes(q)
      );
    })
  );

  const movableInventory = $derived(
    $inventoryItems.filter((i) => !i.isStorageUnit && !i.tradableAfter)
  );
</script>

<svelte:head><title>Storage Units · CS2 Inventory Manager</title></svelte:head>

<header class="head">
  <div>
    <div class="kicker">Storage</div>
    <h1>Storage Units · <span class="tabular text-fg-muted">{casketsList.length}</span></h1>
  </div>
  <div class="search">
    <Search size={14} class="text-fg-dim" />
    <input type="text" bind:value={query} placeholder="Search storage units…" />
  </div>
</header>

{#if loading}
  <div class="state"><TetrisLoader label="Loading storage units" /></div>
{:else if error}
  <div class="state"><div class="text-sm text-danger">{error}</div></div>
{:else if filteredCaskets.length === 0}
  <div class="state">
    <Boxes size={48} class="opacity-20" />
    <div class="text-sm text-fg-muted mt-3">No storage units yet</div>
  </div>
{:else}
  <ul class="list">
    {#each filteredCaskets as c, i (c.id)}
      <li class="row" style="animation-delay: {i * 50}ms">
        <div class="thumb">
          {#if c.imageUrl}<img src={c.imageUrl} alt={c.name} />{:else}<Boxes size={20} />{/if}
        </div>
        <div class="meta">
          {#if renaming === c.id}
            <input
              class="rename"
              bind:value={newName}
              maxlength="40"
              autofocus
              onblur={() => saveRename(c)}
              onkeydown={(e) => { if (e.key === 'Enter') saveRename(c); if (e.key === 'Escape') renaming = null; }}
            />
          {:else}
            <button class="name" onclick={() => { renaming = c.id; newName = c.customName ?? c.name; }}>
              {c.customName ?? c.name}
              <Pencil size={11} class="ml-1 inline opacity-50" />
            </button>
          {/if}
          <div class="text-[11px] text-fg-dim">Storage Unit</div>
        </div>
        <div class="count tabular">{c.storageUnitItemCount ?? 0} / 1000</div>
        <button class="open" onclick={() => (active = c)}>
          <FolderOpen size={14} /><span>Open</span>
        </button>
      </li>
    {/each}
  </ul>
{/if}

{#if active}
  <StoragePanel
    casket={active}
    inventory={movableInventory}
    onClose={() => (active = null)}
  />
{/if}

<style>
  .head {
    display: flex; align-items: center; justify-content: space-between; gap: 1rem;
    padding: 0.75rem 1rem; border-bottom: 1px solid var(--color-border);
  }
  .head h1 { font-size: 1rem; font-weight: 600; }
  .search {
    display: flex; align-items: center; gap: 0.5rem;
    padding: 0.35rem 0.625rem;
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid var(--color-border);
    border-radius: 0.375rem;
    min-width: 240px;
  }
  .search input { flex: 1; background: transparent; outline: none; font-size: 0.8125rem; color: var(--color-fg); }

  .state {
    flex: 1; display: flex; flex-direction: column;
    align-items: center; justify-content: center;
    text-align: center; gap: 0.5rem;
  }

  .list {
    flex: 1; overflow-y: auto;
    padding: 0.5rem;
    display: flex; flex-direction: column; gap: 4px;
  }
  .row {
    display: grid; grid-template-columns: 48px 1fr auto auto; gap: 0.75rem; align-items: center;
    padding: 0.5rem 0.75rem;
    background: rgba(255, 255, 255, 0.02);
    border: 1px solid var(--color-border);
    border-radius: 0.375rem;
    animation: row-in 0.4s cubic-bezier(0.16, 1, 0.3, 1) backwards;
  }
  .row:hover { background: rgba(255, 255, 255, 0.04); }
  @keyframes row-in {
    from { opacity: 0; transform: translateY(8px); }
    to { opacity: 1; transform: translateY(0); }
  }
  .thumb {
    width: 48px; height: 48px;
    display: flex; align-items: center; justify-content: center;
    background: rgba(0, 0, 0, 0.3);
    border-radius: 0.25rem;
    color: var(--color-fg-dim);
  }
  .thumb img { max-width: 90%; max-height: 90%; object-fit: contain; }
  .name {
    font-size: 0.875rem; color: var(--color-fg);
    background: none; padding: 0;
  }
  .rename {
    font-size: 0.875rem;
    background: rgba(0, 0, 0, 0.4);
    border: 1px solid var(--color-accent);
    color: var(--color-fg);
    border-radius: 3px;
    padding: 2px 6px;
  }
  .count {
    font-size: 0.75rem;
    color: var(--color-fg-muted);
    min-width: 80px; text-align: right;
  }
  .open {
    display: inline-flex; align-items: center; gap: 0.4rem;
    padding: 0.35rem 0.6rem;
    font-size: 0.75rem;
    background: var(--color-accent-soft);
    color: hsl(160 84% 75%);
    border: 1px solid hsla(160 84% 39% / 0.35);
    border-radius: 0.375rem;
  }
  .open:hover { background: hsla(160 84% 39% / 0.25); }
</style>
