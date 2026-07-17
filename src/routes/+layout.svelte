<script lang="ts">
  import '../app.css';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import ToastContainer from '$lib/components/ToastContainer.svelte';
  import favicon from '$lib/assets/favicon.svg';

  let { children } = $props();
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
