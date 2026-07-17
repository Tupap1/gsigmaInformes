# AGENTS.md — Informes Recicladora Boyacá

## ¿Qué es este proyecto?
Aplicativo de escritorio local para gestionar proveedores y generar informes de compras acumuladas de una recicladora. Se conecta a una base de datos MySQL 5.5 de un POS en producción (GsigmaPOS/Itheke).

## Stack Actual (en migración)
El proyecto se encuentra **en proceso de migración** del stack original a Tauri.

### Stack Original (Node.js) — Fases 1-2 completadas
- **Backend**: Node.js 20 + Express 4 + mysql2 (driver MySQL 5.5)
- **Frontend**: HTML/CSS/JS vanilla (placeholder, sin SPA real aún)
- **Tests**: Jest 29 + Supertest
- **Process Manager**: PM2 / nodemon (dev)
- **DB Dev**: Docker Compose con MySQL 5.5
- **CI**: GitHub Actions (`ci.yml`) — corre tests con MySQL 5.5 en Ubuntu

### Stack Objetivo (Tauri) — Migración pendiente
- **Desktop shell**: Tauri v2 (Rust)
- **Backend/Core**: Rust (comandos Tauri IPC) + sqlx (driver MySQL)
- **Frontend**: SvelteKit (compilado a estático dentro del Webview)
- **Tests**: Cargo test (Rust) + Vitest (SvelteKit)
- **Build/Release**: GitHub Actions con tauri-action (compilación Windows .msi)
- **Auto-update**: Tauri updater con firma criptográfica + GitHub Releases

## Base de Datos (MySQL 5.5 — Producción)

### REGLA SUPREMA: PROTECCIÓN DE LA BD
> Esta BD está en PRODUCCIÓN. Un error afecta la contabilidad real.
> Nunca asumas, siempre verifica. Nunca modifiques sin confirmar.

### Protocolo Obligatorio: TRAZADO DE DATOS
Antes de escribir código, traza el flujo COMPLETO:
1. ¿DE DÓNDE vienen los datos? → ¿Qué tabla? ¿Qué esquema (adm/pv/sis/tcc)?
2. ¿QUÉ forma tienen AHORA? → Ejecuta un SELECT. Lee los valores reales.
3. ¿CÓMO llegan a la UI? → ¿Qué query? ¿Qué campos devuelve el comando?
4. ¿DÓNDE se renderizan? → ¿Qué componente Svelte?
5. ¿CUÁL es la brecha? → ¿Falta en BD? ¿En el query? ¿En el render?

### Esquemas y Permisos
| Esquema | Permisos App |
|---------|-------------|
| `adm` | SELECT + INSERT/UPDATE en `trc` solamente |
| `pv` | SELECT + INSERT/UPDATE/DELETE en `proveedo` solamente. SELECT en el resto. |
| `sis` | SELECT ONLY |
| `tcc` | SELECT ONLY |

### PROHIBIDO
- `ALTER TABLE`, `DROP TABLE`, `CREATE TABLE` en tablas del POS
- `CREATE INDEX` sin autorización explícita
- Modificar stored procedures existentes
- `SELECT *` — especificar columnas explícitamente
- Queries sin filtro de fecha en tablas grandes (compra, venta, etc.)

### PERMITIDO
- `SELECT` en cualquier tabla
- `INSERT/UPDATE` solo en `adm.trc` y `pv.proveedo`
- `DELETE` solo en `pv.proveedo` y `adm.trc` (borrado seguro de proveedores sin historial)

### Tablas Particionadas por Año
`compra`, `dcmpr`, `venta`, `dvent`, `karde` tienen copias anuales (sufijo 11-35).
Para multi-año usar UNION ALL.

### Relaciones Clave (lógicas, sin FK)
- `adm.trc.TRCID` ↔ `pv.proveedo.PROCOD`
- `pv.pas.PAS` ↔ `pv.dcmpr.DCMART`
- `pv.compra.COMNUM` ↔ `pv.dcmpr.DCMNUM`
- `pv.compra.COMPRO` ↔ `pv.proveedo.PROCOD`

## Convenciones Detectadas en el Código Actual

### Naming
- **Variables/funciones JS**: camelCase (`readPool`, `writePool`, `getYearTables`)
- **Columnas BD**: UPPER_CASE (`TRCID`, `COMNUM`, `PASNOM`) — respetar sin cambiar
- **Archivos JS**: camelCase (`yearTable.js`, `errorHandler.js`)
- **Rutas API**: kebab-case (`/api/proveedores`, `/api/informes/compras-acumuladas`)

### Patrones
- **Error handling**: Clase `AppError` con `isPublic` flag. Errores SQL nunca se envían al cliente.
- **Transacciones**: `getConnection()` → `beginTransaction()` → operaciones → `commit()` / `rollback()` en finally con `release()`
- **Validación**: Server-side obligatoria antes de queries. Frontend no se confía.
- **Pools separados**: `readPool` (informes, solo SELECT) y `writePool` (CRUD proveedores)
- **Borrado seguro**: Verificar historial de compras → hard-delete si sin compras, soft-delete (`status='I'`) si tiene compras

## Scripts del Proyecto (package.json actual)
```bash
npm start        # node server/index.js
npm run dev      # nodemon server/index.js
npm test         # jest --runInBand --detectOpenHandles
```

## Docker (Desarrollo)
```bash
docker compose up -d    # MySQL 5.5 en puerto 3309, seed.sql automático
docker compose down -v  # Destruir todo
```

## Zonas Sensibles — NO TOCAR sin preguntar
1. **db-init/seed.sql** — Contiene GRANTs de privilegios MySQL. Cambiar privilegios puede romper la separación read/write.
2. **Cualquier query INSERT/UPDATE/DELETE** — Debe pasar por el `writePool` y estar en transacción.
3. **Estructura de tablas del POS** — NUNCA se modifica. Solo se lee.
4. **Lógica de borrado seguro** en `server/routes/proveedores.js` (DELETE endpoint) — Toca 2 tablas con validación histórica.

## Spec-Driven Development
- Toda tarea vive en `.tasks/<YYYY-MM-descripcion>/`
- Archivos requeridos: `REQUIREMENTS.md`, `PLAN.md`, `TASKS.md`, `STATUS.md`
- Nunca implementar sin PLAN.md aprobado
- Cada tarea se marca "done" solo cuando su test pasa

## Reglas Obligatorias: Diseño Anti-Vibe-Coded (Evitar Señales de IA Genérica)
Para cualquier desarrollo del frontend en SvelteKit, el agente debe auditar y eliminar elementos que hagan que la aplicación se sienta "vibe-coded" (plantilla genérica generada por IA sin curación).

### 1. Sistema de Color e Identidad
- **PROHIBIDO**: Usar gradientes de fondo morados/violetas/azules sin propósito corporativo.
- **PROHIBIDO**: Usar la paleta de colores por defecto de Tailwind sin calibración.
- **REGLA**: Usar una escala de grises de fondo oscura calibrada (`#0d1425` o similar) con un único color de marca verde ecológico (`#10b981`) con propósito visual (CTAs, estados activos, badges de conexión exitosa).

### 2. Estructura y Layout
- **PROHIBIDO**: Usar el layout predecible de cuadrículas simétricas repetitivas en cada sección.
- **REGLA**: Diseñar interfaces de software de escritorio utilitarias (SaaS robusto), priorizando tablas de datos densas, paneles laterales deslizables (slide-over) y tarjetas financieras claras con números grandes y legibles.

### 3. Componentes y Botones
- **PROHIBIDO**: Envolver todo en cards `rounded-2xl shadow-lg` idénticas.
- **PROHIBIDO**: Usar iconos genéricos de decoración (Lucide cohete, rayo, etc.) sin relación lógica directa.
- **REGLA**: Usar bordes semitransparentes finos (`border: 1px solid rgba(255,255,255,0.05)`) para el efecto de cristal (glassmorphism) sobre fondos oscuros. Los botones deben tener un estado hover suave y un indicador de carga real (no decorativo).

### 4. Copy e Integridad Visual
- **PROHIBIDO**: Textos en inglés o copy genérico de marketing SaaS ("Optimiza tu flujo de trabajo").
- **REGLA**: Toda la interfaz debe estar en español técnico del negocio. El copy debe reflejar el problema exacto del POS (ej. "Historial de Compras Acumuladas", "Resumen de Caja del Período").

### 5. Animaciones e Interacciones
- **PROHIBIDO**: Animaciones de scroll excesivas, blobs de colores flotantes o movimientos innecesarios.
- **REGLA**: Limitar las animaciones a micro-interacciones funcionales de la UI: efectos shimmer de carga (Skeleton Loaders), toasts de notificación que se deslizan brevemente y un pulso suave de conectividad en los estados de base de datos.

