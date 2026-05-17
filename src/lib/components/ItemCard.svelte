<script lang="ts">
  import type { Item } from '$lib/types';
  import { formatTradeBan, tradeBanTooltip } from '$lib/format';
  import { Boxes } from 'lucide-svelte';

  let {
    item,
    onClick = () => {},
    
    
    
    stickerHoverDelay = 250
  }: { item: Item; onClick?: (i: Item) => void; stickerHoverDelay?: number } = $props();

  const tradeBan = $derived(formatTradeBan(item.tradableAfter));
  const tradeBanFull = $derived(tradeBanTooltip(item.tradableAfter));
  const wearPct = $derived(item.paintWear == null ? null : Math.min(100, item.paintWear * 100));

  
  
  const isStickerLikeItem = $derived([1209, 1349].includes(item.defIndex));
  const showStickerSummary = $derived(item.stickers.length > 0 && !isStickerLikeItem);

  let showStickers = $state(false);
  let hoverTimer: ReturnType<typeof setTimeout> | null = null;

  function onCardEnter() {
    if (!showStickerSummary) return;
    hoverTimer = setTimeout(() => { showStickers = true; }, stickerHoverDelay);
  }
  function onCardLeave() {
    if (hoverTimer) { clearTimeout(hoverTimer); hoverTimer = null; }
    showStickers = false;
  }
</script>

<div
  class="card-wrap"
  onmouseenter={onCardEnter}
  onmouseleave={onCardLeave}
  role="presentation"
>
<button
  class="card"
  onclick={() => onClick(item)}
>
  
  <div class="rarity-strip" style="background:{item.rarityColor}"></div>

  
  {#if tradeBan}
    <div class="trade-lock">{tradeBan}</div>
  {/if}

  
  <div class="image-wrap">
    {#if item.imageUrl}
      <img
        src={item.imageUrl}
        alt={item.name}
        loading="lazy"
        draggable="false"
        ondragstart={(e) => e.preventDefault()}
      />
    {:else if item.isStorageUnit}
      <Boxes size={56} color="var(--color-fg-dim)" />
    {:else}
      <div class="text-fg-faint text-[10px] uppercase tracking-wider">no preview</div>
    {/if}

    
    {#if item.equippedCt}<span class="pill pill-ct">CT</span>{/if}
    {#if item.equippedT}<span class="pill pill-t">T</span>{/if}
  </div>

  
  <div class="footer">
    <div class="name-row">
      <div class="name">{item.customName ?? item.name}</div>
      <div class="badges">
        {#if item.stattrak}<span class="badge st">ST™</span>{/if}
        {#if item.souvenir}<span class="badge sv">SV</span>{/if}
      </div>
    </div>

    {#if item.isStorageUnit}
      <div class="meta">Storage Unit · {item.storageUnitItemCount ?? 0}/1000</div>
    {:else if item.wearName}
      <div class="meta">
        <span>{item.wearName}</span>
        {#if item.paintWear != null}<span class="tabular">{item.paintWear.toFixed(4)}</span>{/if}
      </div>
      {#if wearPct != null}
        <div class="wear-track">
          <div class="wear-fill" style="width:{wearPct}%;background:{item.rarityColor}"></div>
        </div>
      {/if}
    {/if}

    {#if showStickerSummary}
      <div class="sticker-count">
        {item.stickers.length} sticker{item.stickers.length === 1 ? '' : 's'}
      </div>
    {/if}
  </div>
</button>

{#if showStickers && showStickerSummary}
  <div class="sticker-popup" role="tooltip">
    {#each item.stickers as s}
      <div class="sticker-row">
        {#if s.imageUrl}
          <img class="sticker-img" src={s.imageUrl} alt={s.name} loading="lazy" />
        {:else}
          <div class="sticker-img placeholder"></div>
        {/if}
        <div class="sticker-meta">
          <div class="sticker-name">{s.name}</div>
          <div class="sticker-wear tabular">{Math.round((1 - s.wear) * 100)}%</div>
        </div>
      </div>
    {/each}
  </div>
{/if}
</div>

<style>
  .card-wrap {
    position: relative;
    width: 100%;
    
  }
  .card-wrap:hover { z-index: 30; }
  .card {
    position: relative;
    width: 100%;
    aspect-ratio: 1 / 1;
    display: grid;
    grid-template-rows: 2px 1fr auto;
    background: rgb(14, 17, 22);
    border: 1px solid rgb(38, 42, 50);
    border-radius: 2px;
    overflow: hidden;
    text-align: left;
    cursor: pointer;
    padding: 0;
    transition: border-color 0.12s, background 0.12s;
    animation: card-in 0.32s cubic-bezier(0.16, 1, 0.3, 1) backwards;
  }
  .card:hover {
    border-color: var(--color-accent);
    background: rgb(18, 22, 28);
  }
  @keyframes card-in {
    from { opacity: 0; transform: translateY(6px); }
    to { opacity: 1; transform: translateY(0); }
  }
  .rarity-strip { height: 2px; width: 100%; }
  .image-wrap {
    position: relative;
    display: grid;
    place-items: center;
    width: 100%;
    min-height: 0;
    background: rgb(10, 12, 16);
    overflow: hidden;
  }
  .image-wrap img {
    display: block;
    max-width: 90%;
    max-height: 90%;
    width: auto;
    height: auto;
    object-fit: contain;
    object-position: center;
    margin: auto;
    image-rendering: -webkit-optimize-contrast;
    user-select: none;
    -webkit-user-drag: none;
    pointer-events: none;
  }
  .trade-lock {
    position: absolute;
    top: 5px;
    left: 5px;
    padding: 1px 5px;
    font-size: 9px;
    font-weight: 600;
    color: rgb(248 113 113);
    background: rgba(15, 16, 20, 0.85);
    border: 1px solid rgba(239, 68, 68, 0.45);
    border-radius: 2px;
    z-index: 1;
  }
  .footer {
    padding: 8px 10px 9px;
    border-top: 1px solid rgb(28, 32, 38);
    background: rgb(11, 14, 18);
    font-family: var(--font-mono);
    line-height: 1.3;
    display: flex;
    flex-direction: column;
    gap: 3px;
    min-height: 0;
  }
  .name-row {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 4px;
  }
  .name {
    font-size: 12px;
    color: var(--color-fg);
    line-height: 1.3;
    word-break: break-word;
    overflow-wrap: anywhere;
    flex: 1;
  }
  .badges {
    display: flex;
    gap: 4px;
    flex-shrink: 0;
  }
  .badge {
    font-size: 10px;
    font-weight: 600;
    padding: 1px 4px;
    border-radius: 2px;
    line-height: 1.3;
  }
  .badge.st { color: rgb(251 146 60); background: rgba(251, 146, 60, 0.12); }
  .badge.sv { color: rgb(251 191 36); background: rgba(251, 191, 36, 0.12); }
  .meta {
    display: flex;
    justify-content: space-between;
    gap: 4px;
    font-size: 11px;
    color: var(--color-fg-dim);
  }
  .pill {
    position: absolute;
    bottom: 4px;
    padding: 0 4px;
    font-size: 9px;
    font-weight: 700;
    border-radius: 2px;
    line-height: 14px;
  }
  .pill-ct { right: 22px; background: rgb(96, 165, 250); color: white; }
  .pill-t  { right: 4px;  background: rgb(251, 191, 36); color: black; }
  .wear-track {
    margin-top: 2px;
    height: 2px;
    width: 100%;
    background: rgb(28, 32, 38);
    border-radius: 0;
    overflow: hidden;
  }
  .wear-fill { height: 100%; }
  .sticker-count {
    font-size: 10px;
    color: var(--color-fg-dim);
    margin-top: 1px;
  }
  .sticker-popup {
    position: absolute;
    top: 100%;
    left: 50%;
    transform: translateX(-50%);
    margin-top: 6px;
    z-index: 30;
    min-width: 240px;
    max-width: 320px;
    padding: 10px;
    background: rgb(11, 14, 18);
    border: 1px solid var(--color-accent);
    border-radius: 2px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.6);
    display: flex;
    flex-direction: column;
    gap: 8px;
    animation: pop-in 0.16s ease-out;
    pointer-events: none;
  }
  @keyframes pop-in {
    from { opacity: 0; transform: translate(-50%, -4px); }
    to { opacity: 1; transform: translate(-50%, 0); }
  }
  .sticker-row {
    display: grid;
    grid-template-columns: 52px 1fr;
    gap: 10px;
    align-items: center;
  }
  .sticker-img {
    width: 52px;
    height: 52px;
    object-fit: contain;
    background: rgb(15, 18, 24);
    border-radius: 2px;
  }
  .sticker-img.placeholder { background: rgb(20, 24, 30); }
  .sticker-meta {
    display: flex;
    flex-direction: column;
    gap: 1px;
    min-width: 0;
  }
  .sticker-name {
    font-size: 11px;
    color: var(--color-fg);
    line-height: 1.25;
    word-break: break-word;
  }
  .sticker-wear {
    font-size: 10px;
    color: var(--color-fg-dim);
  }
</style>
