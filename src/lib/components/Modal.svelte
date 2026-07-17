<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { fade } from 'svelte/transition';

  interface Props {
    title: string;
    onclose: () => void;
    children?: import('svelte').Snippet;
    footer?: import('svelte').Snippet;
    maxWidth?: string; // e.g. "500px", "600px"
  }

  let { title, onclose, children, footer, maxWidth = '500px' }: Props = $props();

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      onclose();
    }
  }

  // Prevent scroll in background when modal is open
  onMount(() => {
    document.body.style.overflow = 'hidden';
    window.addEventListener('keydown', handleKeydown);
  });

  onDestroy(() => {
    document.body.style.overflow = '';
    window.removeEventListener('keydown', handleKeydown);
  });
</script>

<!-- Backdrop -->
<div class="modal-backdrop" onclick={onclose} transition:fade={{ duration: 200 }}>
  <!-- Modal box -->
  <div 
    class="modal-dialog glass-panel animate-fade-in" 
    onclick={(e) => e.stopPropagation()} 
    style="max-width: {maxWidth};"
  >
    <div class="modal-header">
      <h3 class="modal-title">{title}</h3>
      <button class="modal-close" onclick={onclose} aria-label="Cerrar modal">
        &times;
      </button>
    </div>

    <div class="modal-body">
      {#if children}
        {@render children()}
      {/if}
    </div>

    {#if footer}
      <div class="modal-footer">
        {@render footer()}
      </div>
    {/if}
  </div>
</div>

<style>
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(4, 6, 12, 0.75);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    z-index: 1000;
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 20px;
  }

  .modal-dialog {
    width: 100%;
    background: #0f1626;
    border: 1px solid var(--border-color);
    box-shadow: 0 30px 70px -10px rgba(0, 0, 0, 0.8), 0 0 0 1px rgba(255, 255, 255, 0.05);
    display: flex;
    flex-direction: column;
    max-height: calc(100vh - 40px);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 20px 24px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  }

  .modal-title {
    font-size: 18px;
    font-weight: 700;
    letter-spacing: -0.01em;
  }

  .modal-close {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    font-size: 24px;
    line-height: 1;
    cursor: pointer;
    padding: 4px;
    transition: color 0.2s, transform 0.2s;
  }

  .modal-close:hover {
    color: var(--text-primary);
    transform: scale(1.1);
  }

  .modal-body {
    padding: 24px;
    overflow-y: auto;
    font-size: 14px;
    color: var(--text-secondary);
    line-height: 1.6;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    padding: 20px 24px;
    border-top: 1px solid rgba(255, 255, 255, 0.05);
    background: rgba(9, 14, 26, 0.4);
    border-bottom-left-radius: inherit;
    border-bottom-right-radius: inherit;
  }
</style>
