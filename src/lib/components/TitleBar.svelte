<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { Minus, Square, X } from 'lucide-svelte';

  const win = getCurrentWindow();

  async function minimize() { await win.minimize(); }
  async function toggleMaximize() { await win.toggleMaximize(); }
  async function close() { await win.close(); }
</script>

<header
  data-tauri-drag-region
  class="h-12 px-3 flex items-center gap-3 border-b border-border bg-bg-elevated/70 glass select-none shrink-0"
>
  <div class="flex items-center gap-2 text-[11px] tracking-wider uppercase text-fg-muted">
    <img src="/bf-logo.png" alt="BetterFlipper" class="bf-logo" />
    <span class="font-semibold">BetterFlipper</span>
    <span class="text-fg-faint">Rust Version · <span class="text-accent">FREE</span></span>
  </div>
  <div class="flex-1"></div>
  <div class="flex items-center gap-0.5" data-tauri-no-drag>
    <button class="titlebar-btn" onclick={minimize}><Minus size={14} /></button>
    <button class="titlebar-btn" onclick={toggleMaximize}><Square size={12} /></button>
    <button class="titlebar-btn titlebar-btn-close" onclick={close}><X size={14} /></button>
  </div>
</header>

<style>
  .titlebar-btn {
    width: 32px;
    height: 24px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: var(--color-fg-muted);
    background: transparent;
    transition: color 0.18s cubic-bezier(0.22, 1, 0.36, 1);
  }
  .titlebar-btn:hover,
  .titlebar-btn:focus-visible {
    color: var(--color-fg);
    outline: none;
  }
  .titlebar-btn-close:hover,
  .titlebar-btn-close:focus-visible {
    color: var(--color-danger);
  }
  .bf-logo {
    height: 36px;
    width: auto;
    image-rendering: -webkit-optimize-contrast;
  }
</style>
