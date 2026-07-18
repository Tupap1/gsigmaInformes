# Tareas — Migración a Tauri + Rust + SvelteKit

## Fase 0: Prerrequisitos (Instalación del Toolchain)
- [x] **T0.1** Instalar Rust toolchain con rustup → Ejecutar instalador de https://rustup.rs — Test: `rustc --version` retorna versión estable
- [x] **T0.2** Verificar cargo funcional → Test: `cargo --version` exitoso
- [x] **T0.3** Verificar WebView2 instalado (viene con Windows 10/11) → Test: app de ejemplo Tauri abre ventana
- [x] **T0.4** Instalar Tauri CLI → `cargo install tauri-cli` — Test: `cargo tauri --version` exitoso

## Fase 1: Scaffolding
- [x] **T1.1** Verificar prerrequisitos (Rust, WebView2) → `rustc --version` y `cargo --version` exitosos
- [x] **T1.2** Inicializar SvelteKit con adapter-static → `svelte.config.js`, `vite.config.ts`, `src/app.html` — Test: `npm run build` sin errores
- [x] **T1.3** Inicializar Tauri v2 en el proyecto → `src-tauri/`, `Cargo.toml`, `tauri.conf.json` — Test: `cargo tauri dev` abre ventana
- [x] **T1.4** Crear página placeholder en SvelteKit → `src/routes/+page.svelte` — Test: ventana muestra "Recicladora Boyacá"

## Fase 2: Conexión MySQL desde Rust
- [x] **T2.1** Spike: verificar que sqlx funciona con MySQL 5.5 Docker → script de prueba en `src-tauri/src/` — Test: SELECT 1 exitoso
- [x] **T2.2** Crear módulo `db.rs` con read_pool + write_pool → `src-tauri/src/db.rs` — Test: `cargo test test_pools` pasa
- [x] **T2.3** Crear comando `test_connection` IPC → `src-tauri/src/commands/mod.rs` — Test: frontend `invoke('test_connection')` retorna `{read: true, write: true}`
- [x] **T2.4** Configurar `.env` para Tauri (reutilizar variables existentes) → `.env` actualizado — Test: `cargo tauri dev` se conecta al Docker MySQL

## Fase 3: CRUD Proveedores en Rust
- [x] **T3.1** Port `year_table.rs` desde `yearTable.js` → `src-tauri/src/utils/year_table.rs` — Test: 7 tests equivalentes a Jest pasan con `cargo test`
- [x] **T3.2** Crear struct `Proveedor` + serialización → `src-tauri/src/models/proveedor.rs` — Test: serializa/deserializa JSON correctamente
- [x] **T3.3** Comando `list_proveedores` → `src-tauri/src/commands/proveedores.rs` — Test: retorna misma data que `GET /api/proveedores` actual
- [x] **T3.4** Comando `get_proveedor(id)` → mismo archivo — Test: retorna proveedor por ID
- [x] **T3.5** Comando `create_proveedor` con transacción dual → mismo archivo — Test: crea en trc + proveedo, rollback si falla
- [x] **T3.6** Comando `update_proveedor` con transacción → mismo archivo — Test: actualiza campos editables
- [x] **T3.7** Comando `delete_proveedor` con borrado seguro → mismo archivo — Test: hard-delete sin compras, soft-delete con compras
- [x] **T3.8** Lógica de validación de NIT (DIAN Módulo 11) en Rust ➔ `src-tauri/src/utils/validation.rs` — Test: `cargo test test_nit_validation` valida casos válidos y erróneos
- [x] **T3.9** Comando `setup_db_connection` en Rust (conexión temporal as root, creación/GRANT de reci_read y reci_write en MySQL, y guardado de `config.json` en AppData) ➔ `src-tauri/src/commands/setup.rs` — Test: `cargo test test_setup_db_connection` con root pass de Docker

## Fase 4: Frontend SvelteKit — Layout y Proveedores
- [x] **T4.1** Design system CSS (dark mode, variables, animaciones) → `src/app.css` — Test: visual review
- [x] **T4.2** Layout con Sidebar + panel dinámico → `src/routes/+layout.svelte`, `src/lib/components/Sidebar.svelte` — Test: navegación entre secciones funciona
- [x] **T4.3** Componentes base (Toast, Modal, SkeletonLoader) → `src/lib/components/` — Test: cada componente renderiza sin errores
- [x] **T4.4** API wrapper tipado → `src/lib/api.ts` — Test: wrapper de invoke funciona con test_connection
- [x] **T4.5** Vista lista de proveedores con búsqueda → `src/routes/proveedores/+page.svelte` — Test: lista carga, búsqueda filtra en tiempo real
- [x] **T4.6** Formulario crear/editar proveedor → mismo archivo — Test: crear proveedor → aparece en lista
- [x] **T4.7** Modal borrado seguro con feedback → mismo archivo — Test: soft/hard delete muestra mensaje correcto
- [x] **T4.8** Integración de validación de NIT colombiano en el formulario de creación/edición ➔ `src/routes/proveedores/+page.svelte` — Test: NITs inválidos muestran alerta en tiempo real y bloquean el envío.
- [x] **T4.9** Vista/Modal del Asistente de Configuración Inicial (Wizard) ➔ `src/routes/+page.svelte` — Test: si la app no está configurada, bloquea navegación, pide Host/Port/Clave root y realiza la llamada exitosa.

## Fase 5: Informes + PDF
- [x] **T5.1** Comandos Rust `get_compras_acumuladas` + `get_resumen_caja` → `src-tauri/src/commands/informes.rs` — Test: `cargo test` con datos seed
- [x] **T5.2** Comando `get_productos` → mismo archivo — Test: retorna lista de PAS con PASCOMP=1
- [x] **T5.3** Vista informe con filtros y tabla → `src/routes/informes/+page.svelte` — Test: genera tabla con datos correctos
- [x] **T5.4** Resumen de caja visual (tarjetas financieras) → mismo archivo — Test: cálculos coinciden con datos manuales
- [x] **T5.5** Generación PDF réplica y Diálogo de Guardado Nativo → `src/lib/pdf.ts` & `src/routes/informes/+page.svelte` — Test: hacer clic en "Exportar PDF" abre el diálogo de guardado de Windows y guarda el PDF en la ruta seleccionada.

## Fase 6: Auto-Updater + CI/CD
- [x] **T6.1** Generar keypair de firma → almacenar en GitHub Secrets — Test: `tauri signer generate` exitoso
- [x] **T6.2** Configurar updater en `tauri.conf.json` → `src-tauri/tauri.conf.json` — Test: app chequea endpoint al iniciar
- [x] **T6.3** Crear workflow `release.yml` → `.github/workflows/release.yml` — Test: push tag `v0.1.0` → .msi en GitHub Releases
- [ ] **T6.4** Verificar auto-update end-to-end → instalar v0.1.0, publicar v0.2.0 — Test: app detecta y aplica actualización
