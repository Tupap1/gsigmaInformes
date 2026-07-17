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

## Fase 2: Conexión MySQL desde Rust (y Asistente de Configuración)

### Archivos que se crean/modifican
- `src-tauri/src/db.rs` — Configuración de pools con `sqlx::MySqlPool` y carga/escritura de `config.json` en AppData.
- `src-tauri/src/commands/mod.rs` & `commands/db_init.rs` — Comando `setup_db_connection(host, port, root_password)` y `test_connection`.

### Lógica
- **Archivo de Configuración:** En producción se buscará `config.json` en la ruta de AppData (`C:\Users\<User>\AppData\Roaming\com.recicladoraboyaca.informes\config.json`). Si no existe o la conexión falla, la app inicia en "Modo Configuración".
- **Asistente Automático:** Al ingresar los datos de host, puerto y contraseña de `root`:
  1. Conecta temporalmente a MySQL como `root`.
  2. Crea las bases de datos si no existen.
  3. Crea los usuarios restringidos `reci_read` (clave `read_pass_123`) y `reci_write` (clave `write_pass_123`) con sus respectivos GRANTs mínimos si no existen.
  4. Ejecuta `FLUSH PRIVILEGES`.
  5. Escribe el archivo `config.json` con el host, puerto y las credenciales restringidas (nunca guarda la clave de root en disco).
  6. Levanta los pools definitivos de lectura/escritura y cambia el estado de la app a "Conectado".
- Manejar `mysql_native_password` para compatibilidad MySQL 5.5.
- Exponer un comando `test_connection` que retorne `{ read: bool, write: bool }` y `is_configured: bool`.
- Gestionar estado global con `tauri::State<AppState>` mutando los pools de forma segura.

### Test de verificación
- `cargo test` — test unitario que verifica que la inicialización y el parser de `config.json` funcionan.
- Simular flujo de creación de usuarios usando la contraseña de root de Docker (`devrootpassword`).

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
- `src/lib/pdf.ts` — Generación y guardado del PDF (se mantiene en JS/cliente usando pdfmake).

### Lógica SQL (solo readPool)
- Compras acumuladas: JOIN dcmpr × compra × pas con UNION ALL multi-año
- Usar `COMEST = 'C'` para compras cerradas (no 'A')
- Usar `NULLIF(SUM(), 0)` para evitar división por cero
- Resumen de caja: 5 queries separadas (base, ingresos, ventas, compras, egresos)

### Flujo de Consulta y Guardado de PDF
1. **Consulta en el Sistema**: El usuario introduce fechas y presiona "Consultar". La aplicación renderiza en pantalla una tabla HTML interactiva con las compras agrupadas y tarjetas con el resumen de caja.
2. **Generación e Integridad**: Una vez revisados los datos en pantalla, el usuario presiona "Exportar PDF". `pdfmake` genera el documento en memoria.
3. **Diálogo de Selección de Ruta**: Se implementará el plugin oficial de diálogos de Tauri (`@tauri-apps/plugin-dialog`) para que al presionar el botón de exportar, se le abra al usuario el explorador nativo de Windows ("Guardar Como"). El usuario elije la carpeta (ej. Escritorio, Descargas) y el nombre exacto del archivo `.pdf`.
4. **Escritura del Archivo**: Se guardará el binario del PDF en la ruta seleccionada.

### Test de verificación
- La consulta en pantalla muestra las compras y totales de caja calculados correctamente.
- Al exportar, se abre el diálogo nativo de guardado de archivos de Windows.
- El PDF se guarda en la ubicación elegida por el usuario y su formato coincide exactamente con `info/image.png`.

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
