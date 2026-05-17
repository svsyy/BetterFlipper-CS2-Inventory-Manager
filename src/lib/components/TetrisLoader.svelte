<script lang="ts">
  let { label = 'Loading…' }: { label?: string } = $props();
</script>

<div class="wrap">
  <div class="tetris" aria-hidden="true">
    {#each Array(7) as _, i}
      <span style="--i:{i}"></span>
    {/each}
  </div>
  <div class="label">{label}</div>
</div>

<style>
  .wrap { display: flex; flex-direction: column; align-items: center; gap: 1rem; }
  .tetris {
    display: grid;
    grid-template-columns: repeat(7, 8px);
    gap: 3px;
  }
  .tetris span {
    display: block;
    width: 8px;
    height: 18px;
    background: linear-gradient(180deg, var(--color-accent-glow), var(--color-accent));
    border-radius: 2px;
    transform: translateY(-12px);
    opacity: 0.2;
    animation: bounce 1.1s ease-in-out infinite;
    animation-delay: calc(var(--i) * 80ms);
  }
  @keyframes bounce {
    0%, 100% { transform: translateY(-12px); opacity: 0.2; }
    50%      { transform: translateY(12px);  opacity: 1;   }
  }
  .label {
    font-family: var(--font-mono);
    font-size: 0.6875rem;
    text-transform: uppercase;
    letter-spacing: 0.16em;
    color: var(--color-fg-dim);
  }
</style>
