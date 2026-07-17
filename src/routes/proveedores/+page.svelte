<script lang="ts">
  import { onMount } from 'svelte';
  import { api, type Proveedor, type CreateProveedorInput, type UpdateProveedorInput } from '$lib/api';
  import { toasts } from '$lib/stores/toasts';
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
      toasts.error(`Error al cargar proveedores: ${e}`);
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
    // This triggers fetchSuppliers whenever includeInactive changes
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
      toasts.error('El número de documento es requerido.');
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
        toasts.success(`Proveedor creado con éxito. Código: ${newId}`);
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
        toasts.success(`Proveedor actualizado con éxito.`);
      }

      showFormPanel = false;
      fetchSuppliers();
    } catch (e: any) {
      console.error(e);
      toasts.error(`Error al guardar proveedor: ${e}`);
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
        
        // Show detailed pop-up about the result
        toasts.info(`Acción: ${res.action === 'deactivated' ? 'Desactivación lógica' : 'Eliminación física'}. Razón: ${res.reason}`, 6000);
        
        supplierToDelete = null;
        fetchSuppliers();
      } else {
        toasts.error(`No se pudo eliminar: ${res.message}`);
      }
    } catch (e: any) {
      console.error(e);
      toasts.error(`Error al eliminar proveedor: ${e}`);
    } finally {
      deleting = false;
    }
  }

  onMount(() => {
    fetchSuppliers();
  });
</script>

<svelte:head>
  <title>Proveedores - Recicladora Boyacá</title>
</svelte:head>

<div class="proveedores-view animate-fade-in">
  <div class="header-section">
    <div>
      <h1 class="page-title">Gestión de Proveedores</h1>
      <p class="page-subtitle">Administra los proveedores locales del POS</p>
    </div>
    <button class="btn btn-primary shadow-glow" onclick={openCreateForm}>
      <span>➕</span> Nuevo Proveedor
    </button>
  </div>

  <!-- Filters Panel -->
  <div class="filters-card glass-panel">
    <div class="search-box">
      <span class="search-icon">🔍</span>
      <input 
        type="text" 
        placeholder="Buscar por nombre, apellido, documento o código..." 
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
      Mostrar proveedores inactivos
    </label>
  </div>

  <!-- Main Table -->
  <div class="table-container glass-panel">
    {#if loading}
      <SkeletonLoader type="table-row" count={5} />
    {:else if filteredSuppliers.length === 0}
      <div class="empty-state">
        <span class="empty-icon">👥</span>
        <h3>No se encontraron proveedores</h3>
        <p>Prueba ajustando la búsqueda o habilita ver inactivos</p>
      </div>
    {:else}
      <table class="suppliers-table">
        <thead>
          <tr>
            <th>Código</th>
            <th>Documento</th>
            <th>Nombre / Razón Social</th>
            <th>Contacto</th>
            <th>Teléfono</th>
            <th>Email</th>
            <th>Estado</th>
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
                <button class="action-btn edit" onclick={() => openEditForm(supplier)} title="Editar proveedor">
                  ✏️
                </button>
                <button class="action-btn delete" onclick={() => supplierToDelete = supplier} title="Eliminar proveedor">
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
        <h2>{formMode === 'create' ? 'Crear Nuevo Proveedor' : 'Editar Proveedor'}</h2>
        <button class="close-panel" onclick={() => showFormPanel = false}>&times;</button>
      </div>

      <form onsubmit={handleSubmit} class="panel-body">
        <div class="form-section">
          <h3>Información Básica</h3>
          
          <div class="form-row">
            <div class="form-group">
              <label for="tipoDoc">Tipo de Documento</label>
              <select id="tipoDoc" class="form-control" bind:value={tipoDoc} disabled={formMode === 'edit'}>
                <option value="C">Cédula de Ciudadanía (C)</option>
                <option value="N">NIT (N)</option>
                <option value="E">Cédula de Extranjería (E)</option>
              </select>
            </div>
            
            <div class="form-group">
              <label for="numDoc">Número de Documento *</label>
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
              placeholder="Nombre principal" 
              bind:value={nombre} 
              required 
            />
          </div>

          <div class="form-group">
            <label for="apellido">Apellido (Opcional)</label>
            <input 
              id="apellido" 
              type="text" 
              class="form-control" 
              placeholder="Apellido si es persona natural" 
              bind:value={apellido} 
            />
          </div>
        </div>

        <div class="form-section">
          <h3>Contacto y Ubicación</h3>

          <div class="form-row">
            <div class="form-group">
              <label for="telefono1">Teléfono 1</label>
              <input 
                id="telefono1" 
                type="text" 
                class="form-control" 
                placeholder="Principal" 
                bind:value={telefono1} 
              />
            </div>
            
            <div class="form-group">
              <label for="telefono2">Teléfono 2</label>
              <input 
                id="telefono2" 
                type="text" 
                class="form-control" 
                placeholder="Alternativo" 
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
              placeholder="ejemplo@correo.com" 
              bind:value={email} 
            />
          </div>

          <div class="form-group">
            <label for="contacto">Contacto Adicional (Ruta / Encargado)</label>
            <input 
              id="contacto" 
              type="text" 
              class="form-control" 
              placeholder="Ej. Conductor o Asistente" 
              bind:value={contacto} 
            />
          </div>

          <div class="form-group">
            <label for="direccion1">Dirección</label>
            <input 
              id="direccion1" 
              type="text" 
              class="form-control" 
              placeholder="Calle / Carrera" 
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
              <select id="status" class="form-control" bind:value={status}>
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
            {saving ? 'Guardando...' : 'Guardar Proveedor'}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}

<!-- Secure Delete Modal -->
{#if supplierToDelete}
  <Modal 
    title="Confirmar Eliminación de Proveedor" 
    onclose={() => supplierToDelete = null}
    maxWidth="520px"
  >
    <div class="delete-warning-content">
      <div class="warning-banner">
        <span class="warning-icon">⚠️</span>
        <div class="warning-text">
          <strong>Advertencia de Integridad de Datos</strong>
          <p>Esta acción está conectada al POS de producción.</p>
        </div>
      </div>

      <div class="supplier-details-box">
        <div class="detail-item">
          <span class="lbl">Proveedor:</span>
          <span class="val">{supplierToDelete.nombre} {supplierToDelete.apellido || ''}</span>
        </div>
        <div class="detail-item">
          <span class="lbl">Documento:</span>
          <span class="val">({supplierToDelete.tipoDoc}) {supplierToDelete.numDoc}</span>
        </div>
        <div class="detail-item">
          <span class="lbl">Código POS:</span>
          <span class="val">{supplierToDelete.id}</span>
        </div>
      </div>

      <div class="delete-info-note">
        <p>
          <strong>Protocolo de Borrado Seguro:</strong>
        </p>
        <ul>
          <li>Si el proveedor **tiene compras registradas**, se desactivará lógicamente (`status = 'I'`) para no alterar el historial contable de la empresa (soft-delete).</li>
          <li>Si **no tiene compras registradas**, se eliminará físicamente de forma permanente de las tablas `proveedo` y `trc`.</li>
        </ul>
      </div>
    </div>

    {#snippet footer()}
      <button class="btn btn-secondary" onclick={() => supplierToDelete = null} disabled={deleting}>
        Cancelar
      </button>
      <button class="btn btn-danger shadow-glow" onclick={confirmDelete} disabled={deleting}>
        {deleting ? 'Eliminando...' : 'Confirmar Eliminación'}
      </button>
    {/snippet}
  </Modal>
{/if}

<style>
  .proveedores-view {
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

  /* Filters Card */
  .filters-card {
    padding: 20px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 20px;
    flex-wrap: wrap;
    background: rgba(13, 20, 37, 0.4);
  }

  .search-box {
    position: relative;
    flex: 1;
    min-width: 280px;
  }

  .search-icon {
    position: absolute;
    left: 14px;
    top: 50%;
    transform: translateY(-50%);
    font-size: 16px;
    color: var(--text-muted);
  }

  .search-input {
    padding-left: 42px;
    padding-right: 36px;
  }

  .clear-search {
    position: absolute;
    right: 12px;
    top: 50%;
    transform: translateY(-50%);
    background: transparent;
    border: none;
    color: var(--text-secondary);
    font-size: 18px;
    cursor: pointer;
  }

  /* Custom Checkbox */
  .checkbox-container {
    display: flex;
    align-items: center;
    position: relative;
    padding-left: 28px;
    cursor: pointer;
    font-size: 13px;
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
    height: 18px;
    width: 18px;
    background-color: rgba(13, 20, 37, 0.6);
    border: 1px solid var(--border-color);
    border-radius: 5px;
    transition: all 0.2s;
  }

  .checkbox-container:hover input ~ .checkmark {
    border-color: rgba(255, 255, 255, 0.2);
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
    left: 6px;
    top: 2px;
    width: 4px;
    height: 9px;
    border: solid #052e16;
    border-width: 0 2px 2px 0;
    transform: rotate(45deg);
  }

  /* Table styles */
  .table-container {
    overflow-x: auto;
    background: rgba(13, 20, 37, 0.35);
    border-radius: 20px;
    padding: 8px;
  }

  .suppliers-table {
    width: 100%;
    border-collapse: collapse;
    text-align: left;
    font-size: 13.5px;
  }

  .suppliers-table th {
    padding: 16px 20px;
    font-weight: 600;
    color: var(--text-muted);
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  }

  .suppliers-table td {
    padding: 16px 20px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.03);
    color: var(--text-primary);
  }

  .table-row {
    transition: background-color 0.2s;
  }

  .table-row:hover {
    background-color: rgba(255, 255, 255, 0.02);
  }

  .inactive-row {
    opacity: 0.6;
  }

  .supplier-code {
    font-family: monospace;
    font-weight: 600;
    color: var(--accent-green);
    font-size: 13px;
  }

  .supplier-name {
    font-weight: 600;
  }

  .supplier-email {
    max-width: 180px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .doc-badge {
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    padding: 2px 6px;
    font-size: 11px;
    font-weight: 600;
    margin-right: 6px;
    color: var(--text-secondary);
  }

  /* Status Badges */
  .status-badge {
    display: inline-flex;
    align-items: center;
    padding: 3px 10px;
    border-radius: 9999px;
    font-size: 11px;
    font-weight: 600;
    border: 1px solid transparent;
  }

  .status-badge.active {
    background: var(--accent-green-light);
    border-color: rgba(16, 185, 129, 0.2);
    color: #34d399;
  }

  .status-badge.inactive {
    background: var(--accent-red-light);
    border-color: rgba(239, 68, 68, 0.2);
    color: #f87171;
  }

  /* Actions */
  .actions-header {
    text-align: right;
  }

  .actions-cell {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }

  .action-btn {
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid var(--border-color);
    cursor: pointer;
    font-size: 13px;
    width: 32px;
    height: 32px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    border-radius: 8px;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .action-btn:hover {
    transform: translateY(-1px);
  }

  .action-btn.edit:hover {
    background: var(--accent-green-light);
    border-color: rgba(16, 185, 129, 0.3);
    color: var(--accent-green);
  }

  .action-btn.delete:hover {
    background: var(--accent-red-light);
    border-color: rgba(239, 68, 68, 0.3);
    color: var(--accent-red);
  }

  /* Empty State */
  .empty-state {
    padding: 60px 20px;
    text-align: center;
    color: var(--text-secondary);
  }

  .empty-icon {
    font-size: 40px;
    display: block;
    margin-bottom: 16px;
    opacity: 0.5;
  }

  .empty-state h3 {
    margin-bottom: 6px;
    font-weight: 600;
  }

  /* Slide-over Form Panel */
  .form-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(4, 6, 12, 0.5);
    backdrop-filter: blur(4px);
    -webkit-backdrop-filter: blur(4px);
    z-index: 900;
    display: flex;
    justify-content: flex-end;
  }

  .form-panel {
    background: #0d1425;
    width: 100%;
    max-width: 500px;
    height: 100%;
    box-shadow: -10px 0 40px -10px rgba(0, 0, 0, 0.8);
    display: flex;
    flex-direction: column;
    border-left: 1px solid var(--border-color);
  }

  .panel-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 24px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  }

  .close-panel {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    font-size: 28px;
    cursor: pointer;
    line-height: 1;
    padding: 4px;
  }

  .close-panel:hover {
    color: var(--text-primary);
  }

  .panel-body {
    flex: 1;
    overflow-y: auto;
    padding: 24px;
    display: flex;
    flex-direction: column;
    gap: 28px;
  }

  .form-section h3 {
    font-size: 13px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--accent-green);
    margin-bottom: 16px;
    border-left: 3px solid var(--accent-green);
    padding-left: 10px;
  }

  .panel-actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    padding: 20px 24px;
    border-top: 1px solid rgba(255, 255, 255, 0.05);
    background: rgba(9, 14, 26, 0.5);
  }

  /* Delete warning */
  .delete-warning-content {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .warning-banner {
    display: flex;
    align-items: center;
    gap: 14px;
    background: rgba(239, 68, 68, 0.07);
    border: 1px solid rgba(239, 68, 68, 0.2);
    border-radius: 12px;
    padding: 16px;
  }

  .warning-icon {
    font-size: 24px;
  }

  .warning-text strong {
    color: #fca5a5;
    font-size: 14px;
    display: block;
    margin-bottom: 2px;
  }

  .warning-text p {
    font-size: 12px;
    color: var(--text-secondary);
    margin: 0;
  }

  .supplier-details-box {
    background: rgba(255, 255, 255, 0.02);
    border: 1px solid var(--border-color);
    border-radius: 12px;
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .detail-item {
    display: flex;
    justify-content: space-between;
    font-size: 13.5px;
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
    font-size: 13px;
    line-height: 1.5;
  }

  .delete-info-note ul {
    margin-top: 8px;
    padding-left: 20px;
    display: flex;
    flex-direction: column;
    gap: 6px;
    color: var(--text-secondary);
  }
</style>
