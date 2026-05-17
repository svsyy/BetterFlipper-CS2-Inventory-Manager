<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import InventoryFilters from '$lib/components/InventoryFilters.svelte';
  import ItemGrid from '$lib/components/ItemGrid.svelte';
  import StoragePanel from '$lib/components/StoragePanel.svelte';
  import TetrisLoader from '$lib/components/TetrisLoader.svelte';
  import { inventory as inventoryApi } from '$lib/api';
  import { items, filteredItems } from '$lib/stores/inventory';
  import { sessionStatus } from '$lib/stores/auth';
  import type { Item } from '$lib/types';

  let loading = $state(true);
  let error = $state<string | null>(null);
  let activeCasket = $state<Item | null>(null);

  onMount(async () => {
    if (!$sessionStatus.loggedIn) { await goto('/'); return; }
    try {
      const list = await inventoryApi.list();
      items.set(list);
    } catch (e: any) {
      error = e?.message ?? String(e);
    } finally {
      loading = false;
    }
  });

  
  
  function onItemClick(item: Item) {
    if (item.isStorageUnit) {
      activeCasket = item;
    }
  }

  
  const movableInventory = $derived(
    $items.filter((i) => !i.isStorageUnit && !i.tradableAfter && !i.casketId)
  );
</script>

<svelte:head><title>Inventory · CS2 Inventory Manager</title></svelte:head>

<InventoryFilters />

{#if loading}
  <div class="state"><TetrisLoader label="Waiting for inventory" /></div>
{:else if error}
  <div class="state">
    <div class="text-sm text-danger">{error}</div>
    <div class="text-[11px] text-fg-faint mt-2">
      The Game Coordinator hasn't responded yet — check the app's tracing logs.
    </div>
  </div>
{:else}
  <ItemGrid items={$filteredItems} onSelect={onItemClick} />
{/if}

{#if activeCasket}
  <StoragePanel
    casket={activeCasket}
    inventory={movableInventory}
    onClose={() => (activeCasket = null)}
  />
{/if}

<style>
  .state {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    text-align: center;
  }
</style>
