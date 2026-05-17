

export interface StickerInfo {
  slot: number;
  name: string;
  imageUrl: string | null;
  wear: number;
}

export interface Item {
  id: string;
  defIndex: number;
  name: string;
  customName: string | null;
  imageUrl: string | null;
  paintIndex: number | null;
  paintSeed: number | null;
  paintWear: number | null;
  wearName: string | null;
  rarity: number;
  rarityName: string;
  rarityColor: string;
  stattrak: boolean;
  stattrakCount: number | null;
  souvenir: boolean;
  stickers: StickerInfo[];
  moveable: boolean;
  tradableAfter: string | null;
  equippedCt: boolean;
  equippedT: boolean;
  position: number;
  origin: number;
  isContainer: boolean;
  isStorageUnit: boolean;
  storageUnitItemCount: number | null;
  casketId: string | null;
  casketInternalId: string | null;
  originalId: string | null;
  collection: string | null;
  pattern: PatternInfo | null;
}

export interface PatternInfo {
  tier: string;
  tierColor: string;
  rarity: 'common' | 'uncommon' | 'rare' | 'legendary';
}

export interface SessionStatus {
  loggedIn: boolean;
  username: string | null;
  steamId: string | null;
  gcReady: boolean;
  inventoryCount: number;
  savedAccounts: string[];
}

export interface CredentialsRequest {
  username: string;
  password: string;
  steamGuardCode?: string | null;
  sharedSecret?: string | null;
  remember: boolean;
}

export interface LoginResult {
  steamId: string;
  username: string;
  personaName: string | null;
  remembered: boolean;
}

export type LoginStage =
  | 'idle'
  | 'connecting'
  | 'authenticating'
  | 'awaiting-guard'
  | 'logging-in'
  | 'waiting-gc'
  | 'loading-inventory'
  | 'ready'
  | 'error';

export interface LoginStatusPayload {
  stage: LoginStage;
  message: string | null;
}

export interface TransferProgress {
  done: number;
  total: number;
  currentItem: string | null;
  failed: number;
}
