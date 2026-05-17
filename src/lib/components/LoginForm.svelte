<script lang="ts">
  import { steam } from '$lib/api';
  import { sessionStatus } from '$lib/stores/auth';
  import { LogIn, ShieldAlert, Loader2 } from 'lucide-svelte';
  import type { CredentialsRequest } from '$lib/types';

  let { onSuccess = () => {} }: { onSuccess?: () => void } = $props();

  let username = $state('');
  let password = $state('');
  let steamGuardCode = $state('');
  let sharedSecret = $state('');
  let remember = $state(true);

  let loading = $state(false);
  let error = $state<string | null>(null);
  let needsGuard = $state(false);

  async function submit(e: Event) {
    e.preventDefault();
    error = null;
    loading = true;
    try {
      const req: CredentialsRequest = {
        username: username.trim(),
        password,
        steamGuardCode: steamGuardCode.trim() || null,
        sharedSecret: sharedSecret.trim() || null,
        remember,
      };
      const result = await steam.login(req);
      sessionStatus.update((s) => ({
        ...s,
        loggedIn: true,
        username: result.username,
        steamId: result.steamId,
      }));
      onSuccess();
    } catch (err: any) {
      const msg = typeof err === 'string' ? err : err?.message ?? String(err);
      error = msg;
      if (/guard|2fa|two-factor/i.test(msg)) needsGuard = true;
    } finally {
      loading = false;
    }
  }
</script>

<form class="card" onsubmit={submit}>
  <header>
    <div class="kicker">Sign in to Steam</div>
    <h1>Welcome back</h1>
    <p>Your refresh token is stored in the OS keychain. Password is never written to disk.</p>
  </header>

  <label class="field">
    <span>Username</span>
    <input type="text" bind:value={username} autocomplete="username" required />
  </label>

  <label class="field">
    <span>Password</span>
    <input type="password" bind:value={password} autocomplete="current-password" required />
  </label>

  {#if needsGuard}
    <label class="field">
      <span>Steam Guard code</span>
      <input type="text" bind:value={steamGuardCode} maxlength="7" placeholder="ABCDE" />
    </label>
  {/if}

  <details>
    <summary>Have a shared_secret? (auto-generate 2FA codes)</summary>
    <label class="field">
      <span>shared_secret (base64)</span>
      <input type="text" bind:value={sharedSecret} placeholder="from Steam Mobile backup" />
    </label>
  </details>

  <label class="check">
    <input type="checkbox" bind:checked={remember} />
    <span>Remember this account (refresh token in OS keychain)</span>
  </label>

  {#if error}
    <div class="error">
      <ShieldAlert size={14} />
      <span>{error}</span>
    </div>
  {/if}

  <button class="primary" type="submit" disabled={loading || !username || !password}>
    {#if loading}<Loader2 size={14} class="animate-spin" />{:else}<LogIn size={14} />{/if}
    <span>{loading ? 'Signing in…' : 'Sign in'}</span>
  </button>

  <footer>
    <span class="text-fg-faint">Open-source · MIT · no telemetry</span>
  </footer>
</form>

<style>
  .card {
    width: 380px;
    background: var(--color-card);
    border: 1px solid var(--color-border);
    border-radius: 8px;
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 0.875rem;
  }
  header h1 { font-size: 1.25rem; font-weight: 600; margin-top: 0.25rem; }
  header p  { font-size: 0.75rem; color: var(--color-fg-dim); margin-top: 0.25rem; }
  .field { display: flex; flex-direction: column; gap: 0.25rem; font-size: 0.75rem; color: var(--color-fg-muted); }
  .field input {
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid var(--color-border);
    border-radius: 0.375rem;
    padding: 0.5rem 0.625rem;
    color: var(--color-fg);
    font-size: 0.8125rem;
    transition: border 0.15s;
  }
  .field input:focus { outline: none; border-color: hsla(160 84% 39% / 0.5); }
  details summary {
    font-size: 0.6875rem;
    color: var(--color-fg-dim);
    text-transform: uppercase;
    letter-spacing: 0.06em;
    cursor: pointer;
  }
  .check {
    display: flex; align-items: center; gap: 0.5rem;
    font-size: 0.75rem; color: var(--color-fg-muted);
  }
  .error {
    display: flex; align-items: flex-start; gap: 0.4rem;
    padding: 0.5rem 0.625rem;
    font-size: 0.75rem; color: rgb(248 113 113);
    background: rgba(239, 68, 68, 0.08);
    border: 1px solid rgba(239, 68, 68, 0.25);
    border-radius: 0.375rem;
  }
  .primary {
    display: inline-flex; align-items: center; justify-content: center; gap: 0.5rem;
    padding: 0.55rem 0.75rem;
    background: var(--color-accent);
    color: white;
    font-weight: 600;
    font-size: 0.8125rem;
    border-radius: 0.375rem;
    transition: background 0.15s, box-shadow 0.15s;
  }
  .primary:hover:not(:disabled) {
    background: var(--color-accent-glow);
    box-shadow: 0 0 16px hsla(160 84% 39% / 0.45);
  }
  .primary:disabled { opacity: 0.5; cursor: not-allowed; }
  footer { text-align: center; font-size: 0.625rem; }
  :global(.animate-spin) { animation: spin 0.8s linear infinite; }
  @keyframes spin { to { transform: rotate(360deg); } }
</style>
