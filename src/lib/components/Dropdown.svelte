<script lang="ts" generics="T">
  import { ChevronDown } from 'lucide-svelte';

  type Option = { label: string; value: T; color?: string };

  let {
    options,
    value = $bindable<T>(),
    label = '',
    onChange = () => {}
  }: {
    options: Option[];
    value: T;
    label?: string;
    onChange?: (v: T) => void;
  } = $props();

  let open = $state(false);
  let trigger: HTMLButtonElement | undefined = $state();

  const selected = $derived(options.find((o) => o.value === value) ?? options[0]);

  function toggle() { open = !open; }
  function pick(v: T) {
    value = v;
    open = false;
    onChange(v);
  }

  function handleOutside(e: MouseEvent) {
    if (!open) return;
    if (trigger && !trigger.contains(e.target as Node)) {
      const menu = document.getElementById('dropdown-menu-active');
      if (menu && !menu.contains(e.target as Node)) open = false;
    }
  }
</script>

<svelte:window onmousedown={handleOutside} />

<div class="dd">
  <button class="dd-trigger" bind:this={trigger} onclick={toggle} class:open>
    {#if label}<span class="dd-label">{label}:</span>{/if}
    {#if selected?.color}
      <span class="dd-swatch" style="background:{selected.color}"></span>
    {/if}
    <span class="dd-current">{selected?.label ?? ''}</span>
    <ChevronDown size={12} class="dd-chevron {open ? 'open' : ''}" />
  </button>

  {#if open}
    <div id="dropdown-menu-active" class="dd-menu" role="listbox">
      <div class="dd-menu-inner">
        {#each options as opt, i}
          <button
            class="dd-option"
            class:selected={opt.value === value}
            onclick={() => pick(opt.value)}
            role="option"
            aria-selected={opt.value === value}
            style="animation-delay: {i * 22}ms"
          >
            {#if opt.color}
              <span class="dd-swatch" style="background:{opt.color}"></span>
            {/if}
            <span>{opt.label}</span>
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .dd { position: relative; }
  .dd-trigger {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 10px;
    font-size: 12px;
    color: var(--color-fg);
    background: rgb(14, 17, 22);
    border: 1px solid rgb(38, 42, 50);
    border-radius: 2px;
    cursor: pointer;
    transition: border-color 0.12s;
    min-width: 110px;
  }
  .dd-trigger:hover, .dd-trigger.open {
    border-color: var(--color-accent);
  }
  .dd-label {
    font-size: 10px;
    color: var(--color-fg-dim);
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }
  .dd-current { flex: 1; text-align: left; }
  .dd-swatch {
    width: 8px;
    height: 8px;
    display: inline-block;
    border-radius: 0;
    flex-shrink: 0;
  }
  :global(.dd-chevron) {
    color: var(--color-fg-dim);
    transition: transform 0.18s ease;
  }
  :global(.dd-chevron.open) {
    transform: rotate(180deg);
    color: var(--color-accent);
  }
  .dd-menu {
    position: absolute;
    top: calc(100% + 4px);
    left: 0;
    right: 0;
    z-index: 40;
    background: rgb(11, 14, 18);
    border: 1px solid var(--color-accent);
    border-radius: 2px;
    overflow: hidden;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.5);
    
    transform-origin: top;
    animation: dd-expand 0.28s cubic-bezier(0.16, 1, 0.3, 1);
  }
  .dd-menu-inner {
    padding: 4px;
    display: flex;
    flex-direction: column;
    gap: 2px;
    max-height: 320px;
    overflow-y: auto;
  }
  @keyframes dd-expand {
    0% {
      opacity: 0;
      transform: scaleY(0) translateY(-4px);
      max-height: 0;
    }
    100% {
      opacity: 1;
      transform: scaleY(1) translateY(0);
      max-height: 320px;
    }
  }
  .dd-option {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 8px;
    font-size: 12px;
    color: var(--color-fg-muted);
    background: transparent;
    border: none;
    border-radius: 2px;
    cursor: pointer;
    text-align: left;
    width: 100%;
    transition: background 0.12s, color 0.12s;
    
    opacity: 0;
    transform: translateX(-4px);
    animation: dd-option-in 0.22s ease-out forwards;
  }
  @keyframes dd-option-in {
    to { opacity: 1; transform: translateX(0); }
  }
  .dd-option:hover {
    background: rgb(18, 22, 28);
    color: var(--color-fg);
  }
  .dd-option.selected {
    color: var(--color-accent);
    background: var(--color-accent-soft);
  }
</style>
