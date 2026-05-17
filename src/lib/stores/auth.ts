

import { writable, derived } from 'svelte/store';
import type { LoginStage, LoginStatusPayload, SessionStatus } from '$lib/types';

export const sessionStatus = writable<SessionStatus>({
  loggedIn: false,
  username: null,
  steamId: null,
  gcReady: false,
  inventoryCount: 0,
  savedAccounts: []
});

export const loginStage = writable<LoginStage>('idle');
export const loginMessage = writable<string | null>(null);

export const isAuthenticated = derived(sessionStatus, ($s) => $s.loggedIn);
export const isReady = derived(
  [sessionStatus, loginStage],
  ([$s, $stage]) => $s.loggedIn && ($s.gcReady || $stage === 'ready')
);

export function applyLoginStatus(p: LoginStatusPayload) {
  loginStage.set(p.stage);
  loginMessage.set(p.message);
}
