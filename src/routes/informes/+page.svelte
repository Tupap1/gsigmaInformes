<script lang="ts">
  // Placeholder for Informes (Fase 5)
</script>

<svelte:head>
  <title>Informes - Recicladora Boyacá</title>
</svelte:head>

<div class="informes-placeholder animate-fade-in">
  <div class="glass-panel main-placeholder">
    <span class="placeholder-icon">📄</span>
    <h1 class="title">Sección de Informes</h1>
    <p class="subtitle">La Fase 5: Informes + PDF se encuentra pendiente de implementación.</p>
    <div class="badge">Próximamente</div>
  </div>
</div>

<style>
  .informes-placeholder {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: calc(100vh - 80px);
    width: 100%;
  }

  .main-placeholder {
    max-width: 500px;
    width: 100%;
    padding: 60px 40px;
    text-align: center;
    background: rgba(13, 20, 37, 0.4);
  }

  .placeholder-icon {
    font-size: 54px;
    display: block;
    margin-bottom: 24px;
    opacity: 0.7;
    animation: bounce 2s infinite ease-in-out;
  }

  .title {
    font-size: 24px;
    font-weight: 800;
    margin-bottom: 12px;
    background: linear-gradient(135deg, #ffffff 0%, #cbd5e1 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  .subtitle {
    font-size: 14px;
    color: var(--text-secondary);
    line-height: 1.6;
    margin-bottom: 24px;
  }

  .badge {
    display: inline-block;
    padding: 6px 16px;
    background: var(--accent-green-light);
    border: 1px solid rgba(16, 185, 129, 0.2);
    color: #34d399;
    border-radius: 9999px;
    font-size: 12px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  @keyframes bounce {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(-8px); }
  }
</style>
