<script lang="ts">
  import { page } from '$app/stores';
  import { theme } from '$lib/stores/theme.svelte';
  import SettingsModal from './SettingsModal.svelte';

  // State (Svelte 5 runes)
  let showSettingsModal = $state(false);
</script>

<aside class="sidebar">
  <div class="sidebar-header">
    <div class="logo">
      <span class="logo-text">Recicladora</span>
    </div>
    <div class="subtitle">Boyacá</div>
  </div>

  <nav class="sidebar-nav">
    <a href="/" class="nav-item" class:active={$page.url.pathname === '/'}>
      <span class="nav-label">Inicio</span>
    </a>
    
    <a href="/proveedores" class="nav-item" class:active={$page.url.pathname.startsWith('/proveedores')}>
      <span class="nav-label">Terceros</span>
    </a>

    <a href="/informes" class="nav-item" class:active={$page.url.pathname.startsWith('/informes')}>
      <span class="nav-label">Informes</span>
    </a>
  </nav>

  <!-- Footer con Alternador de Temas y Configuración de Conexión -->
  <div class="sidebar-footer">
    <!-- Alternador de Tema Claro / Oscuro -->
    <button 
      class="footer-btn" 
      onclick={() => theme.toggle()} 
      title={theme.current === 'light' ? 'Alternar a Modo Oscuro' : 'Alternar a Modo Claro'}
    >
      {#if theme.current === 'light'}
        <!-- Icono Sol (Modo Claro) -->
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="footer-icon">
          <circle cx="12" cy="12" r="4"/>
          <path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M6.34 17.66l-1.41 1.41M19.07 4.93l-1.41 1.41"/>
        </svg>
      {:else}
        <!-- Icono Luna (Modo Oscuro) -->
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="footer-icon">
          <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"/>
        </svg>
      {/if}
      <span class="btn-label">{theme.current === 'light' ? 'Modo Claro' : 'Modo Oscuro'}</span>
    </button>

    <!-- Botón de Configuración (Engranaje) -->
    <button 
      class="footer-btn" 
      onclick={() => showSettingsModal = true} 
      title="Configuración de Base de Datos"
    >
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="footer-icon">
        <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.1a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/>
        <circle cx="12" cy="12" r="3"/>
      </svg>
      <span class="btn-label">Configuración</span>
    </button>
  </div>
</aside>

<!-- Modal de Configuración -->
{#if showSettingsModal}
  <SettingsModal onclose={() => showSettingsModal = false} />
{/if}

<style>
  .sidebar {
    width: var(--sidebar-width);
    height: 100vh;
    background: var(--bg-secondary);
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
  }

  .sidebar-header {
    padding: 24px 20px;
    border-bottom: 1px solid var(--border-color);
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .logo-text {
    font-size: 17px;
    font-weight: 700;
    color: var(--text-primary);
    letter-spacing: -0.02em;
  }

  .subtitle {
    font-size: 10px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.2em;
    margin-left: 30px;
    margin-top: -3px;
  }

  .sidebar-nav {
    flex: 1;
    padding: 16px 12px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 14px;
    border-radius: 6px;
    color: var(--text-secondary);
    text-decoration: none;
    font-weight: 500;
    font-size: 13.5px;
    border: 1px solid transparent;
    transition: all 0.1s ease;
  }

  .nav-item:hover {
    color: var(--text-primary);
    background: var(--bg-card-hover);
  }

  .nav-item.active {
    color: #ffffff;
    background: var(--accent-green);
    font-weight: 600;
  }

  /* Footer con botón de Configuración y Alternar Tema */
  .sidebar-footer {
    padding: 16px 12px;
    border-top: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .footer-btn {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 14px;
    border-radius: 6px;
    color: var(--text-secondary);
    background: transparent;
    border: 1px solid transparent;
    font-weight: 500;
    font-size: 13px;
    text-align: left;
    width: 100%;
    cursor: pointer;
    transition: all 0.12s ease;
  }

  .footer-btn:hover {
    color: var(--text-primary);
    background: var(--bg-card-hover);
    border-color: var(--border-color);
  }

  .footer-icon {
    width: 16px;
    height: 16px;
    flex-shrink: 0;
  }

  .btn-label {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>
