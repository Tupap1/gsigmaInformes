<script lang="ts">
  import { onMount } from 'svelte';
  import { api, type CompraAcumulada, type ResumenCaja, type Producto } from '$lib/api';
  import { toasts } from '$lib/stores/toasts.svelte';
  import { generateReportPDF } from '$lib/pdf';
  import SkeletonLoader from '$lib/components/SkeletonLoader.svelte';

  // Get current date and first day of month as default strings
  const today = new Date().toISOString().split('T')[0];
  const firstDayOfMonth = today.substring(0, 8) + '01';

  // State (Svelte 5 Runes)
  let fechaInicio = $state(firstDayOfMonth);
  let fechaFin = $state(today);
  let loading = $state(false);
  let reportGenerated = $state(false);

  // Data State
  let compras = $state<CompraAcumulada[]>([]);
  let resumen = $state<ResumenCaja | null>(null);
  let productos = $state<Producto[]>([]);

  // Load products list on mount for catalogue preview
  async function loadCatalog() {
    try {
      productos = await api.getProductos();
    } catch (e: any) {
      console.error("Failed to load products list:", e);
      toasts.error("No se pudo cargar el catálogo de productos.");
    }
  }

  // Generate Report
  async function generateReport() {
    if (!fechaInicio || !fechaFin) {
      toasts.error("Debe seleccionar ambas fechas.");
      return;
    }

    if (new Date(fechaInicio) > new Date(fechaFin)) {
      toasts.error("La fecha de inicio no puede ser posterior a la fecha de fin.");
      return;
    }

    loading = true;
    reportGenerated = false;
    try {
      // Execute read-only aggregations parallelly
      const [comprasData, resumenData] = await Promise.all([
        api.getComprasAcumuladas(fechaInicio, fechaFin),
        api.getResumenCaja(fechaInicio, fechaFin)
      ]);

      compras = comprasData;
      resumen = resumenData;
      reportGenerated = true;
      toasts.success("Informe generado con éxito.");
    } catch (e: any) {
      console.error(e);
      toasts.error(`Error al generar informe: ${e}`);
    } finally {
      loading = false;
    }
  }

  // Export PDF Ticket
  async function exportPDF() {
    if (!reportGenerated || !resumen) return;
    try {
      const saved = await generateReportPDF(fechaInicio, fechaFin, compras, resumen);
      if (saved) {
        toasts.success("Ticket PDF guardado con éxito.");
      }
    } catch (e: any) {
      console.error("PDF Generation error:", e);
      toasts.error(`Error al guardar PDF: ${e}`);
    }
  }

  onMount(() => {
    loadCatalog();
    generateReport(); // Pre-load report for current month
  });
</script>

<svelte:head>
  <title>Informes - Recicladora Boyacá</title>
</svelte:head>

<div class="informes-view animate-fade-in">
  <div class="header-section">
    <div>
      <h1 class="page-title">Informes y Caja</h1>
      <p class="page-subtitle">Visualiza la consolidación de compras y balances de caja</p>
    </div>
    {#if reportGenerated}
      <button class="btn btn-secondary" onclick={exportPDF} title="Generar Ticket Impresora PDF">
        <span>📄</span> Exportar Ticket PDF
      </button>
    {/if}
  </div>

  <!-- Filters Row -->
  <div class="filters-card glass-panel">
    <div class="filters-grid">
      <div class="form-group">
        <label for="fechaInicio">Fecha Inicio</label>
        <input id="fechaInicio" type="date" class="form-control" bind:value={fechaInicio} />
      </div>
      <div class="form-group">
        <label for="fechaFin">Fecha Fin</label>
        <input id="fechaFin" type="date" class="form-control" bind:value={fechaFin} />
      </div>
    </div>
    <button class="btn btn-primary generate-btn shadow-glow" onclick={generateReport} disabled={loading}>
      {loading ? 'Generando...' : 'Generar Informe'}
    </button>
  </div>

  {#if loading}
    <SkeletonLoader type="card" height="120px" count={1} />
    <SkeletonLoader type="table-row" count={5} />
  {:else if reportGenerated && resumen}
    <!-- Financial cards grid (T5.4) -->
    <div class="caja-grid">
      <div class="caja-card">
        <div class="caja-hdr">
          <span class="card-icon">🏦</span>
          <h4>Base Caja</h4>
        </div>
        <div class="caja-val">${resumen.baseCaja.toLocaleString('es-CO')}</div>
        <div class="caja-badge plus">(+) Entrada</div>
      </div>

      <div class="caja-card">
        <div class="caja-hdr">
          <span class="card-icon">📈</span>
          <h4>Venta Contado</h4>
        </div>
        <div class="caja-val">${resumen.ventasContado.toLocaleString('es-CO')}</div>
        <div class="caja-badge plus">(+) Entrada</div>
      </div>

      <div class="caja-card">
        <div class="caja-hdr">
          <span class="card-icon">➕</span>
          <h4>Otros Ingresos</h4>
        </div>
        <div class="caja-val">${resumen.ingresos.toLocaleString('es-CO')}</div>
        <div class="caja-badge plus">(+) Entrada</div>
      </div>

      <div class="caja-card destructive">
        <div class="caja-hdr">
          <span class="card-icon">🛒</span>
          <h4>Pagado en Compras</h4>
        </div>
        <div class="caja-val">${resumen.compras.toLocaleString('es-CO')}</div>
        <div class="caja-badge minus">(-) Salida</div>
      </div>

      <div class="caja-card destructive">
        <div class="caja-hdr">
          <span class="card-icon">💸</span>
          <h4>Total Egresos</h4>
        </div>
        <div class="caja-val">${resumen.egresos.toLocaleString('es-CO')}</div>
        <div class="caja-badge minus">(-) Salida</div>
      </div>

      <div class="caja-card highlighted-green">
        <div class="caja-hdr">
          <span class="card-icon">♻️</span>
          <h4>Caja Efectivo</h4>
        </div>
        <div class="caja-val">${resumen.cajaEfectivo.toLocaleString('es-CO')}</div>
        <div class="caja-desc">Base + VentaContado + Ingresos - Compras - Egresos</div>
      </div>

      <div class="caja-card highlighted-blue">
        <div class="caja-hdr">
          <span class="card-icon">💰</span>
          <h4>Total en Caja</h4>
        </div>
        <div class="caja-val">${resumen.cajaTotal.toLocaleString('es-CO')}</div>
        <div class="caja-desc">Caja Efectivo + Ventas Crédito (${resumen.ventasCredito.toLocaleString('es-CO')})</div>
      </div>
    </div>

    <!-- Layout Columns: Table on Left, Products Catalog Sidebar on Right (T5.2) -->
    <div class="reports-layout">
      <!-- Compras Acumuladas Table -->
      <div class="table-container glass-panel">
        <div class="table-hdr">
          <h3>Compras Acumuladas por Material</h3>
          <span class="badge">{compras.length} materiales</span>
        </div>
        {#if compras.length === 0}
          <div class="empty-report">
            <span>🛒</span>
            <p>No se registraron compras en este rango de fechas.</p>
          </div>
        {:else}
          <table class="report-table">
            <thead>
              <tr>
                <th>Código PAS</th>
                <th>Material</th>
                <th class="num-col">Cantidad Comprada</th>
                <th class="num-col">Costo Promedio (CPP)</th>
                <th class="num-col">Total Invertido</th>
              </tr>
            </thead>
            <tbody>
              {#each compras as item}
                <tr class="table-row">
                  <td class="mat-code">{item.pas}</td>
                  <td class="mat-name">{item.nombre}</td>
                  <td class="num-col bold">{item.cantidad.toLocaleString('es-CO', {minimumFractionDigits:2, maximumFractionDigits:2})}</td>
                  <td class="num-col">${item.costoPromedio.toLocaleString('es-CO', {maximumFractionDigits:0})}</td>
                  <td class="num-col total-val">${item.total.toLocaleString('es-CO', {maximumFractionDigits:0})}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        {/if}
      </div>

      <!-- Products Catalog sidebar (T5.2 Catalog view) -->
      <div class="catalog-sidebar glass-panel">
        <div class="catalog-hdr">
          <h3>Catálogo PAS Comprables</h3>
          <p>Materiales activos marcados para compra (PASCOMP=1)</p>
        </div>
        <div class="catalog-list">
          {#each productos as prod}
            <div class="catalog-item">
              <span class="catalog-code">{prod.pas}</span>
              <span class="catalog-name">{prod.pasnom}</span>
            </div>
          {/each}
          {#if productos.length === 0}
            <div class="empty-catalog">Cargando catálogo...</div>
          {/if}
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .informes-view {
    display: flex;
    flex-direction: column;
    gap: 24px;
    animation: fadeIn 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards;
  }

  .header-section {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .page-title {
    font-size: 26px;
    font-weight: 800;
    letter-spacing: -0.02em;
    background: linear-gradient(135deg, #ffffff 0%, #a5b4fc 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  .page-subtitle {
    font-size: 14px;
    color: var(--text-secondary);
    margin-top: 4px;
  }

  /* Filters panel */
  .filters-card {
    padding: 20px;
    display: flex;
    align-items: flex-end;
    justify-content: space-between;
    gap: 20px;
    background: rgba(13, 20, 37, 0.4);
  }

  .filters-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
    flex: 1;
    max-width: 500px;
  }

  .filters-grid .form-group {
    margin-bottom: 0;
  }

  .generate-btn {
    height: 44px;
    padding: 0 32px;
  }

  /* Caja Grid */
  .caja-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(190px, 1fr));
    gap: 16px;
  }

  .caja-card {
    background: rgba(13, 20, 37, 0.4);
    border: 1px solid var(--border-color);
    border-radius: 16px;
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    position: relative;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .caja-card:hover {
    transform: translateY(-2px);
    border-color: var(--border-color-hover);
  }

  .caja-hdr {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .caja-hdr h4 {
    font-size: 13px;
    color: var(--text-secondary);
    font-weight: 600;
  }

  .card-icon {
    font-size: 16px;
  }

  .caja-val {
    font-size: 20px;
    font-weight: 700;
    color: var(--text-primary);
  }

  .caja-badge {
    align-self: flex-start;
    font-size: 11px;
    font-weight: 600;
    padding: 2px 8px;
    border-radius: 9999px;
  }

  .caja-badge.plus {
    background: var(--accent-green-light);
    color: #34d399;
  }

  .caja-badge.minus {
    background: var(--accent-red-light);
    color: #f87171;
  }

  .caja-desc {
    font-size: 11px;
    color: var(--text-muted);
    line-height: 1.4;
  }

  /* Highlight card styles */
  .caja-card.highlighted-green {
    background: linear-gradient(135deg, rgba(16, 185, 129, 0.08) 0%, rgba(5, 150, 105, 0.03) 100%);
    border-color: rgba(16, 185, 129, 0.25);
    box-shadow: 0 10px 25px -10px rgba(16, 185, 129, 0.1);
  }

  .caja-card.highlighted-green:hover {
    border-color: rgba(16, 185, 129, 0.5);
  }

  .caja-card.highlighted-green .caja-val {
    color: #34d399;
  }

  .caja-card.highlighted-blue {
    background: linear-gradient(135deg, rgba(59, 130, 246, 0.08) 0%, rgba(37, 99, 235, 0.03) 100%);
    border-color: rgba(59, 130, 246, 0.25);
    box-shadow: 0 10px 25px -10px rgba(59, 130, 246, 0.1);
  }

  .caja-card.highlighted-blue:hover {
    border-color: rgba(59, 130, 246, 0.5);
  }

  .caja-card.highlighted-blue .caja-val {
    color: #60a5fa;
  }

  /* Layout Columns */
  .reports-layout {
    display: grid;
    grid-template-columns: 1fr 280px;
    gap: 24px;
    align-items: start;
  }

  .table-container {
    padding: 24px;
    background: rgba(13, 20, 37, 0.3);
  }

  .table-hdr {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
  }

  .table-hdr h3 {
    font-size: 16px;
    font-weight: 700;
  }

  .table-hdr .badge {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid var(--border-color);
    padding: 3px 10px;
    border-radius: 9999px;
    font-size: 11px;
    color: var(--text-secondary);
  }

  /* Table styling */
  .report-table {
    width: 100%;
    border-collapse: collapse;
    text-align: left;
    font-size: 13.5px;
  }

  .report-table th {
    padding: 12px 16px;
    font-weight: 600;
    color: var(--text-muted);
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  }

  .report-table td {
    padding: 14px 16px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.02);
  }

  .num-col {
    text-align: right;
  }

  .mat-code {
    font-family: monospace;
    font-weight: 600;
    color: var(--text-muted);
  }

  .mat-name {
    font-weight: 600;
    color: var(--text-primary);
  }

  .bold {
    font-weight: 700;
  }

  .total-val {
    font-weight: 700;
    color: var(--accent-green);
  }

  .empty-report {
    padding: 40px;
    text-align: center;
    color: var(--text-secondary);
  }

  .empty-report span {
    font-size: 32px;
    display: block;
    margin-bottom: 12px;
    opacity: 0.5;
  }

  /* Catalog Sidebar */
  .catalog-sidebar {
    padding: 20px;
    background: rgba(13, 20, 37, 0.4);
    max-height: 500px;
    display: flex;
    flex-direction: column;
  }

  .catalog-hdr h3 {
    font-size: 14px;
    font-weight: 700;
  }

  .catalog-hdr p {
    font-size: 11px;
    color: var(--text-muted);
    margin-top: 4px;
    line-height: 1.4;
  }

  .catalog-list {
    margin-top: 16px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 8px;
    flex: 1;
    padding-right: 4px;
  }

  .catalog-item {
    display: flex;
    align-items: center;
    gap: 10px;
    background: rgba(255, 255, 255, 0.02);
    border: 1px solid rgba(255, 255, 255, 0.03);
    border-radius: 8px;
    padding: 8px 12px;
    font-size: 12.5px;
    transition: all 0.2s;
  }

  .catalog-item:hover {
    border-color: rgba(16, 185, 129, 0.2);
    background: rgba(16, 185, 129, 0.01);
  }

  .catalog-code {
    font-family: monospace;
    font-weight: 700;
    color: var(--accent-green);
  }

  .catalog-name {
    color: var(--text-secondary);
    font-weight: 500;
  }

  .empty-catalog {
    padding: 10px;
    color: var(--text-muted);
    font-size: 12px;
    text-align: center;
  }
</style>
