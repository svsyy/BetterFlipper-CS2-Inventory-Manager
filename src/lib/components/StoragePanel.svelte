<script lang="ts">
  import type { Item, TransferProgress } from '$lib/types';
  import { caskets } from '$lib/api';
  import { ArrowLeft, ArrowRight, CheckSquare, Loader2, Lock, Square, X } from 'lucide-svelte';
  import ItemCard from './ItemCard.svelte';

  let {
    casket,
    inventory,
    onClose
  }: {
    casket: Item;
    inventory: Item[];
    onClose: () => void;
  } = $props();

  let contents = $state<Item[]>([]);
  let loadingContents = $state(true);
  let error = $state<string | null>(null);

  let selectedInventory = $state(new Set<string>());
  let selectedStorage = $state(new Set<string>());

  let progress = $state<TransferProgress | null>(null);

  
  const INVENTORY_CAP = 1000;
  const inventoryUsed = $derived(inventory.length);
  const freeSlots = $derived(Math.max(0, INVENTORY_CAP - inventoryUsed));
  const moveBackBlocked = $derived(freeSlots < selectedStorage.size);

  
  
  
  let dragColumn = $state<'inv' | 'sto' | null>(null);
  let dragMode = $state<'add' | 'remove'>('add');

  $effect(() => {
    void load();
  });

  async function load() {
    loadingContents = true;
    error = null;
    try {
      contents = await caskets.contents(casket.id);
    } catch (e: any) {
      error = e?.message ?? String(e);
    } finally {
      loadingContents = false;
    }
  }

  function setSelected(set: Set<string>, id: string, on: boolean): Set<string> {
    const next = new Set(set);
    if (on) next.add(id); else next.delete(id);
    return next;
  }

  
  
  
  function isLocked(item: Item): boolean { return !item.moveable; }

  function startDrag(column: 'inv' | 'sto', item: Item, e: MouseEvent) {
    if (e.button !== 0) return;
    if (isLocked(item)) return;
    const set = column === 'inv' ? selectedInventory : selectedStorage;
    dragMode = set.has(item.id) ? 'remove' : 'add';
    dragColumn = column;
    applyDrag(column, item);
  }

  function applyDrag(column: 'inv' | 'sto', item: Item) {
    if (dragColumn !== column) return;
    if (isLocked(item)) return;
    const on = dragMode === 'add';
    if (column === 'inv') {
      selectedInventory = setSelected(selectedInventory, item.id, on);
    } else {
      selectedStorage = setSelected(selectedStorage, item.id, on);
    }
  }

  function endDrag() { dragColumn = null; }

  function selectAll(column: 'inv' | 'sto') {
    if (column === 'inv') {
      selectedInventory = new Set(inventory.filter((i) => !isLocked(i)).map((i) => i.id));
    } else {
      selectedStorage = new Set(contents.filter((i) => !isLocked(i)).map((i) => i.id));
    }
  }
  function deselectAll(column: 'inv' | 'sto') {
    if (column === 'inv') selectedInventory = new Set();
    else selectedStorage = new Set();
  }

  async function moveToStorage() {
    const ids = Array.from(selectedInventory);
    if (!ids.length) return;
    progress = { done: 0, total: ids.length, currentItem: null, failed: 0 };
    for (const id of ids) {
      progress = { ...progress, currentItem: id };
      try {
        await caskets.add(casket.id, id);
        progress.done += 1;
      } catch (e) {
        progress.failed += 1;
      }
    }
    progress = null;
    selectedInventory = new Set();
    await load();
  }

  async function moveToInventory() {
    const ids = Array.from(selectedStorage);
    if (!ids.length) return;
    progress = { done: 0, total: ids.length, currentItem: null, failed: 0 };
    for (const id of ids) {
      progress = { ...progress, currentItem: id };
      try {
        await caskets.remove(casket.id, id);
        progress.done += 1;
      } catch (e) {
        progress.failed += 1;
      }
    }
    progress = null;
    selectedStorage = new Set();
    await load();
  }
</script>

<svelte:window onmouseup={endDrag} />

<div class="overlay" role="dialog" aria-modal="true">
  <header class="header">
    <div class="header-left">
      <div class="kicker">Storage Unit</div>
      <h2>{casket.customName ?? casket.name}</h2>
      <div class="header-counts">
        <span class="tabular">Storage: <strong>{contents.length}/1000</strong></span>
        <span class="sep">·</span>
        <span class="tabular" class:warn={freeSlots < 50}>
          Inventory: <strong>{inventoryUsed}/{INVENTORY_CAP}</strong>
          <span class="text-fg-faint">({freeSlots} free)</span>
        </span>
      </div>
    </div>
    <button class="close" onclick={onClose}><X size={16} /></button>
  </header>

  {#if progress}
    <div class="progress">
      <div class="bar"><span style="width:{(progress.done / Math.max(1, progress.total)) * 100}%"></span></div>
      <div class="meta tabular">
        {progress.done}/{progress.total}
        {#if progress.failed}<span class="text-danger">· {progress.failed} failed</span>{/if}
      </div>
    </div>
  {/if}

  <div class="columns">
    
    <section class="col">
      <header class="col-head">
        <div class="col-head-left">
          <span class="kicker">Inventory</span>
          <span class="count tabular">{selectedInventory.size}/{inventory.length}</span>
        </div>
        <div class="col-head-right">
          <button class="sel-btn" onclick={() => selectAll('inv')} title="Select all">
            <CheckSquare size={13} /><span>All</span>
          </button>
          <button class="sel-btn" onclick={() => deselectAll('inv')} title="Deselect all" disabled={!selectedInventory.size}>
            <Square size={13} /><span>None</span>
          </button>
        </div>
      </header>
      <div class="grid">
        {#each inventory as item (item.id)}
          <div
            class="sel-card"
            class:selected={selectedInventory.has(item.id)}
            class:locked={isLocked(item)}
            onmousedown={(e) => startDrag('inv', item, e)}
            onmouseenter={() => dragColumn === 'inv' && applyDrag('inv', item)}
            role="checkbox"
            tabindex="0"
            aria-checked={selectedInventory.has(item.id)}
            title={isLocked(item) ? 'Locked — cannot be moved' : ''}
          >
            <ItemCard {item} stickerHoverDelay={2000} />
            {#if isLocked(item)}
              <div class="lock-overlay"><Lock size={18} /><span>LOCKED</span></div>
            {/if}
          </div>
        {/each}
      </div>
    </section>

    
    <div class="arrows">
      <button class="arrow" disabled={!selectedInventory.size || !!progress} onclick={moveToStorage} title="Move to storage">
        <ArrowRight size={18} />
      </button>
      <button
        class="arrow"
        disabled={!selectedStorage.size || !!progress || moveBackBlocked}
        onclick={moveToInventory}
        title={moveBackBlocked ? `Inventory full — only ${freeSlots} slots free` : 'Move to inventory'}
      >
        <ArrowLeft size={18} />
      </button>
      {#if moveBackBlocked}
        <div class="full-warn" title="Inventory is full">FULL</div>
      {/if}
    </div>

    
    <section class="col">
      <header class="col-head">
        <div class="col-head-left">
          <span class="kicker">In Storage</span>
          <span class="count tabular">{selectedStorage.size}/{contents.length}</span>
        </div>
        <div class="col-head-right">
          <button class="sel-btn" onclick={() => selectAll('sto')} title="Select all" disabled={contents.length === 0}>
            <CheckSquare size={13} /><span>All</span>
          </button>
          <button class="sel-btn" onclick={() => deselectAll('sto')} title="Deselect all" disabled={!selectedStorage.size}>
            <Square size={13} /><span>None</span>
          </button>
        </div>
      </header>
      {#if loadingContents}
        <div class="loading"><Loader2 size={20} class="animate-spin" /></div>
      {:else if error}
        <div class="error">{error}</div>
      {:else}
        <div class="grid">
          {#each contents as item (item.id)}
            <div
              class="sel-card"
              class:selected={selectedStorage.has(item.id)}
              class:locked={isLocked(item)}
              onmousedown={(e) => startDrag('sto', item, e)}
              onmouseenter={() => dragColumn === 'sto' && applyDrag('sto', item)}
              role="checkbox"
              tabindex="0"
              aria-checked={selectedStorage.has(item.id)}
              title={isLocked(item) ? 'Locked — cannot be moved' : ''}
            >
              <ItemCard {item} stickerHoverDelay={1000} />
              {#if isLocked(item)}
                <div class="lock-overlay"><Lock size={18} /><span>LOCKED</span></div>
              {/if}
            </div>
          {/each}
        </div>
      {/if}
    </section>
  </div>
</div>

<style>
  .overlay {
    position: fixed; inset: 0; z-index: 50;
    background: rgb(8, 10, 14);
    display: flex; flex-direction: column;
  }
  .header {
    display: flex; align-items: flex-start; justify-content: space-between;
    padding: 0.875rem 1rem; border-bottom: 1px solid var(--color-border);
  }
  .header h2 { font-size: 1rem; font-weight: 600; }
  .header-left { display: flex; flex-direction: column; gap: 2px; }
  .header-counts {
    display: flex;
    gap: 8px;
    align-items: center;
    font-size: 11px;
    color: var(--color-fg-dim);
    margin-top: 2px;
  }
  .header-counts strong { color: var(--color-fg); font-weight: 600; }
  .header-counts .sep { color: var(--color-fg-faint); }
  .header-counts .warn strong { color: rgb(251 146 60); }
  .full-warn {
    margin-top: 4px;
    padding: 2px 6px;
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.08em;
    color: rgb(248 113 113);
    background: rgba(239, 68, 68, 0.12);
    border: 1px solid rgba(239, 68, 68, 0.35);
    border-radius: 2px;
  }
  .close {
    width: 32px; height: 32px;
    display: inline-flex; align-items: center; justify-content: center;
    border-radius: 2px;
    color: var(--color-fg-muted);
  }
  .close:hover { background: rgb(20, 24, 30); color: var(--color-fg); }

  .progress { padding: 0.5rem 1rem; border-bottom: 1px solid var(--color-border); }
  .bar {
    height: 3px; background: rgb(28, 32, 38); overflow: hidden;
  }
  .bar span { display: block; height: 100%; background: var(--color-accent); transition: width 0.15s; }
  .meta { font-size: 0.6875rem; color: var(--color-fg-dim); margin-top: 0.25rem; }

  .columns {
    flex: 1; display: grid;
    grid-template-columns: 1fr auto 1fr;
    overflow: hidden;
  }
  .col { display: flex; flex-direction: column; overflow: hidden; }
  .col-head {
    display: flex; align-items: center; justify-content: space-between;
    padding: 0.5rem 0.75rem;
    border-bottom: 1px solid var(--color-border);
    background: rgb(11, 14, 18);
  }
  .col-head-left { display: flex; align-items: baseline; gap: 0.5rem; }
  .col-head-right { display: flex; gap: 4px; }
  .count { font-size: 11px; color: var(--color-fg-dim); }
  .sel-btn {
    display: inline-flex; align-items: center; gap: 4px;
    padding: 3px 7px;
    font-size: 11px;
    color: var(--color-fg-muted);
    background: rgb(15, 18, 24);
    border: 1px solid rgb(28, 32, 38);
    border-radius: 2px;
  }
  .sel-btn:hover:not(:disabled) {
    color: var(--color-fg);
    border-color: var(--color-accent);
  }
  .sel-btn:disabled { opacity: 0.4; cursor: not-allowed; }
  .grid {
    flex: 1; overflow-y: auto; padding: 8px;
    display: grid; gap: 8px;
    grid-template-columns: repeat(auto-fill, minmax(170px, 1fr));
    user-select: none;
  }
  .sel-card {
    position: relative;
    padding: 0;
    background: none;
    border-radius: 2px;
    border: 2px solid transparent;
    cursor: pointer;
  }
  .sel-card.selected { border-color: var(--color-accent); }
  .sel-card.locked { cursor: not-allowed; }
  .sel-card.locked > :global(.card) {
    opacity: 0.42;
    filter: grayscale(0.6);
  }
  .lock-overlay {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 4px;
    background: rgba(8, 10, 14, 0.55);
    color: rgb(248 113 113);
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.12em;
    pointer-events: none;
    border-radius: 2px;
  }

  .arrows {
    width: 56px; display: flex; flex-direction: column; align-items: center;
    justify-content: center; gap: 8px;
    border-left: 1px solid var(--color-border);
    border-right: 1px solid var(--color-border);
    background: rgb(10, 12, 16);
  }
  .arrow {
    width: 36px; height: 36px;
    display: inline-flex; align-items: center; justify-content: center;
    background: rgb(15, 18, 24);
    border: 1px solid rgb(28, 32, 38);
    border-radius: 2px;
    color: var(--color-fg-muted);
  }
  .arrow:hover:not(:disabled) {
    background: var(--color-accent-soft);
    color: var(--color-accent);
    border-color: var(--color-accent);
  }
  .arrow:disabled { opacity: 0.25; cursor: not-allowed; }

  .loading { flex: 1; display: flex; align-items: center; justify-content: center; }
  .error {
    margin: 0.5rem;
    padding: 0.5rem 0.625rem;
    font-size: 0.75rem;
    color: rgb(248 113 113);
    background: rgba(239, 68, 68, 0.06);
    border: 1px solid rgba(239, 68, 68, 0.25);
    border-radius: 2px;
  }
</style>
