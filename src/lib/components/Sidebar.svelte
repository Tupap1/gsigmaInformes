<script lang="ts">
  import { page } from '$app/stores';
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
</aside>

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
    border-bottom: 1px solid rgba(255, 255, 255, 0.03);
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
    background: rgba(255, 255, 255, 0.03);
  }

  .nav-item.active {
    color: #ffffff;
    background: var(--accent-green-light);
    border: 1px solid rgba(16, 185, 129, 0.15);
    font-weight: 600;
  }
</style>
