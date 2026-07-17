<script lang="ts">
  interface Props {
    type?: 'table-row' | 'card' | 'text' | 'circle';
    count?: number;
    height?: string;
  }

  let { type = 'text', count = 1, height }: Props = $props();
</script>

<div class="skeleton-wrapper">
  {#each Array(count) as _}
    {#if type === 'table-row'}
      <div class="skeleton-row" style={height ? `height: ${height};` : ''}>
        <div class="skeleton-item skeleton-shimmer w-10"></div>
        <div class="skeleton-item skeleton-shimmer w-20"></div>
        <div class="skeleton-item skeleton-shimmer w-40"></div>
        <div class="skeleton-item skeleton-shimmer w-15"></div>
        <div class="skeleton-item skeleton-shimmer w-15"></div>
      </div>
    {:else if type === 'card'}
      <div class="skeleton-card" style={height ? `height: ${height};` : ''}>
        <div class="skeleton-header">
          <div class="skeleton-item skeleton-shimmer circle"></div>
          <div class="skeleton-lines">
            <div class="skeleton-item skeleton-shimmer w-50"></div>
            <div class="skeleton-item skeleton-shimmer w-30"></div>
          </div>
        </div>
        <div class="skeleton-body">
          <div class="skeleton-item skeleton-shimmer w-100"></div>
          <div class="skeleton-item skeleton-shimmer w-90"></div>
          <div class="skeleton-item skeleton-shimmer w-60"></div>
        </div>
      </div>
    {:else if type === 'circle'}
      <div class="skeleton-item skeleton-shimmer circle" style={height ? `width: ${height}; height: ${height};` : ''}></div>
    {:else}
      <div class="skeleton-text" style={height ? `height: ${height};` : ''}>
        <div class="skeleton-item skeleton-shimmer w-100"></div>
      </div>
    {/if}
  {/each}
</div>

<style>
  .skeleton-wrapper {
    display: flex;
    flex-direction: column;
    gap: 12px;
    width: 100%;
  }

  .skeleton-item {
    border-radius: 6px;
    background: rgba(255, 255, 255, 0.04);
  }

  /* Width classes */
  .w-10 { width: 10%; }
  .w-15 { width: 15%; }
  .w-20 { width: 20%; }
  .w-30 { width: 30%; }
  .w-40 { width: 40%; }
  .w-50 { width: 50%; }
  .w-60 { width: 60%; }
  .w-90 { width: 90%; }
  .w-100 { width: 100%; }

  .circle {
    border-radius: 50%;
    width: 40px;
    height: 40px;
    flex-shrink: 0;
  }

  /* Table row layout */
  .skeleton-row {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 16px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.03);
    width: 100%;
  }

  .skeleton-row .skeleton-item {
    height: 18px;
  }

  /* Card layout */
  .skeleton-card {
    background: rgba(255, 255, 255, 0.02);
    border: 1px solid rgba(255, 255, 255, 0.04);
    border-radius: 16px;
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .skeleton-header {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .skeleton-lines {
    display: flex;
    flex-direction: column;
    gap: 6px;
    flex: 1;
  }

  .skeleton-lines .skeleton-item {
    height: 14px;
  }

  .skeleton-body {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .skeleton-body .skeleton-item {
    height: 12px;
  }

  /* Simple text layout */
  .skeleton-text {
    height: 16px;
    width: 100%;
  }

  .skeleton-text .skeleton-item {
    height: 100%;
  }
</style>
