

import { derived, writable } from 'svelte/store';
import type { Item } from '$lib/types';

export type ItemFilterRarity = 'all' | 1 | 2 | 3 | 4 | 5 | 6;
export type ItemSortKey = 'position' | 'name' | 'rarity' | 'wear' | 'price';
export type ItemTypeFilter =
  | 'normal'
  | 'stattrak'
  | 'souvenir'
  | 'containers'
  | 'storage';
export type TradabilityFilter = 'tradable' | 'locked';

export const items = writable<Item[]>([]);
export const search = writable('');
export const rarityFilter = writable<ItemFilterRarity>('all');
export const sortKey = writable<ItemSortKey>('position');
export const sortDir = writable<'asc' | 'desc'>('asc');
export const typeFilters = writable<Set<ItemTypeFilter>>(new Set());
export const tradabilityFilters = writable<Set<TradabilityFilter>>(new Set());
export const stacked = writable(false);

export const filteredItems = derived(
  [items, search, rarityFilter, sortKey, sortDir, typeFilters, tradabilityFilters],
  ([$items, $search, $rar, $sk, $sd, $tf, $tradf]) => {
    const q = $search.trim().toLowerCase();
    let out = $items.filter((i) => {
      if (q && !(i.name.toLowerCase().includes(q) || (i.customName ?? '').toLowerCase().includes(q))) return false;
      if ($rar !== 'all' && i.rarity !== $rar) return false;

      if ($tf.size) {
        const hits = (
          ($tf.has('normal')     && !i.stattrak && !i.souvenir && !i.isContainer && !i.isStorageUnit) ||
          ($tf.has('stattrak')   && i.stattrak) ||
          ($tf.has('souvenir')   && i.souvenir) ||
          ($tf.has('containers') && i.isContainer) ||
          ($tf.has('storage')    && i.isStorageUnit)
        );
        if (!hits) return false;
      }

      if ($tradf.size) {
        const tradable = !i.tradableAfter;
        if ($tradf.has('tradable') && !tradable) return false;
        if ($tradf.has('locked')   && tradable)  return false;
      }
      return true;
    });

    out.sort((a, b) => {
      let cmp = 0;
      switch ($sk) {
        case 'name':     cmp = a.name.localeCompare(b.name); break;
        case 'rarity':   cmp = a.rarity - b.rarity; break;
        case 'wear':     cmp = (a.paintWear ?? 1) - (b.paintWear ?? 1); break;
        case 'price':    cmp = 0; break; 
        default:         cmp = a.position - b.position;
      }
      return $sd === 'asc' ? cmp : -cmp;
    });

    return out;
  }
);
