<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { invoke } from '@tauri-apps/api/core';

  let readConnected = $state<boolean | null>(null);
  let writeConnected = $state<boolean | null>(null);
  let checking = $state(false);

  async function checkDbConnection() {
    checking = true;
    try {
      const status = await invoke<{ read: boolean; write: boolean }>('test_connection');
      readConnected = status.read;
      writeConnected = status.write;
    } catch (e) {
      console.error("Failed to check db connection in Sidebar:", e);
      readConnected = false;
      writeConnected = false;
    } finally {
      checking = false;
    }
  }

  onMount(() => {
    checkDbConnection();
    // Re-check every 30 seconds
    const interval = setInterval(checkDbConnection, 30000);
    return () => clearInterval(interval);
  });
</script>

<aside class="sidebar">
  <div class="sidebar-header">
    <div class="logo">
      <span class="logo-icon">♻️</span>
      <span class="logo-text">Recicladora</span>
    </div>
    <div class="subtitle">Boyacá</div>
  </div>

  <nav class="sidebar-nav">
    <a href="/" class="nav-item" class:active={$page.url.pathname === '/'}>
      <span class="nav-icon">📊</span>
      <span class="nav-label">Inicio</span>
    </a>
    
    <a href="/proveedores" class="nav-item" class:active={$page.url.pathname.startsWith('/proveedores')}>
      <span class="nav-icon">👥</span>
      <span class="nav-label">Proveedores</span>
    </a>

    <a href="/informes" class="nav-item" class:active={$page.url.pathname.startsWith('/informes')}>
      <span class="nav-icon">📄</span>
      <span class="nav-label">Informes</span>
    </a>
  </nav>

  <div class="sidebar-footer">
    <div class="db-status-panel">
      <div class="db-status-header">
        <span class="panel-title">Base de Datos</span>
        <button class="refresh-btn" onclick={checkDbConnection} disabled={checking} class:spinning={checking} title="Recomprobar conexión">
          🔄
        </button>
      </div>

      <div class="status-row">
        <span class="status-indicator" class:active={readConnected === true} class:inactive={readConnected === false}></span>
        <span class="status-label">Lectura:</span>
        <span class="status-val">
          {readConnected === null ? '...' : (readConnected ? 'reci_read' : 'Error')}
        </span>
      </div>

      <div class="status-row">
        <span class="status-indicator" class:active={writeConnected === true} class:inactive={writeConnected === false}></span>
        <span class="status-label">Escritura:</span>
        <span class="status-val">
          {writeConnected === null ? '...' : (writeConnected ? 'reci_write' : 'Error')}
        </span>
      </div>
    </div>
  </div>
</aside>

<style>
  .sidebar {
    width: var(--sidebar-width);
    height: 100vh;
    background: rgba(13, 20, 37, 0.7);
    backdrop-filter: var(--glass-blur);
    -webkit-backdrop-filter: var(--glass-blur);
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
  }

  .sidebar-header {
    padding: 30px 24px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.03);
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .logo-icon {
    font-size: 28px;
    animation: pulse-logo 3s infinite ease-in-out;
  }

  .logo-text {
    font-size: 20px;
    font-weight: 800;
    background: linear-gradient(135deg, #10b981 0%, #34d399 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    letter-spacing: -0.03em;
  }

  .subtitle {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.25em;
    margin-left: 38px;
    margin-top: -4px;
  }

  .sidebar-nav {
    flex: 1;
    padding: 24px 16px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    border-radius: 12px;
    color: var(--text-secondary);
    text-decoration: none;
    font-weight: 500;
    font-size: 14px;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .nav-item:hover {
    color: var(--text-primary);
    background: rgba(255, 255, 255, 0.03);
    transform: translateX(3px);
  }

  .nav-item.active {
    color: #ffffff;
    background: var(--accent-green-light);
    border: 1px solid rgba(16, 185, 129, 0.15);
    font-weight: 600;
  }

  .nav-item.active .nav-icon {
    filter: drop-shadow(0 0 5px var(--accent-green-glow));
  }

  .nav-icon {
    font-size: 18px;
  }

  .sidebar-footer {
    padding: 20px;
    border-top: 1px solid rgba(255, 255, 255, 0.03);
  }

  .db-status-panel {
    background: rgba(9, 14, 26, 0.5);
    border: 1px solid rgba(255, 255, 255, 0.03);
    border-radius: 14px;
    padding: 14px;
  }

  .db-status-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
  }

  .panel-title {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .refresh-btn {
    background: transparent;
    border: none;
    cursor: pointer;
    font-size: 11px;
    padding: 2px;
    opacity: 0.5;
    transition: opacity 0.2s, transform 0.2s;
  }

  .refresh-btn:hover {
    opacity: 1;
  }

  .refresh-btn:disabled {
    cursor: not-allowed;
  }

  .spinning {
    animation: spin 1s linear infinite;
  }

  .status-row {
    display: flex;
    align-items: center;
    font-size: 12px;
    margin-top: 6px;
  }

  .status-indicator {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    margin-right: 8px;
    background: #64748b;
  }

  .status-indicator.active {
    background: var(--accent-green);
    box-shadow: 0 0 6px var(--accent-green);
  }

  .status-indicator.inactive {
    background: var(--accent-red);
    box-shadow: 0 0 6px var(--accent-red);
  }

  .status-label {
    color: var(--text-secondary);
    margin-right: 4px;
  }

  .status-val {
    color: var(--text-primary);
    font-weight: 500;
  }

  @keyframes pulse-logo {
    0%, 100% { transform: scale(1); }
    50% { transform: scale(1.06); }
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }
</style>
