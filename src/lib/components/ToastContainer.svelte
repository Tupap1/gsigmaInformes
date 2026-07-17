<script lang="ts">
  import { toasts } from '$lib/stores/toasts';
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
          ✅
        {:else if toast.type === 'error'}
          ❌
        {:else}
          ℹ️
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
    align-items: flex-start;
    gap: 12px;
    padding: 16px 20px;
    border-radius: 14px;
    background: rgba(13, 20, 37, 0.9);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    border: 1px solid var(--border-color);
    box-shadow: 0 10px 30px -10px rgba(0, 0, 0, 0.5);
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
  .toast-card.success {
    border-color: rgba(16, 185, 129, 0.2);
    box-shadow: 0 10px 30px -10px rgba(16, 185, 129, 0.1);
  }
  .toast-card.success::before {
    background: var(--accent-green);
  }

  .toast-card.error {
    border-color: rgba(239, 68, 68, 0.2);
    box-shadow: 0 10px 30px -10px rgba(239, 68, 68, 0.1);
  }
  .toast-card.error::before {
    background: var(--accent-red);
  }

  .toast-card.info {
    border-color: rgba(59, 130, 246, 0.2);
    box-shadow: 0 10px 30px -10px rgba(59, 130, 246, 0.1);
  }
  .toast-card.info::before {
    background: #3b82f6;
  }

  .toast-icon {
    font-size: 16px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    margin-top: 2px;
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
