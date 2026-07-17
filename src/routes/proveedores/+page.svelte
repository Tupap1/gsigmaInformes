<script lang="ts">
  import { onMount } from 'svelte';
  import { api, type Proveedor, type CreateProveedorInput, type UpdateProveedorInput } from '$lib/api';
  import { toasts } from '$lib/stores/toasts.svelte';
  import Modal from '$lib/components/Modal.svelte';
  import SkeletonLoader from '$lib/components/SkeletonLoader.svelte';

  // State (Svelte 5 Runes)
  let suppliers = $state<Proveedor[]>([]);
  let searchQuery = $state('');
  let includeInactive = $state(false);
  let loading = $state(true);
  
  // Modal & Form State
  let showFormPanel = $state(false);
  let formMode = $state<'create' | 'edit'>('create');
  let selectedSupplier = $state<Proveedor | null>(null);
  let supplierToDelete = $state<Proveedor | null>(null);
  let saving = $state(false);
  let deleting = $state(false);

  // Form Inputs
  let numDoc = $state('');
  let tipoDoc = $state('C');
  let nombre = $state('');
  let apellido = $state('');
  let telefono1 = $state('');
  let telefono2 = $state('');
  let email = $state('');
  let contacto = $state('');
  let direccion1 = $state('');
  let ciudad = $state('');
  let departamento = $state('');
  let status = $state('A');

  // Fetch Suppliers
  async function fetchSuppliers() {
    loading = true;
    try {
      suppliers = await api.listProveedores(includeInactive);
    } catch (e: any) {
      console.error(e);
      toasts.error(`Error al cargar terceros: ${e}`);
    } finally {
      loading = false;
    }
  }

  // Filtered suppliers list (Client side fuzzy matching for instant typing response)
  const filteredSuppliers = $derived.by(() => {
    const q = searchQuery.trim().toLowerCase();
    if (!q) return suppliers;
    return suppliers.filter(p => 
      p.nombre.toLowerCase().includes(q) ||
      (p.apellido && p.apellido.toLowerCase().includes(q)) ||
      p.numDoc.toLowerCase().includes(q) ||
      p.id.toLowerCase().includes(q)
    );
  });

  // Watch includeInactive to reload
  $effect(() => {
    fetchSuppliers();
  });

  // Open Create Form
  function openCreateForm() {
    formMode = 'create';
    selectedSupplier = null;
    
    // Reset Form fields
    numDoc = '';
    tipoDoc = 'C';
    nombre = '';
    apellido = '';
    telefono1 = '';
    telefono2 = '';
    email = '';
    contacto = '';
    direccion1 = '';
    ciudad = '';
    departamento = '';
    status = 'A';

    showFormPanel = true;
  }

  // Open Edit Form
  function openEditForm(supplier: Proveedor) {
    formMode = 'edit';
    selectedSupplier = supplier;

    // Load fields
    numDoc = supplier.numDoc;
    tipoDoc = supplier.tipoDoc;
    nombre = supplier.nombre;
    apellido = supplier.apellido || '';
    telefono1 = supplier.telefono1 || '';
    telefono2 = supplier.telefono2 || '';
    email = supplier.email || '';
    contacto = supplier.contacto || '';
    direccion1 = supplier.direccion1 || '';
    ciudad = supplier.ciudad || '';
    departamento = supplier.departamento || '';
    status = supplier.status;

    showFormPanel = true;
  }

  // Handle Form Submit
  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (!nombre.trim()) {
      toasts.error('El nombre/razón social es requerido.');
      return;
    }

    if (formMode === 'create' && !numDoc.trim()) {
      toasts.error('El número de identificación es requerido.');
      return;
    }

    saving = true;
    try {
      if (formMode === 'create') {
        const input: CreateProveedorInput = {
          numDoc: numDoc.trim(),
          tipoDoc,
          nombre: nombre.trim(),
          apellido: apellido.trim() || null,
          telefono1: telefono1.trim() || null,
          telefono2: telefono2.trim() || null,
          email: email.trim() || null,
          contacto: contacto.trim() || null,
          direccion1: direccion1.trim() || null,
          ciudad: ciudad.trim() || null,
          departamento: departamento.trim() || null
        };
        const newId = await api.createProveedor(input);
        toasts.success(`Tercero creado con éxito. Código: ${newId}`);
      } else if (formMode === 'edit' && selectedSupplier) {
        const input: UpdateProveedorInput = {
          nombre: nombre.trim(),
          apellido: apellido.trim() || null,
          telefono1: telefono1.trim() || null,
          telefono2: telefono2.trim() || null,
          email: email.trim() || null,
          contacto: contacto.trim() || null,
          direccion1: direccion1.trim() || null,
          ciudad: ciudad.trim() || null,
          departamento: departamento.trim() || null,
          status: status
        };
        await api.updateProveedor(selectedSupplier.id, input);
        toasts.success(`Tercero actualizado con éxito.`);
      }

      showFormPanel = false;
      fetchSuppliers();
    } catch (e: any) {
      console.error(e);
      toasts.error(`Error al guardar tercero: ${e}`);
    } finally {
      saving = false;
    }
  }

  // Handle Delete Confirmation
  async function confirmDelete() {
    if (!supplierToDelete) return;
    deleting = true;
    try {
      const res = await api.deleteProveedor(supplierToDelete.id);
      if (res.success) {
        if (res.action === 'deactivated') {
          toasts.info(res.message);
        } else {
          toasts.success(res.message);
        }
        
        toasts.info(`Acción: ${res.action === 'deactivated' ? 'Desactivación lógica' : 'Eliminación física'}. Motivo: ${res.reason}`, 6000);
        
        supplierToDelete = null;
        fetchSuppliers();
      } else {
        toasts.error(`No se pudo procesar: ${res.message}`);
      }
    } catch (e: any) {
      console.error(e);
      toasts.error(`Error al eliminar tercero: ${e}`);
    } finally {
      deleting = false;
    }
  }

  onMount(() => {
    fetchSuppliers();
  });
</script>

<svelte:head>
  <title>Terceros - Recicladora Boyacá</title>
</svelte:head>

<div class="proveedores-view animate-fade-in">
  <div class="header-section">
    <div>
      <h1 class="page-title">Gestión de Terceros (Proveedores)</h1>
      <p class="page-subtitle">Catálogo centralizado de proveedores del POS en producción</p>
    </div>
    <button class="btn btn-primary" onclick={openCreateForm}>
      <span>➕</span> Registrar Tercero
    </button>
  </div>

  <!-- Filters Panel -->
  <div class="filters-card glass-panel">
    <div class="search-box">
      <span class="search-icon">🔍</span>
      <input 
        type="text" 
        placeholder="Buscar por nombre, apellidos, identificación o código..." 
        class="form-control search-input" 
        bind:value={searchQuery}
      />
      {#if searchQuery}
        <button class="clear-search" onclick={() => searchQuery = ''}>&times;</button>
      {/if}
    </div>

    <label class="checkbox-container">
      <input type="checkbox" bind:checked={includeInactive} />
      <span class="checkmark"></span>
      Incluir terceros inactivos
    </label>
  </div>

  <!-- Main Table -->
  <div class="table-container glass-panel">
    {#if loading}
      <SkeletonLoader type="table-row" count={8} />
    {:else if filteredSuppliers.length === 0}
      <div class="empty-state">
        <span class="empty-icon">👥</span>
        <h3>No se encontraron registros de terceros</h3>
        <p>Ajuste el término de búsqueda o habilite la inclusión de inactivos</p>
      </div>
    {:else}
      <table class="suppliers-table">
        <thead>
          <tr>
            <th class="col-code">Código</th>
            <th class="col-doc">Identificación</th>
            <th>Nombre / Razón Social</th>
            <th>Contacto / Ruta</th>
            <th>Teléfono</th>
            <th>Correo Electrónico</th>
            <th class="col-status">Estado</th>
            <th class="actions-header">Acciones</th>
          </tr>
        </thead>
        <tbody>
          {#each filteredSuppliers as supplier (supplier.id)}
            <tr class="table-row" class:inactive-row={supplier.status === 'I'}>
              <td class="supplier-code">{supplier.id}</td>
              <td>
                <span class="doc-badge">{supplier.tipoDoc}</span>
                {supplier.numDoc}
              </td>
              <td class="supplier-name">
                {supplier.nombre} {supplier.apellido || ''}
              </td>
              <td>{supplier.contacto || '-'}</td>
              <td>{supplier.telefono1 || supplier.telefono2 || '-'}</td>
              <td class="supplier-email" title={supplier.email}>{supplier.email || '-'}</td>
              <td>
                <span class="status-badge" class:active={supplier.status === 'A'} class:inactive={supplier.status === 'I'}>
                  {supplier.status === 'A' ? 'Activo' : 'Inactivo'}
                </span>
              </td>
              <td class="actions-cell">
                <button class="action-btn edit" onclick={() => openEditForm(supplier)} title="Editar tercero">
                  ✏️
                </button>
                <button class="action-btn delete" onclick={() => supplierToDelete = supplier} title="Desactivar/Eliminar tercero">
                  🗑️
                </button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </div>
</div>

<!-- Slide-over Panel (Form) -->
{#if showFormPanel}
  <div class="form-overlay" onclick={() => showFormPanel = false}>
    <div class="form-panel animate-slide-in-right" onclick={(e) => e.stopPropagation()}>
      <div class="panel-header">
        <h2>{formMode === 'create' ? 'Registrar Tercero (Proveedor)' : 'Modificar Tercero (Proveedor)'}</h2>
        <button class="close-panel" onclick={() => showFormPanel = false}>&times;</button>
      </div>

      <form onsubmit={handleSubmit} class="panel-body">
        <div class="form-section">
          <h3>Datos Básicos</h3>
          
          <div class="form-row">
            <div class="form-group">
              <label for="tipoDoc">Tipo de Identificación</label>
              <select id="tipoDoc" class="form-control custom-select" bind:value={tipoDoc} disabled={formMode === 'edit'}>
                <option value="C">Cédula de Ciudadanía (C)</option>
                <option value="N">NIT (N)</option>
                <option value="E">Cédula de Extranjería (E)</option>
              </select>
            </div>
            
            <div class="form-group">
              <label for="numDoc">Número de Identificación *</label>
              <input 
                id="numDoc" 
                type="text" 
                class="form-control" 
                placeholder="Ej. 12345678" 
                bind:value={numDoc} 
                disabled={formMode === 'edit'} 
                required 
              />
            </div>
          </div>

          <div class="form-group">
            <label for="nombre">Nombre o Razón Social *</label>
            <input 
              id="nombre" 
              type="text" 
              class="form-control" 
              placeholder="Nombre comercial o principal" 
              bind:value={nombre} 
              required 
            />
          </div>

          <div class="form-group">
            <label for="apellido">Primer/Segundo Apellido (Persona Natural)</label>
            <input 
              id="apellido" 
              type="text" 
              class="form-control" 
              placeholder="Apellidos del tercero" 
              bind:value={apellido} 
            />
          </div>
        </div>

        <div class="form-section">
          <h3>Ubicación y Contacto</h3>

          <div class="form-row">
            <div class="form-group">
              <label for="telefono1">Teléfono Principal</label>
              <input 
                id="telefono1" 
                type="text" 
                class="form-control" 
                placeholder="Celular/Fijo" 
                bind:value={telefono1} 
              />
            </div>
            
            <div class="form-group">
              <label for="telefono2">Teléfono Alterno</label>
              <input 
                id="telefono2" 
                type="text" 
                class="form-control" 
                placeholder="Opcional" 
                bind:value={telefono2} 
              />
            </div>
          </div>

          <div class="form-group">
            <label for="email">Correo Electrónico</label>
            <input 
              id="email" 
              type="email" 
              class="form-control" 
              placeholder="ejemplo@recicladora.com" 
              bind:value={email} 
            />
          </div>

          <div class="form-group">
            <label for="contacto">Contacto de Ruta / Encargado</label>
            <input 
              id="contacto" 
              type="text" 
              class="form-control" 
              placeholder="Ej. Conductor, Administrador" 
              bind:value={contacto} 
            />
          </div>

          <div class="form-group">
            <label for="direccion1">Dirección</label>
            <input 
              id="direccion1" 
              type="text" 
              class="form-control" 
              placeholder="Ej. Calle 10 # 4-50" 
              bind:value={direccion1} 
            />
          </div>

          <div class="form-row">
            <div class="form-group">
              <label for="ciudad">Ciudad</label>
              <input 
                id="ciudad" 
                type="text" 
                class="form-control" 
                placeholder="Ej. Tunja" 
                bind:value={ciudad} 
              />
            </div>
            
            <div class="form-group">
              <label for="departamento">Departamento</label>
              <input 
                id="departamento" 
                type="text" 
                class="form-control" 
                placeholder="Ej. Boyacá" 
                bind:value={departamento} 
              />
            </div>
          </div>

          {#if formMode === 'edit'}
            <div class="form-group">
              <label for="status">Estado</label>
              <select id="status" class="form-control custom-select" bind:value={status}>
                <option value="A">Activo</option>
                <option value="I">Inactivo</option>
              </select>
            </div>
          {/if}
        </div>

        <div class="panel-actions">
          <button type="button" class="btn btn-secondary" onclick={() => showFormPanel = false}>
            Cancelar
          </button>
          <button type="submit" class="btn btn-primary" disabled={saving}>
            {saving ? 'Guardando...' : 'Guardar Registro'}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}

<!-- Secure Delete Modal -->
{#if supplierToDelete}
  <Modal 
    title="Confirmar Inactivación / Eliminación" 
    onclose={() => supplierToDelete = null}
    maxWidth="500px"
  >
    <div class="delete-warning-content animate-fade-in">
      <div class="warning-banner">
        <span class="warning-icon">⚠️</span>
        <div class="warning-text">
          <strong>Advertencia de Integridad Contable</strong>
          <p>Esta acción se ejecutará directamente sobre la base de datos de producción.</p>
        </div>
      </div>

      <div class="supplier-details-box">
        <div class="detail-item">
          <span class="lbl">Tercero:</span>
          <span class="val">{supplierToDelete.nombre} {supplierToDelete.apellido || ''}</span>
        </div>
        <div class="detail-item">
          <span class="lbl">Identificación:</span>
          <span class="val">({supplierToDelete.tipoDoc}) {supplierToDelete.numDoc}</span>
        </div>
        <div class="detail-item">
          <span class="lbl">Código POS:</span>
          <span class="val">{supplierToDelete.id}</span>
        </div>
      </div>

      <div class="delete-info-note">
        <p><strong>Protocolo de Seguridad Contable (Borrado Seguro):</strong></p>
        <ul>
          <li>Si el tercero <strong>registra compras contables</strong>, se desactivará lógicamente (Estado: Inactivo) para conservar la trazabilidad histórica de la empresa.</li>
          <li>Si el tercero <strong>no registra movimientos de compra</strong>, se eliminará físicamente del sistema (tablas <code>proveedo</code> y <code>trc</code>).</li>
        </ul>
      </div>
    </div>

    {#snippet footer()}
      <button class="btn btn-secondary" onclick={() => supplierToDelete = null} disabled={deleting}>
        Cancelar
      </button>
      <button class="btn btn-danger" onclick={confirmDelete} disabled={deleting}>
        {deleting ? 'Procesando...' : 'Desactivar / Eliminar Tercero'}
      </button>
    {/snippet}
  </Modal>
{/if}

<style>
  .proveedores-view {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .header-section {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .page-title {
    font-size: 20px;
    font-weight: 700;
    letter-spacing: -0.01em;
    color: var(--text-primary);
  }

  .page-subtitle {
    font-size: 13px;
    color: var(--text-secondary);
    margin-top: 2px;
  }

  /* Filters Card */
  .filters-card {
    padding: 12px 16px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    flex-wrap: wrap;
    background: rgba(13, 20, 37, 0.25);
  }

  .search-box {
    position: relative;
    flex: 1;
    min-width: 280px;
  }

  .search-icon {
    position: absolute;
    left: 12px;
    top: 50%;
    transform: translateY(-50%);
    font-size: 14px;
    color: var(--text-muted);
  }

  .search-input {
    padding-left: 36px;
    padding-right: 32px;
  }

  .clear-search {
    position: absolute;
    right: 10px;
    top: 50%;
    transform: translateY(-50%);
    background: transparent;
    border: none;
    color: var(--text-secondary);
    font-size: 16px;
    cursor: pointer;
  }

  /* Custom Checkbox */
  .checkbox-container {
    display: flex;
    align-items: center;
    position: relative;
    padding-left: 24px;
    cursor: pointer;
    font-size: 12.5px;
    font-weight: 500;
    color: var(--text-secondary);
    user-select: none;
  }

  .checkbox-container input {
    position: absolute;
    opacity: 0;
    cursor: pointer;
    height: 0;
    width: 0;
  }

  .checkmark {
    position: absolute;
    left: 0;
    height: 15px;
    width: 15px;
    background-color: rgba(13, 20, 37, 0.4);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    transition: all 0.1s;
  }

  .checkbox-container:hover input ~ .checkmark {
    border-color: var(--border-color-hover);
  }

  .checkbox-container input:checked ~ .checkmark {
    background-color: var(--accent-green);
    border-color: var(--accent-green);
  }

  .checkmark:after {
    content: "";
    position: absolute;
    display: none;
  }

  .checkbox-container input:checked ~ .checkmark:after {
    display: block;
  }

  .checkbox-container .checkmark:after {
    left: 5px;
    top: 2px;
    width: 3px;
    height: 7px;
    border: solid #042f1a;
    border-width: 0 1.5px 1.5px 0;
    transform: rotate(45deg);
  }

  /* Table styles - Denser structure */
  .table-container {
    overflow-x: auto;
    background: rgba(13, 20, 37, 0.2);
    border-radius: 8px;
    padding: 0;
  }

  .suppliers-table {
    width: 100%;
    border-collapse: collapse;
    text-align: left;
    font-size: 12.5px;
  }

  .suppliers-table th {
    padding: 10px 14px;
    font-weight: 600;
    color: var(--text-muted);
    font-size: 10.5px;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    background: rgba(9, 14, 26, 0.2);
  }

  .suppliers-table td {
    padding: 10px 14px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.03);
    color: var(--text-primary);
    white-space: nowrap;
  }

  .table-row {
    transition: background-color 0.1s ease;
  }

  .table-row:hover {
    background-color: rgba(255, 255, 255, 0.015);
  }

  .inactive-row {
    opacity: 0.55;
  }

  .col-code {
    width: 90px;
  }

  .col-doc {
    width: 120px;
  }

  .col-status {
    width: 100px;
  }

  .supplier-code {
    font-family: monospace;
    font-weight: 600;
    color: var(--accent-green);
  }

  .supplier-name {
    font-weight: 600;
  }

  .supplier-email {
    max-width: 180px;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .doc-badge {
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid var(--border-color);
    border-radius: 3px;
    padding: 1px 4px;
    font-size: 10px;
    font-weight: 600;
    margin-right: 4px;
    color: var(--text-secondary);
  }

  /* Status Badges */
  .status-badge {
    display: inline-flex;
    align-items: center;
    padding: 2px 8px;
    border-radius: 4px;
    font-size: 10.5px;
    font-weight: 600;
    border: 1px solid transparent;
  }

  .status-badge.active {
    background: var(--accent-green-light);
    border-color: rgba(16, 185, 129, 0.15);
    color: #34d399;
  }

  .status-badge.inactive {
    background: var(--accent-red-light);
    border-color: rgba(239, 68, 68, 0.15);
    color: #f87171;
  }

  /* Actions */
  .actions-header {
    text-align: right;
    width: 80px;
  }

  .actions-cell {
    display: flex;
    justify-content: flex-end;
    gap: 6px;
  }

  .action-btn {
    background: rgba(255, 255, 255, 0.02);
    border: 1px solid var(--border-color);
    cursor: pointer;
    font-size: 11px;
    width: 26px;
    height: 26px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    transition: all 0.1s ease;
  }

  .action-btn.edit:hover {
    background: var(--accent-green-light);
    border-color: rgba(16, 185, 129, 0.25);
    color: var(--accent-green);
  }

  .action-btn.delete:hover {
    background: var(--accent-red-light);
    border-color: rgba(239, 68, 68, 0.25);
    color: var(--accent-red);
  }

  /* Empty State */
  .empty-state {
    padding: 48px 16px;
    text-align: center;
    color: var(--text-secondary);
  }

  .empty-icon {
    font-size: 32px;
    display: block;
    margin-bottom: 12px;
    opacity: 0.4;
  }

  .empty-state h3 {
    margin-bottom: 4px;
    font-weight: 600;
    font-size: 14px;
  }

  /* Slide-over Form Panel */
  .form-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(4, 6, 12, 0.45);
    backdrop-filter: blur(2px);
    -webkit-backdrop-filter: blur(2px);
    z-index: 900;
    display: flex;
    justify-content: flex-end;
  }

  .form-panel {
    background: var(--bg-secondary);
    width: 100%;
    max-width: 460px;
    height: 100%;
    box-shadow: -8px 0 24px -10px rgba(0, 0, 0, 0.7);
    display: flex;
    flex-direction: column;
    border-left: 1px solid var(--border-color);
  }

  .panel-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 18px 20px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.04);
  }

  .panel-header h2 {
    font-size: 15px;
    font-weight: 600;
  }

  .close-panel {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    font-size: 24px;
    cursor: pointer;
    line-height: 1;
    padding: 2px;
  }

  .close-panel:hover {
    color: var(--text-primary);
  }

  .panel-body {
    flex: 1;
    overflow-y: auto;
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .form-section h3 {
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--accent-green);
    margin-bottom: 12px;
    border-left: 2px solid var(--accent-green);
    padding-left: 8px;
  }

  .custom-select {
    appearance: none;
    -webkit-appearance: none;
    background-image: url("data:image/svg+xml;utf8,<svg fill='white' height='24' viewBox='0 0 24 24' width='24' xmlns='http://www.w3.org/2000/svg'><path d='M7 10l5 5 5-5z'/><path d='M0 0h24v24H0z' fill='none'/></svg>");
    background-repeat: no-repeat;
    background-position: right 8px center;
    background-size: 18px;
    padding-right: 30px;
  }

  .custom-select option {
    background-color: var(--bg-secondary);
    color: var(--text-primary);
  }

  .panel-actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    padding: 14px 20px;
    border-top: 1px solid rgba(255, 255, 255, 0.04);
    background: rgba(9, 14, 26, 0.3);
  }

  /* Delete warning - Compact */
  .delete-warning-content {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .warning-banner {
    display: flex;
    align-items: center;
    gap: 12px;
    background: rgba(239, 68, 68, 0.05);
    border: 1px solid rgba(239, 68, 68, 0.15);
    border-radius: 8px;
    padding: 12px;
  }

  .warning-icon {
    font-size: 20px;
  }

  .warning-text strong {
    color: #fca5a5;
    font-size: 13px;
    display: block;
    margin-bottom: 2px;
  }

  .warning-text p {
    font-size: 11.5px;
    color: var(--text-secondary);
    margin: 0;
  }

  .supplier-details-box {
    background: rgba(255, 255, 255, 0.01);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 12px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .detail-item {
    display: flex;
    justify-content: space-between;
    font-size: 12.5px;
  }

  .detail-item .lbl {
    color: var(--text-muted);
    font-weight: 500;
  }

  .detail-item .val {
    color: var(--text-primary);
    font-weight: 600;
  }

  .delete-info-note {
    font-size: 12px;
    line-height: 1.45;
  }

  .delete-info-note p {
    color: var(--text-primary);
  }

  .delete-info-note ul {
    margin-top: 6px;
    padding-left: 16px;
    display: flex;
    flex-direction: column;
    gap: 4px;
    color: var(--text-secondary);
  }
</style>
