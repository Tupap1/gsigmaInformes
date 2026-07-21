<script lang="ts">
  import { onMount } from 'svelte';
  import '../app.css';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import ToastContainer from '$lib/components/ToastContainer.svelte';
  import { toasts } from '$lib/stores/toasts.svelte';
  import { theme } from '$lib/stores/theme.svelte';
  import favicon from '$lib/assets/favicon.svg';

  let { children } = $props();

  async function checkAppUpdates() {
    try {
      // Importación dinámica para prevenir errores en entornos que no tengan inyectada la API de Tauri (como builds Web)
      const { check } = await import('@tauri-apps/plugin-updater');
      const { relaunch } = await import('@tauri-apps/plugin-process');

      console.log('🔄 Comprobando actualizaciones de la aplicación...');
      const update = await check();
      
      if (update && update.available) {
        console.log(`💡 Nueva versión disponible: v${update.version}`);
        toasts.info(`Nueva versión v${update.version} disponible. Descargando e instalando...`, 8000);
        
        // Descargar e instalar
        await update.downloadAndInstall();
        
        toasts.success('Actualización instalada con éxito. Reiniciando aplicación...', 5000);
        
        // Pequeña espera para que el usuario lea el mensaje y reiniciar
        setTimeout(async () => {
          await relaunch();
        }, 3000);
      } else {
        console.log('✅ La aplicación está actualizada a la última versión.');
      }
    } catch (err: any) {
      console.warn('⚠️ No se pudo comprobar actualizaciones:', err.message);
    }
  }

  onMount(() => {
    // Retrasar 2 segundos la comprobación al iniciar para no bloquear el renderizado inicial y dar prioridad a la conexión de base de datos
    const timeout = setTimeout(checkAppUpdates, 2000);
    return () => clearTimeout(timeout);
  });
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
  <title>Recicladora Boyacá</title>
</svelte:head>

<div class="app-container">
  <Sidebar />
  <main class="main-content">
    {@render children()}
  </main>
</div>

<ToastContainer />

<style>
  /* Layout structure layout is already configured by app-container and main-content in app.css */
</style>
