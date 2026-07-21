<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '$lib/api';
  import { toasts } from '$lib/stores/toasts.svelte';
  import { fade } from 'svelte/transition';
  import { getVersion } from '@tauri-apps/api/app';

  // Props (Svelte 5 runes)
  let { onclose } = $props<{ onclose: () => void }>();

  // States
  let host = $state('127.0.0.1');
  let port = $state(3306);
  let reprovision = $state(false);
  let rootPassword = $state('');
  
  let loading = $state(true);
  let saving = $state(false);

  let appVersion = $state('Cargando...');
  let checkingUpdate = $state(false);

  // Cargar configuración existente en el onMount
  onMount(async () => {
    try {
      const config = await api.getDbConfig();
      host = config.host;
      port = config.port;
    } catch (e: any) {
      console.error('Error cargando la configuración de BD:', e);
      toasts.error('No se pudo cargar la configuración de red actual.');
    }

    try {
      appVersion = await getVersion();
    } catch (e) {
      appVersion = 'Desarrollo';
    } finally {
      loading = false;
    }
  });

  async function handleCheckUpdate() {
    checkingUpdate = true;
    try {
      const { check } = await import('@tauri-apps/plugin-updater');
      const { relaunch } = await import('@tauri-apps/plugin-process');

      toasts.info('Comprobando si hay nuevas actualizaciones...');
      const update = await check();
      
      if (update && update.available) {
        toasts.success(`Nueva versión v${update.version} encontrada. Descargando e instalando...`, 6000);
        await update.downloadAndInstall();
        toasts.success('Actualización instalada con éxito. Reiniciando...', 4000);
        setTimeout(async () => {
          await relaunch();
        }, 2000);
      } else {
        toasts.success('La aplicación está en la versión más reciente.');
      }
    } catch (err: any) {
      console.error(err);
      toasts.error('No se pudo comprobar actualizaciones en este entorno.');
    } finally {
      checkingUpdate = false;
    }
  }

  async function handleSave(e: SubmitEvent) {
    e.preventDefault();
    if (!host.trim()) {
      toasts.error('La dirección del servidor es requerida.');
      return;
    }
    if (!port || port <= 0) {
      toasts.error('El puerto ingresado no es válido.');
      return;
    }
    if (reprovision && !rootPassword.trim()) {
      toasts.error('La contraseña de root es requerida para re-provisionar.');
      return;
    }

    saving = true;
    try {
      if (reprovision) {
        // Opción con re-creación de usuarios y GRANTs (necesita clave root)
        toasts.info('Conectando como root y reconfigurando usuarios...');
        await api.setupDbConnection(host.trim(), port, rootPassword.trim());
        toasts.success('Base de datos re-provisionada y conectada con éxito.');
      } else {
        // Opción rápida (sólo actualizar host/puerto en config.json y reconectar pools)
        toasts.info('Actualizando parámetros de red...');
        await api.updateDbConnection(host.trim(), port);
        toasts.success('Conexión actualizada y pools reconectados con éxito.');
      }
      
      // Cerrar modal
      onclose();
    } catch (err: any) {
      console.error(err);
      toasts.error(`Error al conectar con la base de datos: ${err}`);
    } finally {
      saving = false;
    }
  }
</script>

<!-- Backdrop click to close -->
<div class="modal-backdrop" onclick={onclose} transition:fade={{ duration: 150 }}>
  <!-- Modal box -->
  <div 
    class="modal-dialog glass-panel animate-fade-in" 
    onclick={(e) => e.stopPropagation()}
    style="max-width: 480px; width: 100%;"
  >
    <div class="modal-header">
      <h2>Configuración de Base de Datos</h2>
      <button class="close-btn" onclick={onclose}>&times;</button>
    </div>

    <form onsubmit={handleSave} class="modal-body">
      {#if loading}
        <div style="padding: 2rem 0; text-align: center; color: var(--text-secondary);">
          Cargando configuración actual...
        </div>
      {:else}
        <div style="display: flex; flex-direction: column; gap: 1rem;">
          
          <div class="form-group">
            <label for="settings-host">Dirección del Servidor</label>
            <input 
              id="settings-host" 
              type="text" 
              class="form-control" 
              placeholder="Ej. 127.0.0.1 o 192.168.1.50" 
              bind:value={host}
              required 
            />
            <span style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.25rem; display: block;">
              IP o nombre de red de la máquina servidor donde corre el POS MySQL.
            </span>
          </div>

          <div class="form-group">
            <label for="settings-port">Puerto</label>
            <input 
              id="settings-port" 
              type="number" 
              class="form-control" 
              bind:value={port}
              required 
            />
          </div>

          <!-- Provision Switcher -->
          <div class="provision-toggle-container" style="margin-top: 0.5rem; padding: 0.75rem; background: rgba(0,0,0,0.03); border: 1px solid var(--border-color); border-radius: 6px;">
            <label style="display: flex; align-items: center; gap: 0.5rem; font-size: 0.9rem; cursor: pointer; color: var(--text-primary); font-weight: 500;">
              <input type="checkbox" bind:checked={reprovision} style="width: 1rem; height: 1rem;" />
              Re-crear usuarios en MySQL (Requiere Root)
            </label>
            
            {#if reprovision}
              <div class="form-group" style="margin-top: 1rem;" transition:fade={{ duration: 100 }}>
                <label for="settings-root" style="font-size: 0.85rem;">Contraseña de Root de MySQL</label>
                <input 
                  id="settings-root" 
                  type="password" 
                  class="form-control" 
                  placeholder="Ingrese clave de root del servidor" 
                  bind:value={rootPassword}
                  required 
                />
                <span style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.25rem; display: block;">
                  Se usa temporalmente para reconfigurar privilegios de usuarios. No se almacena en el sistema.
                </span>
              </div>
            {/if}
          </div>

        </div>
      {/if}

      <div class="version-section" style="margin-top: 1.5rem; padding-top: 1.25rem; border-top: 1px solid var(--border-color); display: flex; align-items: center; justify-content: space-between;">
        <div style="font-size: 0.85rem; color: var(--text-secondary);">
          Versión de la App: <strong style="color: var(--text-primary);">{appVersion}</strong>
        </div>
        <button 
          type="button" 
          class="btn btn-secondary" 
          style="padding: 6px 12px; font-size: 0.8rem;" 
          disabled={checkingUpdate}
          onclick={handleCheckUpdate}
        >
          {checkingUpdate ? 'Buscando...' : 'Buscar Actualizaciones'}
        </button>
      </div>

      <div class="modal-footer" style="margin-top: 1.5rem; display: flex; justify-content: flex-end; gap: 0.75rem;">
        <button type="button" class="btn btn-secondary" onclick={onclose}>
          Cancelar
        </button>
        <button type="submit" class="btn btn-primary" disabled={saving || loading}>
          {saving ? 'Validando...' : 'Aplicar y Conectar'}
        </button>
      </div>
    </form>
  </div>
</div>

<style>
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(0, 0, 0, 0.4);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-dialog {
    padding: 24px;
    border-radius: 8px;
    box-shadow: var(--shadow-premium);
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 20px;
  }

  .modal-header h2 {
    font-size: 1.25rem;
    margin: 0;
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 1.5rem;
    color: var(--text-secondary);
    cursor: pointer;
    line-height: 1;
    padding: 0;
  }

  .close-btn:hover {
    color: var(--text-primary);
  }

  .form-group {
    display: flex;
    flex-direction: column;
  }

  .form-group label {
    font-size: 0.9rem;
    font-weight: 500;
    margin-bottom: 6px;
    color: var(--text-secondary);
  }

  .form-control {
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    color: var(--text-primary);
    padding: 10px 14px;
    border-radius: 6px;
    font-size: 0.9rem;
    width: 100%;
    outline: none;
    transition: all 0.15s ease;
  }

  .form-control:focus {
    border-color: var(--accent-green);
    box-shadow: 0 0 0 2px var(--accent-green-light);
  }

  .btn {
    padding: 10px 18px;
    font-size: 0.9rem;
    font-weight: 600;
    border-radius: 6px;
    border: 1px solid transparent;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .btn-primary {
    background: var(--accent-green);
    color: #ffffff;
  }

  .btn-primary:hover:not(:disabled) {
    background: var(--accent-green-hover);
  }

  .btn-primary:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .btn-secondary {
    background: transparent;
    border: 1px solid var(--border-color);
    color: var(--text-primary);
  }

  .btn-secondary:hover {
    background: var(--bg-card-hover);
    border-color: var(--border-color-hover);
  }
</style>
