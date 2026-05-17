<script lang="ts">
  import type { Item } from '$lib/types';
  import ItemCard from './ItemCard.svelte';
  import { Package } from 'lucide-svelte';

  let {
    items,
    onSelect = () => {}
  }: { items: Item[]; onSelect?: (i: Item) => void } = $props();

  
  
  let width = $state(0);
  const cols = $derived(
    width >= 1600 ? 6 :
    width >= 1280 ? 5 :
    width >= 1024 ? 4 :
    width >= 768  ? 3 :
    width >= 640  ? 3 : 2
  );
</script>

<div class="grid-container" bind:clientWidth={width}>
  {#if items.length === 0}
    <div class="empty">
      <Package size={48} class="opacity-20" />
      <div class="text-sm text-fg-muted mt-3">No items to show</div>
      <div class="text-[11px] text-fg-faint">
        Sign in to Steam and wait for the Game Coordinator to populate your inventory.
      </div>
    </div>
  {:else}
    <div class="grid" style="grid-template-columns: repeat({cols}, minmax(0, 1fr));">
      {#each items as item (item.id)}
        <ItemCard {item} onClick={onSelect} />
      {/each}
    </div>
  {/if}
</div>

<style>
  .grid-container {
    flex: 1;
    overflow-y: auto;
    padding: 0.5rem;
  }
  .grid {
    display: grid;
    gap: 6px;
  }
  .empty {
    height: 100%;
    min-height: 240px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
  }
</style>
