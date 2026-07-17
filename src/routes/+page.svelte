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

<main class="dashboard">
	<div class="glass-card">
		<div class="logo-container">
			<div class="logo-icon">♻️</div>
		</div>
		<h1 class="title">Recicladora Boyacá</h1>
		<p class="subtitle">Sistema de Gestión de Proveedores e Informes de Compras</p>
		
		<div class="divider"></div>
		
		<div class="status-container">
			{#if loading}
				<div class="status-indicator loading">
					<span class="pulse-dot-loading"></span>
					<span class="status-text">Probando conexión a MySQL desde Rust...</span>
				</div>
			{:else if error}
				<div class="status-indicator failed">
					<span class="pulse-dot-failed"></span>
					<span class="status-text">Error de Conexión: {error}</span>
				</div>
				<button class="retry-btn" onclick={checkConnection}>Reintentar</button>
			{:else if connectionStatus}
				<div class="pools-grid">
					<div class="pool-card {connectionStatus.read ? 'success' : 'failed'}">
						<div class="pool-header">
							<span class="pool-icon">📖</span>
							<h3>Pool Lectura</h3>
						</div>
						<div class="pool-status">
							<span class="pulse-dot {connectionStatus.read ? 'active' : 'inactive'}"></span>
							<span class="pool-text">{connectionStatus.read ? 'Conectado (reci_read)' : 'Desconectado'}</span>
						</div>
					</div>

					<div class="pool-card {connectionStatus.write ? 'success' : 'failed'}">
						<div class="pool-header">
							<span class="pool-icon">✍️</span>
							<h3>Pool Escritura</h3>
						</div>
						<div class="pool-status">
							<span class="pulse-dot {connectionStatus.write ? 'active' : 'inactive'}"></span>
							<span class="pool-text">{connectionStatus.write ? 'Conectado (reci_write)' : 'Desconectado'}</span>
						</div>
					</div>
				</div>
				
				<div class="info-badge">
					<span class="info-icon">⚡</span>
					<p class="details">
						Fase 2 Completada: Rust se conecta con éxito a MySQL 5.5 a través de SQLx.
					</p>
				</div>
			{/if}
		</div>
	</div>
</main>

<style>
	:global(body) {
		margin: 0;
		padding: 0;
		background: radial-gradient(circle at top right, #0f172a, #020617);
		color: #f8fafc;
		font-family: 'Inter', system-ui, -apple-system, sans-serif;
		height: 100vh;
		display: flex;
		justify-content: center;
		align-items: center;
		overflow: hidden;
	}

	.dashboard {
		display: flex;
		justify-content: center;
		align-items: center;
		width: 100%;
		height: 100%;
		padding: 20px;
		box-sizing: border-box;
	}

	.glass-card {
		background: rgba(15, 23, 42, 0.65);
		backdrop-filter: blur(20px);
		-webkit-backdrop-filter: blur(20px);
		border: 1px solid rgba(255, 255, 255, 0.08);
		border-radius: 28px;
		padding: 48px;
		max-width: 550px;
		width: 100%;
		text-align: center;
		box-shadow: 0 25px 60px -15px rgba(0, 0, 0, 0.7);
	}

	.logo-container {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 84px;
		height: 84px;
		background: radial-gradient(circle, rgba(16, 185, 129, 0.15) 0%, rgba(16, 185, 129, 0) 70%);
		border-radius: 50%;
		margin-bottom: 24px;
	}

	.logo-icon {
		font-size: 42px;
		animation: spin 10s linear infinite;
	}

	.title {
		font-size: 34px;
		font-weight: 800;
		margin: 0 0 8px 0;
		background: linear-gradient(135deg, #10b981 0%, #059669 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		letter-spacing: -0.5px;
	}

	.subtitle {
		font-size: 15px;
		color: #94a3b8;
		margin: 0 0 32px 0;
		font-weight: 400;
		line-height: 1.5;
	}

	.divider {
		height: 1px;
		background: linear-gradient(90deg, rgba(255,255,255,0) 0%, rgba(255,255,255,0.06) 50%, rgba(255,255,255,0) 100%);
		margin-bottom: 32px;
	}

	.status-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 20px;
		width: 100%;
	}

	.status-indicator {
		display: inline-flex;
		align-items: center;
		gap: 12px;
		padding: 10px 20px;
		border-radius: 9999px;
		font-size: 14px;
		font-weight: 500;
		border: 1px solid transparent;
		transition: all 0.3s ease;
	}

	.status-indicator.loading {
		background: rgba(148, 163, 184, 0.06);
		border-color: rgba(148, 163, 184, 0.15);
		color: #cbd5e1;
	}

	.status-indicator.failed {
		background: rgba(239, 68, 68, 0.08);
		border-color: rgba(239, 68, 68, 0.2);
		color: #fca5a5;
	}

	.retry-btn {
		background: rgba(255, 255, 255, 0.08);
		border: 1px solid rgba(255, 255, 255, 0.15);
		color: #f8fafc;
		padding: 10px 24px;
		border-radius: 12px;
		font-weight: 600;
		font-size: 14px;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.retry-btn:hover {
		background: rgba(255, 255, 255, 0.15);
		transform: translateY(-1px);
	}

	.pools-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 16px;
		width: 100%;
	}

	.pool-card {
		background: rgba(30, 41, 59, 0.4);
		border: 1px solid rgba(255, 255, 255, 0.05);
		border-radius: 16px;
		padding: 20px;
		text-align: left;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.pool-card:hover {
		transform: translateY(-2px);
		border-color: rgba(255, 255, 255, 0.1);
	}

	.pool-card.success {
		box-shadow: 0 0 20px -5px rgba(16, 185, 129, 0.1);
	}

	.pool-card.success:hover {
		border-color: rgba(16, 185, 129, 0.3);
	}

	.pool-card.failed {
		box-shadow: 0 0 20px -5px rgba(239, 68, 68, 0.1);
	}

	.pool-card.failed:hover {
		border-color: rgba(239, 68, 68, 0.3);
	}

	.pool-header {
		display: flex;
		align-items: center;
		gap: 10px;
		margin-bottom: 12px;
	}

	.pool-icon {
		font-size: 20px;
	}

	.pool-header h3 {
		margin: 0;
		font-size: 15px;
		font-weight: 600;
		color: #e2e8f0;
	}

	.pool-status {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.pool-text {
		font-size: 12px;
		color: #94a3b8;
		font-weight: 500;
	}

	.pulse-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		display: inline-block;
	}

	.pulse-dot.active {
		background-color: #10b981;
		box-shadow: 0 0 8px #10b981;
		animation: pulse-green 2s infinite;
	}

	.pulse-dot.inactive {
		background-color: #ef4444;
		box-shadow: 0 0 8px #ef4444;
		animation: pulse-red 2s infinite;
	}

	.pulse-dot-loading {
		width: 8px;
		height: 8px;
		background-color: #94a3b8;
		border-radius: 50%;
		animation: pulse-gray 1.5s infinite;
	}

	.pulse-dot-failed {
		width: 8px;
		height: 8px;
		background-color: #ef4444;
		border-radius: 50%;
		animation: pulse-red 1.5s infinite;
	}

	.info-badge {
		display: flex;
		align-items: center;
		gap: 12px;
		background: rgba(16, 185, 129, 0.05);
		border: 1px solid rgba(16, 185, 129, 0.15);
		border-radius: 14px;
		padding: 12px 20px;
		margin-top: 12px;
		width: 100%;
		box-sizing: border-box;
	}

	.info-icon {
		font-size: 18px;
	}

	.details {
		font-size: 13px;
		color: #cbd5e1;
		line-height: 1.5;
		margin: 0;
		text-align: left;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	@keyframes pulse-green {
		0% {
			transform: scale(0.95);
			box-shadow: 0 0 0 0 rgba(16, 185, 129, 0.5);
		}
		70% {
			transform: scale(1);
			box-shadow: 0 0 0 5px rgba(16, 185, 129, 0);
		}
		100% {
			transform: scale(0.95);
			box-shadow: 0 0 0 0 rgba(16, 185, 129, 0);
		}
	}

	@keyframes pulse-red {
		0% {
			transform: scale(0.95);
			box-shadow: 0 0 0 0 rgba(239, 68, 68, 0.5);
		}
		70% {
			transform: scale(1);
			box-shadow: 0 0 0 5px rgba(239, 68, 68, 0);
		}
		100% {
			transform: scale(0.95);
			box-shadow: 0 0 0 0 rgba(239, 68, 68, 0);
		}
	}

	@keyframes pulse-gray {
		0% {
			transform: scale(0.95);
			box-shadow: 0 0 0 0 rgba(148, 163, 184, 0.5);
		}
		70% {
			transform: scale(1);
			box-shadow: 0 0 0 5px rgba(148, 163, 184, 0);
		}
		100% {
			transform: scale(0.95);
			box-shadow: 0 0 0 0 rgba(148, 163, 184, 0);
		}
	}
</style>
