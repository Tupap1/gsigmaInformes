<script lang="ts">
  import { toasts } from '$lib/stores/toasts.svelte';
  import { fade, slide } from 'svelte/transition';
</script>

<div class="toast-container">
  {#each toasts.list as toast (toast.id)}
    <div 
      class="toast-card {toast.type}" 
      in:slide={{ duration: 250 }} 
      out:fade={{ duration: 150 }}
    >
      <span class="toast-icon">
        {#if toast.type === 'success'}
          <svg class="toast-svg success-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
            <polyline points="22 4 12 14.01 9 11.01"/>
          </svg>
        {:else if toast.type === 'error'}
          <svg class="toast-svg error-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="10"/>
            <line x1="15" y1="9" x2="9" y2="15"/>
            <line x1="9" y1="9" x2="15" y2="15"/>
          </svg>
        {:else}
          <svg class="toast-svg info-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="10"/>
            <line x1="12" y1="16" x2="12" y2="12"/>
            <line x1="12" y1="8" x2="12.01" y2="8"/>
          </svg>
        {/if}
      </span>
      <div class="toast-content">
        {toast.message}
      </div>
      <button class="toast-close" onclick={() => toasts.dismiss(toast.id)}>
        &times;
      </button>
    </div>
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    top: 24px;
    right: 24px;
    z-index: 9999;
    display: flex;
    flex-direction: column;
    gap: 12px;
    max-width: 380px;
    width: calc(100vw - 48px);
    pointer-events: none;
  }

  .toast-card {
    pointer-events: auto;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 14px 18px;
    border-radius: 8px;
    background: var(--bg-card);
    border: 1px solid var(--border-color);
    box-shadow: var(--shadow-premium);
    color: var(--text-primary);
    position: relative;
    overflow: hidden;
  }

  .toast-card::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 4px;
    height: 100%;
  }

  /* Types style */
  .toast-card.success::before {
    background: var(--accent-green);
  }

  .toast-card.error::before {
    background: var(--accent-red);
  }

  .toast-card.info::before {
    background: #3b82f6;
  }

  .toast-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .toast-svg {
    width: 18px;
    height: 18px;
  }

  .toast-svg.success-icon {
    color: var(--accent-green);
  }

  .toast-svg.error-icon {
    color: var(--accent-red);
  }

  .toast-svg.info-icon {
    color: #3b82f6;
  }

  .toast-content {
    flex: 1;
    font-size: 13px;
    font-weight: 500;
    line-height: 1.5;
    color: var(--text-primary);
  }

  .toast-close {
    background: transparent;
    border: none;
    color: var(--text-muted);
    font-size: 18px;
    line-height: 1;
    cursor: pointer;
    padding: 0 2px;
    transition: color 0.2s;
  }

  .toast-close:hover {
    color: var(--text-primary);
  }
</style>
