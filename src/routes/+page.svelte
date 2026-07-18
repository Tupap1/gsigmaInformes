<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '$lib/api';

  let isConfigured = $state(true);
  let isLoading = $state(true);

  // Setup form state
  let host = $state('127.0.0.1');
  let port = $state(3306);
  let rootPassword = $state('');
  let setupError = $state('');
  let isSubmitting = $state(false);
  let showPassword = $state(false);

  onMount(async () => {
    try {
      isConfigured = await api.checkConfigured();
    } catch {
      isConfigured = false;
    }
    isLoading = false;
  });

  async function handleSetup(e: Event) {
    e.preventDefault();
    setupError = '';
    isSubmitting = true;

    try {
      await api.setupDbConnection(host, port, rootPassword);
      isConfigured = true;
    } catch (err: unknown) {
      setupError = err instanceof Error ? err.message : String(err);
    } finally {
      isSubmitting = false;
    }
  }
</script>

<svelte:head>
	<title>Recicladora Boyacá - Panel de Control</title>
</svelte:head>

{#if isLoading}
	<main class="dashboard animate-fade-in">
		<div class="glass-card" style="text-align:center; padding:48px;">
			<div class="shimmer-block" style="width:180px; height:24px; margin:0 auto 16px;"></div>
			<div class="shimmer-block" style="width:260px; height:14px; margin:0 auto;"></div>
		</div>
	</main>
{:else if !isConfigured}
	<!-- ═══════════════════════════════════════════════════════ -->
	<!-- ASISTENTE DE CONFIGURACIÓN INICIAL                     -->
	<!-- ═══════════════════════════════════════════════════════ -->
	<main class="dashboard animate-fade-in">
		<div class="glass-card setup-card">
			<div class="setup-header">
				<div class="setup-icon">
					<svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
						<path d="M4 6a2 2 0 012-2h12a2 2 0 012 2v4a2 2 0 01-2 2H6a2 2 0 01-2-2V6z"/>
						<path d="M4 14a2 2 0 012-2h12a2 2 0 012 2v4a2 2 0 01-2 2H6a2 2 0 01-2-2v-4z"/>
						<circle cx="8" cy="8" r="1" fill="currentColor"/>
						<circle cx="8" cy="16" r="1" fill="currentColor"/>
					</svg>
				</div>
				<div>
					<h1 class="title">Configuración Inicial</h1>
					<p class="subtitle">Configure la conexión a la base de datos MySQL del POS</p>
				</div>
			</div>

			<div class="divider"></div>

			<form class="setup-form" onsubmit={handleSetup}>
				<div class="form-group">
					<label for="setup-host">Dirección del Servidor</label>
					<input
						id="setup-host"
						type="text"
						bind:value={host}
						placeholder="127.0.0.1 o nombre del host"
						required
						disabled={isSubmitting}
					/>
					<span class="form-hint">IP o hostname del servidor MySQL del POS</span>
				</div>

				<div class="form-group">
					<label for="setup-port">Puerto</label>
					<input
						id="setup-port"
						type="number"
						bind:value={port}
						min="1"
						max="65535"
						required
						disabled={isSubmitting}
					/>
					<span class="form-hint">Puerto estándar de MySQL: 3306</span>
				</div>

				<div class="form-group">
					<label for="setup-password">Contraseña de Root de MySQL</label>
					<div class="password-wrapper">
						<input
							id="setup-password"
							type={showPassword ? 'text' : 'password'}
							bind:value={rootPassword}
							placeholder="Ingrese la contraseña de root"
							required
							disabled={isSubmitting}
							autocomplete="off"
						/>
						<button
							type="button"
							class="toggle-password"
							onclick={() => showPassword = !showPassword}
							tabindex={-1}
						>
							{showPassword ? '🔒' : '👁'}
						</button>
					</div>
					<span class="form-hint">Se usa solo para crear los usuarios de la aplicación. No se almacena.</span>
				</div>

				{#if setupError}
					<div class="setup-error">
						<strong>Error:</strong> {setupError}
					</div>
				{/if}

				<button
					type="submit"
					class="btn-setup"
					disabled={isSubmitting || !host || !rootPassword}
				>
					{#if isSubmitting}
						<span class="spinner"></span>
						Configurando...
					{:else}
						Conectar y Configurar
					{/if}
				</button>

				<p class="setup-note">
					Este asistente creará automáticamente los usuarios de lectura y escritura con permisos restringidos en la base de datos. La contraseña de root no se almacenará.
				</p>
			</form>
		</div>
	</main>
{:else}
	<!-- ═══════════════════════════════════════════════════════ -->
	<!-- DASHBOARD PRINCIPAL                                    -->
	<!-- ═══════════════════════════════════════════════════════ -->
	<main class="dashboard animate-fade-in">
		<div class="glass-card">
			<h1 class="title">Recicladora Boyacá</h1>
			<p class="subtitle">Módulo Administrativo del Sistema POS</p>

			<div class="divider"></div>

			<div class="welcome-container">
				<p class="welcome-text">
					Bienvenido al panel de administración. Seleccione una opción del menú lateral o utilice los accesos rápidos a continuación para comenzar:
				</p>

				<div class="action-grid">
					<a href="/proveedores" class="action-card">
						<h3>Gestión de Terceros</h3>
						<p>Registrar, modificar e inactivar proveedores locales en las bases del POS.</p>
					</a>

					<a href="/informes" class="action-card">
						<h3>Informes de Compra</h3>
						<p>Consultar acumulados históricos y generar reportes financieros consolidados.</p>
					</a>
				</div>
			</div>
		</div>
	</main>
{/if}

<style>
	.dashboard {
		display: flex;
		justify-content: center;
		align-items: center;
		width: 100%;
		height: calc(100vh - 56px);
		padding: 20px;
		box-sizing: border-box;
	}

	.glass-card {
		background: var(--bg-card);
		border: 1px solid var(--border-color);
		border-radius: 8px;
		padding: 36px;
		max-width: 580px;
		width: 100%;
		text-align: left;
		box-shadow: var(--shadow-premium);
	}

	.title {
		font-size: 22px;
		font-weight: 700;
		margin: 0 0 6px 0;
		color: var(--text-primary);
		letter-spacing: -0.02em;
	}

	.subtitle {
		font-size: 13px;
		color: var(--text-secondary);
		margin: 0 0 20px 0;
		font-weight: 400;
	}

	.divider {
		height: 1px;
		background: var(--border-color);
		margin-bottom: 20px;
	}

	.welcome-container {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.welcome-text {
		font-size: 13.5px;
		color: var(--text-secondary);
		line-height: 1.45;
	}

	.action-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 14px;
		margin-top: 8px;
	}

	.action-card {
		background: rgba(255, 255, 255, 0.015);
		border: 1px solid var(--border-color);
		border-radius: 6px;
		padding: 16px;
		text-decoration: none;
		transition: border-color 0.12s ease, background-color 0.12s ease;
	}

	.action-card:hover {
		border-color: var(--border-color-hover);
		background: rgba(255, 255, 255, 0.03);
	}

	.action-card h3 {
		font-size: 14px;
		font-weight: 600;
		color: var(--text-primary);
		margin-bottom: 6px;
	}

	.action-card p {
		font-size: 12px;
		color: var(--text-secondary);
		line-height: 1.4;
	}

	/* ═══════════════════════════════════════════════════════ */
	/* SETUP WIZARD STYLES                                    */
	/* ═══════════════════════════════════════════════════════ */
	.setup-card {
		max-width: 460px;
	}

	.setup-header {
		display: flex;
		align-items: flex-start;
		gap: 14px;
		margin-bottom: 20px;
	}

	.setup-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 44px;
		height: 44px;
		border-radius: 8px;
		background: rgba(16, 185, 129, 0.08);
		color: #10b981;
		flex-shrink: 0;
	}

	.setup-header .title {
		margin-bottom: 4px;
	}

	.setup-header .subtitle {
		margin-bottom: 0;
	}

	.setup-form {
		display: flex;
		flex-direction: column;
		gap: 18px;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.form-group label {
		font-size: 12.5px;
		font-weight: 600;
		color: var(--text-primary);
		letter-spacing: 0.01em;
	}

	.form-group input {
		background: rgba(255, 255, 255, 0.03);
		border: 1px solid var(--border-color);
		border-radius: 6px;
		padding: 10px 12px;
		font-size: 13.5px;
		color: var(--text-primary);
		outline: none;
		transition: border-color 0.15s ease;
		font-family: inherit;
		width: 100%;
		box-sizing: border-box;
	}

	.form-group input:focus {
		border-color: #10b981;
	}

	.form-group input:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.form-group input::placeholder {
		color: var(--text-secondary);
		opacity: 0.6;
	}

	.form-hint {
		font-size: 11px;
		color: var(--text-secondary);
		opacity: 0.7;
	}

	.password-wrapper {
		position: relative;
		display: flex;
		align-items: center;
	}

	.password-wrapper input {
		padding-right: 40px;
	}

	.toggle-password {
		position: absolute;
		right: 8px;
		background: none;
		border: none;
		cursor: pointer;
		font-size: 16px;
		padding: 4px;
		line-height: 1;
		opacity: 0.6;
		transition: opacity 0.12s ease;
	}

	.toggle-password:hover {
		opacity: 1;
	}

	.setup-error {
		background: rgba(239, 68, 68, 0.08);
		border: 1px solid rgba(239, 68, 68, 0.2);
		border-radius: 6px;
		padding: 10px 14px;
		font-size: 12.5px;
		color: #fca5a5;
		line-height: 1.45;
	}

	.btn-setup {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		width: 100%;
		padding: 11px 16px;
		background: #10b981;
		color: #fff;
		border: none;
		border-radius: 6px;
		font-size: 13.5px;
		font-weight: 600;
		cursor: pointer;
		transition: background-color 0.15s ease, opacity 0.15s ease;
		font-family: inherit;
	}

	.btn-setup:hover:not(:disabled) {
		background: #059669;
	}

	.btn-setup:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: #fff;
		border-radius: 50%;
		animation: spin 0.6s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.setup-note {
		font-size: 11px;
		color: var(--text-secondary);
		opacity: 0.6;
		line-height: 1.5;
		text-align: center;
	}

	/* Skeleton loader shimmer */
	.shimmer-block {
		background: linear-gradient(90deg, var(--bg-card) 25%, rgba(255,255,255,0.05) 50%, var(--bg-card) 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
	}

	@keyframes shimmer {
		0% { background-position: 200% 0; }
		100% { background-position: -200% 0; }
	}
</style>
