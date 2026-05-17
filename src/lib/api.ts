

import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type {
  CredentialsRequest,
  Item,
  LoginResult,
  LoginStatusPayload,
  SessionStatus,
  TransferProgress
} from './types';

export const steam = {
  sessionStatus: () => invoke<SessionStatus>('steam_session_status'),
  login: (req: CredentialsRequest) => invoke<LoginResult>('steam_login', { req }),
  loginWithToken: (username: string) =>
    invoke<LoginResult>('steam_login_with_token', { username }),
  logout: () => invoke<void>('steam_logout'),
  savedAccounts: () => invoke<string[]>('list_saved_accounts')
};

export const inventory = {
  list: () => invoke<Item[]>('inventory_get'),
  refresh: () => invoke<number>('inventory_refresh')
};

export const caskets = {
  list: () => invoke<Item[]>('casket_list'),
  contents: (casketId: string) => invoke<Item[]>('casket_contents', { casketId }),
  add: (casketId: string, itemId: string) =>
    invoke<void>('casket_add', { casketId, itemId }),
  remove: (casketId: string, itemId: string) =>
    invoke<void>('casket_remove', { casketId, itemId }),
  rename: (casketId: string, newName: string) =>
    invoke<void>('casket_rename', { casketId, newName })
};

export function onLoginStatus(cb: (s: LoginStatusPayload) => void): Promise<UnlistenFn> {
  return listen<LoginStatusPayload>('steam://login-status', (e) => cb(e.payload));
}

export function onInventoryUpdated(cb: (items: Item[]) => void): Promise<UnlistenFn> {
  return listen<Item[]>('inventory://updated', (e) => cb(e.payload));
}

export function onCasketProgress(cb: (p: TransferProgress) => void): Promise<UnlistenFn> {
  return listen<TransferProgress>('casket://transfer-progress', (e) => cb(e.payload));
}
