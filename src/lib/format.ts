

export function formatFloat(wear: number | null): string {
  if (wear === null || Number.isNaN(wear)) return '—';
  return wear.toFixed(6);
}

export function formatItemCount(n: number): string {
  if (n < 1000) return n.toString();
  return (n / 1000).toFixed(n < 10_000 ? 1 : 0) + 'k';
}

export function formatTradeBan(tradableAfter: string | null): string | null {
  if (!tradableAfter) return null;
  const date = new Date(tradableAfter);
  if (Number.isNaN(date.getTime())) return null;
  const ms = date.getTime() - Date.now();
  if (ms <= 0) return null;
  const days = Math.floor(ms / 86_400_000);
  if (days >= 1) return `${days}d`;
  const hours = Math.floor(ms / 3_600_000);
  return `${hours}h`;
}

export function tradeBanTooltip(tradableAfter: string | null): string | null {
  if (!tradableAfter) return null;
  const date = new Date(tradableAfter);
  if (Number.isNaN(date.getTime())) return null;
  return `Tradable on ${date.toLocaleDateString()} ${date.toLocaleTimeString()}`;
}

export function shortName(name: string, max = 38): string {
  if (name.length <= max) return name;
  return name.slice(0, max - 1) + '…';
}
