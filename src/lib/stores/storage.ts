

import { writable } from 'svelte/store';
import type { Item, TransferProgress } from '$lib/types';

export const openCasket = writable<Item | null>(null);
export const casketContents = writable<Item[]>([]);
export const casketLoading = writable(false);
export const lastTransfer = writable<TransferProgress | null>(null);
