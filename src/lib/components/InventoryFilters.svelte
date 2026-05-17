<script lang="ts">
  import { Search, ArrowUp, ArrowDown } from 'lucide-svelte';
  import Dropdown from './Dropdown.svelte';
  import {
    search, rarityFilter, sortKey, sortDir, typeFilters, tradabilityFilters,
    type ItemFilterRarity, type ItemSortKey, type ItemTypeFilter, type TradabilityFilter
  } from '$lib/stores/inventory';

  const rarityOptions = [
    { label: 'All',         value: 'all' as ItemFilterRarity, color: undefined },
    { label: 'Consumer',    value: 1 as ItemFilterRarity,     color: '#b0c3d9' },
    { label: 'Industrial',  value: 2 as ItemFilterRarity,     color: '#5e98d9' },
    { label: 'Mil-Spec',    value: 3 as ItemFilterRarity,     color: '#4b69ff' },
    { label: 'Restricted',  value: 4 as ItemFilterRarity,     color: '#8847ff' },
    { label: 'Classified',  value: 5 as ItemFilterRarity,     color: '#d32ce6' },
    { label: 'Covert',      value: 6 as ItemFilterRarity,     color: '#eb4b4b' },
  ];

  const sortOptions = [
    { label: 'Position', value: 'position' as ItemSortKey },
    { label: 'Name',     value: 'name'     as ItemSortKey },
    { label: 'Rarity',   value: 'rarity'   as ItemSortKey },
    { label: 'Wear',     value: 'wear'     as ItemSortKey },
  ];

  function toggle<T>(store: typeof typeFilters, val: T) {
    store.update((s) => {
      const next = new Set(s);
      if (next.has(val as any)) next.delete(val as any); else next.add(val as any);
      return next;
    });
  }
</script>

<div class="filters">
  <div class="row">
    <div class="search">
      <Search size={14} class="text-fg-dim" />
      <input
        type="text"
        placeholder="Search items…"
        bind:value={$search}
      />
    </div>

    <Dropdown
      options={rarityOptions}
      bind:value={$rarityFilter}
      label="Rarity"
    />

    <Dropdown
      options={sortOptions}
      bind:value={$sortKey}
      label="Sort"
    />

    <button class="icon-btn" onclick={() => sortDir.update((d) => (d === 'asc' ? 'desc' : 'asc'))} title="Toggle sort direction">
      {#if $sortDir === 'asc'}<ArrowUp size={14} />{:else}<ArrowDown size={14} />{/if}
      <span class="text-[11px] uppercase">{$sortDir}</span>
    </button>
  </div>

  <div class="row toggles">
    <div class="seg">
      <span class="kicker pr-2">Tradability</span>
      {#each ['tradable','locked'] as v}
        <button
          class="seg-btn"
          class:active={$tradabilityFilters.has(v as TradabilityFilter)}
          onclick={() => toggle(tradabilityFilters, v)}
        >{v}</button>
      {/each}
    </div>
    <div class="seg">
      <span class="kicker pr-2">Type</span>
      {#each ['normal','stattrak','souvenir','containers','storage'] as v}
        <button
          class="seg-btn"
          class:active={$typeFilters.has(v as ItemTypeFilter)}
          onclick={() => toggle(typeFilters, v)}
        >{v}</button>
      {/each}
    </div>
  </div>
</div>

<style>
  .filters { padding: 0.5rem 0.75rem; border-bottom: 1px solid var(--color-border); display: flex; flex-direction: column; gap: 0.5rem; }
  .row { display: flex; align-items: center; gap: 0.5rem; flex-wrap: wrap; }
  .search {
    flex: 1; min-width: 200px; display: flex; align-items: center; gap: 0.5rem;
    padding: 6px 10px;
    background: rgb(14, 17, 22);
    border: 1px solid rgb(38, 42, 50);
    border-radius: 2px;
  }
  .search input { flex: 1; background: transparent; outline: none; font-size: 12px; color: var(--color-fg); }
  .icon-btn {
    display: inline-flex; align-items: center; gap: 6px;
    padding: 6px 10px;
    background: rgb(14, 17, 22);
    border: 1px solid rgb(38, 42, 50);
    border-radius: 2px;
    color: var(--color-fg-muted);
    cursor: pointer;
    transition: border-color 0.12s, color 0.12s;
  }
  .icon-btn:hover { border-color: var(--color-accent); color: var(--color-fg); }
  .toggles { padding-top: 0.125rem; }
  .seg { display: flex; align-items: center; gap: 0.25rem; }
  .seg-btn {
    padding: 4px 10px;
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--color-fg-dim);
    background: rgb(14, 17, 22);
    border: 1px solid rgb(38, 42, 50);
    border-radius: 2px;
    cursor: pointer;
    transition: border-color 0.12s, color 0.12s;
  }
  .seg-btn:hover { border-color: var(--color-accent); color: var(--color-fg); }
  .seg-btn.active {
    color: hsl(160 84% 75%);
    background: var(--color-accent-soft);
    border-color: hsla(160 84% 39% / 0.45);
  }
</style>
