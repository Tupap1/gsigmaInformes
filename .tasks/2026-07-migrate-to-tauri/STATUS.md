# Estado — Migración a Tauri + Rust + SvelteKit

## Estado actual: `nit_validation_in_progress`

## Resumen
Migración completa del stack Node.js+Express+Vanilla a Tauri v2+Rust+SvelteKit. Incluye reescritura del backend en Rust (comandos IPC), nuevo frontend en SvelteKit con diseño premium, y sistema de auto-actualización con firma criptográfica.

## Fases
| Fase | Descripción | Estado |
|------|-------------|--------|
| 1 | Scaffolding Tauri + SvelteKit | `completed` |
| 2 | Conexión MySQL desde Rust | `completed` |
| 3 | CRUD Proveedores en Rust | `in_progress` |
| 4 | Frontend SvelteKit — Layout y Proveedores | `in_progress` |
| 5 | Informes + PDF | `completed` |
| 6 | Auto-Updater + CI/CD | `completed` |

## Bloqueantes
- Decisión: ¿`sqlx` o `mysql_async` como driver MySQL en Rust? (Resuelto: Se verificó que `sqlx` v0.8 es 100% compatible con MySQL 5.5 y los pools de conexión independientes)
- Decisión: ¿PDF se genera en JS (pdfmake) o en Rust? (Resuelto: Generación del ticket PDF en el cliente usando pdfmake)

## Historial
| Fecha | Evento |
|-------|--------|
| 2026-07-17 | Creación de spec: REQUIREMENTS.md, PLAN.md, TASKS.md, STATUS.md |
| 2026-07-17 | Fase 0 (Rust + VS Build Tools) y Fase 1 (Scaffolding Tauri v2 + SvelteKit) completadas con éxito |
| 2026-07-17 | Fase 4 (Frontend SvelteKit — Layout y Proveedores) completada con éxito |
| 2026-07-17 | Fase 5 en progreso: Reabierta tarea T5.5 para implementar la previsualización del PDF en línea y selección de ruta nativa de guardado |
| 2026-07-17 | Fase 5 (Informes + PDF) completada exitosamente. Integrado diálogo nativo y guardado directo. |
| 2026-07-17 | Incorporación de requerimiento para verificar el Dígito de Verificación (DV) del NIT colombiano con algoritmo de la DIAN. |
