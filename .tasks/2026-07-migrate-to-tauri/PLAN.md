# Plan de Migración: Node.js+Express → Tauri+Rust+SvelteKit

## Enfoque General
Migración incremental en 6 fases. Cada fase es independiente y verificable. El código Node.js existente se conserva como referencia hasta que su equivalente en Rust esté probado y funcionando.

La estructura objetivo del proyecto será:
```
InformesRecicladora/
├── src-tauri/              # Backend Rust (Tauri core)
│   ├── src/
│   │   ├── main.rs         # Entry point Tauri
│   │   ├── db.rs           # Pools MySQL (read/write separados)
│   │   ├── commands/       # Comandos IPC expuestos al frontend
│   │   │   ├── mod.rs
│   │   │   ├── proveedores.rs
│   │   │   └── informes.rs
│   │   ├── models/         # Structs de datos (serde Serialize/Deserialize)
│   │   │   ├── mod.rs
│   │   │   ├── proveedor.rs
│   │   │   └── informe.rs
│   │   └── utils/
│   │       └── year_table.rs
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── icons/
├── src/                    # Frontend SvelteKit
│   ├── lib/
│   │   ├── components/     # Componentes reutilizables
│   │   ├── stores/         # Svelte stores (estado global)
│   │   └── api.ts          # Wrapper de invoke() para comandos Tauri
│   ├── routes/
│   │   ├── +layout.svelte  # Layout con sidebar
│   │   ├── +page.svelte    # Dashboard/Home
│   │   ├── proveedores/
│   │   │   └── +page.svelte
│   │   └── informes/
│   │       └── +page.svelte
│   └── app.html
├── static/                 # Assets estáticos (fonts, icons)
├── db-init/                # Docker seed (se conserva)
│   └── seed.sql
├── server/                 # [LEGACY] Backend Node.js — se conserva como referencia
├── docker-compose.yml      # Se conserva para dev MySQL
├── svelte.config.js
├── vite.config.ts
├── package.json            # Ahora para SvelteKit + Tauri CLI
├── .tasks/                 # Spec-driven tasks
└── AGENTS.md
```

---

## Fase 1: Scaffolding Tauri + SvelteKit (Sin lógica de negocio)

### Archivos que se crean
- `src-tauri/src/main.rs` — Entry point mínimo de Tauri
- `src-tauri/Cargo.toml` — Dependencias Rust (tauri, sqlx, serde, tokio)
- `src-tauri/tauri.conf.json` — Config de ventana, título, permisos
- `svelte.config.js` — Config SvelteKit con adapter-static
- `vite.config.ts` — Config Vite con Tauri integration
- `src/app.html` — HTML base
- `src/routes/+page.svelte` — Página placeholder
- `package.json` — Se actualiza con deps de SvelteKit + @tauri-apps/cli

### Test de verificación
- `cargo tauri dev` abre una ventana nativa con el texto "Recicladora Boyacá"
- No hay errores de compilación Rust ni SvelteKit

### Riesgo
- **Prerrequisitos en la máquina del dev**: Necesita Rust toolchain (`rustup`), Node.js y WebView2 (viene con Windows 10/11). Verificar antes de empezar.

---

## Fase 2: Conexión MySQL desde Rust

### Archivos que se crean
- `src-tauri/src/db.rs` — Configuración de pools con `sqlx::MySqlPool`

### Lógica
- Crear `read_pool` y `write_pool` con credenciales de `.env`
- Manejar `mysql_native_password` para compatibilidad MySQL 5.5
- Exponer un comando `test_connection` que retorne `{ read: bool, write: bool }`
- Gestionar estado global con `tauri::State<AppState>`

### Test de verificación
- `cargo test` — test unitario que verifica que `read_pool` se conecta al Docker MySQL
- Desde el frontend: `invoke('test_connection')` retorna `{ read: true, write: true }`

### Decisión que necesita tu OK
> **¿`sqlx` o `mysql_async`?** Recomiendo `sqlx` por su tipado seguro y soporte de migraciones, pero necesito confirmar que funcione con MySQL 5.5. Haré un spike (prueba rápida) como primer paso de esta fase.

---

## Fase 3: Comandos Rust — CRUD Proveedores

### Archivos que se crean
- `src-tauri/src/models/proveedor.rs` — Structs `Proveedor`, `CreateProveedorInput`, `UpdateProveedorInput`
- `src-tauri/src/commands/proveedores.rs` — Comandos IPC:
  - `list_proveedores(include_inactive: bool) -> Vec<Proveedor>`
  - `get_proveedor(id: String) -> Proveedor`
  - `create_proveedor(input: CreateProveedorInput) -> String` (retorna ID)
  - `update_proveedor(id: String, input: UpdateProveedorInput)`
  - `delete_proveedor(id: String) -> DeleteResult { action, reason }`
- `src-tauri/src/utils/year_table.rs` — Port de `server/utils/yearTable.js`

### Lógica crítica a portar
La transacción de `create_proveedor` debe ser idéntica a la de Express:
1. Verificar duplicado por TRCNUMDOC
2. Si existe en trc pero no en proveedo → reutilizar TRCID, actualizar trc
3. Si no existe → generar nuevo PROCOD (prefijo + consecutivo), insertar en trc
4. Insertar en proveedo
5. Todo en transacción SQL

El `delete_proveedor` debe verificar historial en todas las tablas compra* usando `information_schema.TABLES`.

### Tests de verificación
- `cargo test` — tests unitarios de year_table (port de los 7 tests de Jest existentes)
- `cargo test` — test de integración create → list → delete con Docker MySQL
- Comparar output JSON de `list_proveedores` con el output actual de `GET /api/proveedores`

---

## Fase 4: Frontend SvelteKit — Layout y Proveedores

### Archivos que se crean
- `src/routes/+layout.svelte` — Sidebar + panel principal, dark mode
- `src/lib/components/Sidebar.svelte`
- `src/lib/components/Toast.svelte`
- `src/lib/components/Modal.svelte`
- `src/lib/components/SkeletonLoader.svelte`
- `src/lib/api.ts` — Wrapper tipado de `invoke()`
- `src/lib/stores/proveedores.ts` — Svelte store
- `src/routes/proveedores/+page.svelte` — Lista + CRUD completo
- `src/app.css` — Design system (dark mode, glassmorphism, variables CSS)

### Diseño visual
- Dark mode: `#111827` fondo, `#1f2937` cards, `#10b981` acento verde
- Tipografía: Inter (Google Fonts)
- Glassmorphism: `backdrop-filter: blur()` en paneles
- Micro-animaciones: transiciones en hover, fade-in, skeleton loaders
- Resolución mínima: 1024×768

### Test de verificación
- Crear un proveedor desde la UI → aparece en la lista
- Eliminar un proveedor sin compras → desaparece de la lista
- Eliminar un proveedor con compras → se muestra mensaje "desactivado"

---

## Fase 5: Comandos Rust + Frontend — Informes

### Archivos que se crean
- `src-tauri/src/commands/informes.rs` — Comandos:
  - `get_compras_acumuladas(fecha_inicio, fecha_fin) -> ComprasReport`
  - `get_resumen_caja(fecha_inicio, fecha_fin) -> ResumenCaja`
  - `get_productos() -> Vec<Producto>`
- `src-tauri/src/models/informe.rs` — Structs de reporte
- `src/routes/informes/+page.svelte` — Vista del informe
- `src/lib/pdf.ts` — Generación, previsualización y guardado del PDF (se mantiene en JS/cliente usando pdfmake).
- `src/lib/components/PDFPreviewModal.svelte` — [NEW] Modal de previsualización interactivo con un `<iframe>` usando el data URL generado por pdfmake.

### Lógica SQL (solo readPool)
- Compras acumuladas: JOIN dcmpr × compra × pas con UNION ALL multi-año
- Usar `COMEST = 'C'` para compras cerradas (no 'A')
- Usar `NULLIF(SUM(), 0)` para evitar división por cero
- Resumen de caja: 5 queries separadas (base, ingresos, ventas, compras, egresos)

### Gestión de PDF (Previsualización y Descarga)
- **Previsualización en Línea**: En lugar de descargar a ciegas, se utilizará `pdfDocGenerator.getDataUrl((dataUrl) => ...)` para cargar el PDF directamente en un iframe dentro del componente `PDFPreviewModal` y visualizarlo dentro del mismo Tauri Webview.
- **Ubicación de Descarga**:
  - Por defecto, las descargas del navegador en un Webview van a la carpeta de descargas del sistema (`C:\Users\Andres\Downloads` en Windows).
  - Para permitir cambiar esto, se implementará el plugin oficial de diálogos de Tauri (`@tauri-apps/plugin-dialog`) para que al presionar "Guardar Como", se le abra al usuario el explorador nativo de archivos permitiendo elegir la ubicación exacta (ej. Escritorio, descargas) y el nombre del archivo `.pdf`.

### Test de verificación
- Query con rango de fechas retorna datos agrupados correctamente.
- El PDF se renderiza y previsualiza en el modal interactivo sin errores de CORS o renderizado.
- El diálogo nativo de Tauri permite guardar el archivo en una ruta elegida por el usuario.
- El PDF descargado/guardado coincide exactamente con el formato de `info/image.png`.

---

## Fase 6: Auto-Updater + CI/CD + Release Pipeline

### Archivos que se crean/modifican
- `src-tauri/tauri.conf.json` — Sección `updater` con endpoint y pubkey
- `.github/workflows/release.yml` — Build + firma + publish con tauri-action
- `src-tauri/src/main.rs` — Integración del updater plugin

### Lógica
- Generar keypair con `tauri signer generate`
- Configurar GitHub Secrets: `TAURI_PRIVATE_KEY`, `TAURI_KEY_PASSWORD`
- Trigger de release: push de tag `v*`
- Artefacto: `.msi` firmado + `update.json` para auto-update

### Test de verificación
- Push de tag `v0.1.0` → GitHub Actions compila .msi → Release creado
- App instalada detecta nueva versión y muestra diálogo de actualización

### Riesgo
- **Firma digital**: Necesitas generar y guardar la clave privada de forma segura. Si se pierde, no puedes firmar nuevas releases.

---

## Autoevaluación del Plan

| Checklist | ✅/❌ |
|-----------|-------|
| ¿Los pasos son concretos con archivo/módulo identificado? | ✅ Cada fase lista archivos exactos |
| ¿Incluye tests, no solo cambios de código? | ✅ Cada fase tiene tests de verificación |
| ¿Identifica explícitamente qué archivos toca? | ✅ Estructura completa detallada |
| ¿Hay algo de alto riesgo que confirmar primero? | ✅ Driver sqlx vs mysql_async (Fase 2), keypair (Fase 6) |
