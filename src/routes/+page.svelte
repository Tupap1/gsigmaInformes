<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';

	// Svelte 5 runes
	let connectionStatus = $state<{ read: boolean; write: boolean } | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);
	
	async function checkConnection() {
		loading = true;
		error = null;
		try {
			connectionStatus = await invoke('test_connection');
		} catch (err: any) {
			error = err.toString();
		} finally {
			loading = false;
		}
	}
	
	onMount(() => {
		checkConnection();
	});
</script>

<svelte:head>
	<title>Recicladora Boyacá - Panel de Control</title>
</svelte:head>

<main class="dashboard animate-fade-in">
	<div class="glass-card">
		<div class="logo-container">
			<div class="logo-icon">♻️</div>
		</div>
		<h1 class="title">Recicladora Boyacá</h1>
		<p class="subtitle">Gestión de Terceros e Informes de Compras Acumuladas</p>
		
		<div class="divider"></div>
		
		<div class="status-container">
			{#if loading}
				<div class="status-indicator loading">
					<span class="pulse-dot-loading"></span>
					<span class="status-text">Verificando conexión con el motor de base de datos...</span>
				</div>
			{:else if error}
				<div class="status-indicator failed">
					<span class="pulse-dot-failed"></span>
					<span class="status-text">Fallo de conexión: {error}</span>
				</div>
				<button class="btn btn-secondary retry-btn" onclick={checkConnection}>Reintentar</button>
			{:else if connectionStatus}
				<div class="pools-grid">
					<div class="pool-card {connectionStatus.read ? 'success' : 'failed'}">
						<div class="pool-header">
							<span class="pool-icon">📖</span>
							<h3>Pool de Lectura</h3>
						</div>
						<div class="pool-status">
							<span class="pulse-dot {connectionStatus.read ? 'active' : 'inactive'}"></span>
							<span class="pool-text">{connectionStatus.read ? 'Conectado (reci_read)' : 'Desconectado'}</span>
						</div>
					</div>

					<div class="pool-card {connectionStatus.write ? 'success' : 'failed'}">
						<div class="pool-header">
							<span class="pool-icon">✍️</span>
							<h3>Pool de Escritura</h3>
						</div>
						<div class="pool-status">
							<span class="pulse-dot {connectionStatus.write ? 'active' : 'inactive'}"></span>
							<span class="pool-text">{connectionStatus.write ? 'Conectado (reci_write)' : 'Desconectado'}</span>
						</div>
					</div>
				</div>
				
				<div class="info-badge">
					<span class="info-icon">⚙️</span>
					<p class="details">
						Conectividad verificada: Servicios activos del motor MySQL 5.5 en entorno seguro.
					</p>
				</div>
			{/if}
		</div>
	</div>
</main>

<style>
	.dashboard {
		display: flex;
		justify-content: center;
		align-items: center;
		width: 100%;
		height: calc(100vh - 56px); /* offset body margins */
		padding: 20px;
		box-sizing: border-box;
	}

	.glass-card {
		background: var(--bg-card);
		backdrop-filter: var(--glass-blur);
		-webkit-backdrop-filter: var(--glass-blur);
		border: 1px solid var(--border-color);
		border-radius: 12px;
		padding: 36px;
		max-width: 500px;
		width: 100%;
		text-align: center;
		box-shadow: var(--shadow-premium);
	}

	.logo-container {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 64px;
		height: 64px;
		background: var(--accent-green-light);
		border: 1px solid rgba(16, 185, 129, 0.1);
		border-radius: 50%;
		margin-bottom: 20px;
	}

	.logo-icon {
		font-size: 32px;
	}

	.title {
		font-size: 24px;
		font-weight: 700;
		margin: 0 0 6px 0;
		color: var(--text-primary);
		letter-spacing: -0.02em;
	}

	.subtitle {
		font-size: 13.5px;
		color: var(--text-secondary);
		margin: 0 0 24px 0;
		font-weight: 400;
		line-height: 1.4;
	}

	.divider {
		height: 1px;
		background: linear-gradient(90deg, rgba(255,255,255,0) 0%, rgba(255,255,255,0.04) 50%, rgba(255,255,255,0) 100%);
		margin-bottom: 24px;
	}

	.status-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 16px;
		width: 100%;
	}

	.status-indicator {
		display: inline-flex;
		align-items: center;
		gap: 10px;
		padding: 8px 16px;
		border-radius: 9999px;
		font-size: 13px;
		font-weight: 500;
		border: 1px solid transparent;
	}

	.status-indicator.loading {
		background: rgba(148, 163, 184, 0.04);
		border-color: rgba(148, 163, 184, 0.1);
		color: var(--text-secondary);
	}

	.status-indicator.failed {
		background: var(--accent-red-light);
		border-color: rgba(239, 68, 68, 0.15);
		color: #fca5a5;
	}

	.retry-btn {
		margin-top: 8px;
	}

	.pools-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 12px;
		width: 100%;
	}

	.pool-card {
		background: rgba(9, 14, 26, 0.25);
		border: 1px solid var(--border-color);
		border-radius: 8px;
		padding: 16px;
		text-align: left;
		transition: border-color 0.15s ease;
	}

	.pool-card.success {
		border-left: 3px solid var(--accent-green);
	}

	.pool-card.failed {
		border-left: 3px solid var(--accent-red);
	}

	.pool-card:hover {
		border-color: var(--border-color-hover);
	}

	.pool-card.success:hover {
		border-color: rgba(16, 185, 129, 0.2);
	}

	.pool-card.failed:hover {
		border-color: rgba(239, 68, 68, 0.2);
	}

	.pool-header {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-bottom: 10px;
	}

	.pool-icon {
		font-size: 16px;
	}

	.pool-header h3 {
		margin: 0;
		font-size: 13.5px;
		font-weight: 600;
		color: var(--text-primary);
	}

	.pool-status {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.pool-text {
		font-size: 11.5px;
		color: var(--text-secondary);
		font-weight: 500;
		font-family: monospace;
	}

	.pulse-dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		display: inline-block;
	}

	.pulse-dot.active {
		background-color: var(--accent-green);
		box-shadow: 0 0 6px var(--accent-green-glow);
		animation: pulse-green 2s infinite;
	}

	.pulse-dot.inactive {
		background-color: var(--accent-red);
		box-shadow: 0 0 6px var(--accent-red-glow);
		animation: pulse-red 2s infinite;
	}

	.pulse-dot-loading {
		width: 6px;
		height: 6px;
		background-color: var(--text-muted);
		border-radius: 50%;
		animation: pulse-gray 1.5s infinite;
	}

	.pulse-dot-failed {
		width: 6px;
		height: 6px;
		background-color: var(--accent-red);
		border-radius: 50%;
		animation: pulse-red 1.5s infinite;
	}

	.info-badge {
		display: flex;
		align-items: center;
		gap: 10px;
		background: rgba(255, 255, 255, 0.01);
		border: 1px solid var(--border-color);
		border-radius: 8px;
		padding: 10px 14px;
		margin-top: 8px;
		width: 100%;
		box-sizing: border-box;
	}

	.info-icon {
		font-size: 16px;
		color: var(--text-muted);
	}

	.details {
		font-size: 12px;
		color: var(--text-secondary);
		line-height: 1.4;
		margin: 0;
		text-align: left;
	}

	@keyframes pulse-green {
		0% {
			transform: scale(0.95);
			box-shadow: 0 0 0 0 rgba(16, 185, 129, 0.3);
		}
		70% {
			transform: scale(1);
			box-shadow: 0 0 0 4px rgba(16, 185, 129, 0);
		}
		100% {
			transform: scale(0.95);
			box-shadow: 0 0 0 0 rgba(16, 185, 129, 0);
		}
	}

	@keyframes pulse-red {
		0% {
			transform: scale(0.95);
			box-shadow: 0 0 0 0 rgba(239, 68, 68, 0.3);
		}
		70% {
			transform: scale(1);
			box-shadow: 0 0 0 4px rgba(239, 68, 68, 0);
		}
		100% {
			transform: scale(0.95);
			box-shadow: 0 0 0 0 rgba(239, 68, 68, 0);
		}
	}

	@keyframes pulse-gray {
		0% {
			transform: scale(0.95);
			box-shadow: 0 0 0 0 rgba(148, 163, 184, 0.3);
		}
		70% {
			transform: scale(1);
			box-shadow: 0 0 0 4px rgba(148, 163, 184, 0);
		}
		100% {
			transform: scale(0.95);
			box-shadow: 0 0 0 0 rgba(148, 163, 184, 0);
		}
	}
</style>
