# Migración a Tauri + SvelteKit + Rust — Requisitos

## Problema
El proyecto actualmente usa Node.js + Express como backend web local. Esto requiere instalar Node.js, Git y PM2 en cada máquina del cliente, las actualizaciones son manuales (git pull + script .bat), y la app se abre en una pestaña del navegador, no como programa nativo.

Se necesita migrar a una aplicación de escritorio nativa con Tauri que:
- Se instale como un `.exe` / `.msi` sin dependencias externas
- Se auto-actualice de forma transparente y segura
- Consuma mínima RAM (~25MB vs ~120MB actual)
- Mantenga toda la funcionalidad existente del CRUD de proveedores y los informes

## Funcionalidad Existente a Preservar

### F1: CRUD de Proveedores (ya implementado en Express)
- `GET /api/proveedores` — Lista proveedores activos con JOIN `pv.proveedo` ↔ `adm.trc`
- `GET /api/proveedores/:id` — Detalle de un proveedor
- `POST /api/proveedores` — Creación transaccional en `adm.trc` + `pv.proveedo` con validación de duplicados
- `PUT /api/proveedores/:id` — Actualización transaccional de datos no-financieros
- `DELETE /api/proveedores/:id` — Borrado seguro (hard-delete si sin compras, soft-delete si tiene historial)

### F2: Informe "Compras Acumuladas" (aún no implementado)
- Filtro por rango de fechas
- Agrupamiento por material (PAS) con cantidad, total y costo promedio ponderado
- Resumen de caja del período (base, ingresos, ventas contado/crédito, compras, egresos)
- Exportación a PDF réplica del ticket físico

### F3: Utilidades (ya implementadas)
- Year Table Resolver: resuelve tablas particionadas por año (compra11-compra35)
- Pools separados de lectura/escritura
- Error handler que oculta errores SQL del cliente

## Criterios de Aceptación

### CA1: Estructura Tauri funcional
- [x] `cargo tauri dev` levanta la app de escritorio con ventana nativa
- [x] El frontend SvelteKit se renderiza dentro del Webview
- [x] El proyecto compila sin errores en Windows

### CA2: Backend Rust con conexión MySQL 5.5
- [x] Comando Tauri `list_proveedores` retorna los mismos datos que `GET /api/proveedores`
- [x] Comando `create_proveedor` ejecuta la misma transacción dual (trc + proveedo)
- [x] Comando `delete_proveedor` implementa la lógica de borrado seguro idéntica
- [x] Los pools de lectura y escritura están separados en Rust
- [x] `cargo test` pasa con la BD Docker de desarrollo

### CA3: Frontend SvelteKit
- [x] Vista de lista de proveedores con búsqueda en tiempo real
- [x] Formulario de creación/edición con validación client-side
- [x] Modal de confirmación para borrado con feedback (hard/soft delete)
- [ ] Vista de informe con filtros de fecha y tabla de resultados
- [ ] Exportación PDF funcional
- [ ] Diseño dark mode premium (glassmorphism, micro-animaciones)

### CA4: Auto-updater
- [ ] La app chequea actualizaciones al iniciar desde un endpoint configurable
- [ ] Las actualizaciones se descargan y aplican automáticamente con firma digital
- [ ] GitHub Actions compila `.msi`, firma y publica en GitHub Releases

### CA5: Docker dev environment
- [x] `docker compose up -d` levanta MySQL 5.5 con seed data idéntica
- [x] `cargo tauri dev` se conecta correctamente al Docker MySQL

## Fuera de Alcance
- Módulo de ventas (solo lectura para informes)
- Autenticación de usuarios (app local sin login)
- Soporte multi-empresa (solo EMPID hardcodeado)
- Soporte macOS/Linux (solo Windows por ahora)

## Preguntas Abiertas
1. **Driver MySQL en Rust**: ¿Usar `sqlx` (async, type-safe) o `mysql` crate (más simple)? `sqlx` tiene mejor soporte pero requiere verificar compatibilidad con MySQL 5.5.
2. **Generación PDF**: ¿Mantener pdfmake en el frontend (SvelteKit/JS) o mover a Rust con `printpdf`/`genpdf`?
3. **Datos de seed**: ¿El `seed.sql` actual es suficiente para testear los informes o necesitamos datos de venta/caja también?
