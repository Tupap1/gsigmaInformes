use crate::db::{AppDb, AppState, get_db};
use crate::models::informe::{CompraAcumulada, Producto, ResumenCaja};
use crate::utils::year_table::get_year_tables;
use sqlx::{query, Row};
use tauri::State;

fn get_dcmpr_table(compra_table: &str) -> String {
    if compra_table == "compra" {
        "dcmpr".to_string()
    } else {
        let suffix = &compra_table[6..];
        format!("dcmpr{}", suffix)
    }
}

// -------------------------------------------------------------------------
// Core execution functions for unit testing
// -------------------------------------------------------------------------
pub async fn perform_get_productos(db: &AppDb) -> Result<Vec<Producto>, String> {
    let query_str = "SELECT TRIM(PAS) AS pas, TRIM(PASNOM) AS pasnom FROM pv.pas WHERE PASCOMP = 1 ORDER BY PASNOM ASC";

    let result = sqlx::query_as::<_, Producto>(query_str)
        .fetch_all(&db.read_pool)
        .await
        .map_err(|e| {
            log::error!("Error fetching products: {:?}", e);
            "Ocurrió un error al obtener la lista de productos.".to_string()
        })?;

    Ok(result)
}

pub async fn perform_get_compras_acumuladas(
    db: &AppDb,
    fecha_inicio: String,
    fecha_fin: String,
) -> Result<Vec<CompraAcumulada>, String> {
    let tables = get_year_tables("compra", &fecha_inicio, &fecha_fin)?;
    if tables.is_empty() {
        return Ok(Vec::new());
    }

    let mut union_parts = Vec::new();
    for compra_table in &tables {
        let dcmpr_table = get_dcmpr_table(compra_table);
        union_parts.push(format!(
            r#"
            SELECT 
              d.DCMART AS pas, 
              (SUM(d.DCMCAN) * 1e0) AS cantidad, 
              (SUM(d.DCMCOS * d.DCMCAN) * 1e0) AS total 
            FROM pv.{} d 
            INNER JOIN pv.{} c ON d.DCMNUM = c.COMNUM 
            WHERE c.COMFEC >= ? AND c.COMFEC <= ? AND c.COMEST = 'C' 
            GROUP BY d.DCMART
            "#,
            dcmpr_table, compra_table
        ));
    }

    let union_sql = union_parts.join(" UNION ALL ");
    let query_sql = format!(
        r#"
        SELECT 
          TRIM(p.PAS) AS pas,
          TRIM(p.PASNOM) AS nombre,
          COALESCE(sub.cantidad, 0.0) AS cantidad,
          COALESCE(sub.total, 0.0) AS total,
          COALESCE(sub.total / NULLIF(sub.cantidad, 0), 0.0) AS costo_promedio
        FROM pv.pas p
        INNER JOIN (
          SELECT 
            pas,
            SUM(cantidad) AS cantidad,
            SUM(total) AS total
          FROM ({}) AS u
          GROUP BY pas
        ) AS sub ON p.PAS = sub.pas
        ORDER BY p.PASNOM ASC
        "#,
        union_sql
    );

    let mut query = sqlx::query_as::<_, CompraAcumulada>(&query_sql);
    for _ in 0..tables.len() {
        query = query.bind(&fecha_inicio).bind(&fecha_fin);
    }

    let result = query.fetch_all(&db.read_pool).await.map_err(|e| {
        log::error!("Error executing compras acumuladas: {:?}", e);
        "Ocurrió un error al generar el informe de compras acumuladas.".to_string()
    })?;

    Ok(result)
}

pub async fn perform_get_resumen_caja(
    db: &AppDb,
    fecha_inicio: String,
    fecha_fin: String,
) -> Result<ResumenCaja, String> {
    // 1. Base Caja
    let base_caja: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(BAS_VALOR), 0.0) FROM pv.bas_caj WHERE BAS_FECHA >= ? AND BAS_FECHA <= ? AND BASEST != 'A'"
    )
    .bind(&fecha_inicio)
    .bind(&fecha_fin)
    .fetch_one(&db.read_pool)
    .await
    .map_err(|e| {
        log::error!("Error fetching base_caja: {:?}", e);
        "Error al obtener base de caja.".to_string()
    })?;

    // 2. Ingresos
    let ingresos: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(INGVALOR), 0.0) FROM pv.ingcaj WHERE INGFECHA >= ? AND INGFECHA <= ? AND INGESTADO != 'A'"
    )
    .bind(&fecha_inicio)
    .bind(&fecha_fin)
    .fetch_one(&db.read_pool)
    .await
    .map_err(|e| {
        log::error!("Error fetching ingresos: {:?}", e);
        "Error al obtener ingresos.".to_string()
    })?;

    // 3. Egresos (Desactivado a solicitud de la recicladora; control independiente)
    let egresos: f64 = 0.0;

    // 4. Ventas (Contado & Crédito)
    let venta_tables = get_year_tables("venta", &fecha_inicio, &fecha_fin)?;
    let (ventas_contado, ventas_credito) = if venta_tables.is_empty() {
        (0.0, 0.0)
    } else {
        let mut union_parts = Vec::new();
        for t in &venta_tables {
            union_parts.push(format!(
                r#"
                SELECT 
                  COALESCE(SUM(CASE WHEN TRNTIPO != 'CRE' THEN VENVAL ELSE 0.0 END), 0.0) AS contado, 
                  COALESCE(SUM(CASE WHEN TRNTIPO = 'CRE' THEN VENVAL ELSE 0.0 END), 0.0) AS credito 
                FROM pv.{} 
                WHERE VENFEC >= ? AND VENFEC <= ? AND (VENEST = 'C' OR VENEST != 'A')
                "#,
                t
            ));
        }
        let union_sql = union_parts.join(" UNION ALL ");
        let query_sql = format!(
            "SELECT COALESCE(SUM(contado), 0.0) AS total_contado, COALESCE(SUM(credito), 0.0) AS total_credito FROM ({}) AS u",
            union_sql
        );

        let mut q = query(&query_sql);
        for _ in 0..venta_tables.len() {
            q = q.bind(&fecha_inicio).bind(&fecha_fin);
        }

        let row = q.fetch_one(&db.read_pool).await.map_err(|e| {
            log::error!("Error executing sales summary: {:?}", e);
            "Error al calcular resumen de ventas.".to_string()
        })?;

        let contado: f64 = row.try_get("total_contado").unwrap_or(0.0);
        let credito: f64 = row.try_get("total_credito").unwrap_or(0.0);
        (contado, credito)
    };

    // 5. Compras (Suma exacta del informe de compras acumuladas por material)
    let compras_list = perform_get_compras_acumuladas(db, fecha_inicio.clone(), fecha_fin.clone()).await?;
    let compras: f64 = compras_list.iter().map(|item| item.total).sum();

    // Formulas
    let caja_efectivo = base_caja + ingresos + ventas_contado - compras - egresos;
    let caja_total = caja_efectivo + ventas_credito;

    Ok(ResumenCaja {
        base_caja,
        ingresos,
        ventas_contado,
        ventas_credito,
        compras,
        egresos,
        caja_efectivo,
        caja_total,
    })
}

// -------------------------------------------------------------------------
// Tauri commands
// -------------------------------------------------------------------------
#[tauri::command]
pub async fn get_productos(app_state: State<'_, AppState>) -> Result<Vec<Producto>, String> {
    let db = get_db(&app_state).await?;
    perform_get_productos(&db).await
}

#[tauri::command]
pub async fn get_compras_acumuladas(
    app_state: State<'_, AppState>,
    fecha_inicio: String,
    fecha_fin: String,
) -> Result<Vec<CompraAcumulada>, String> {
    let db = get_db(&app_state).await?;
    perform_get_compras_acumuladas(&db, fecha_inicio, fecha_fin).await
}

#[tauri::command]
pub async fn get_resumen_caja(
    app_state: State<'_, AppState>,
    fecha_inicio: String,
    fecha_fin: String,
) -> Result<ResumenCaja, String> {
    let db = get_db(&app_state).await?;
    perform_get_resumen_caja(&db, fecha_inicio, fecha_fin).await
}

#[tauri::command]
pub async fn save_pdf_file(path: String, content: Vec<u8>) -> Result<(), String> {
    tokio::fs::write(&path, content).await.map_err(|e| {
        log::error!("Error saving PDF file: {:?}", e);
        format!("No se pudo escribir el archivo en la ruta especificada: {}", e)
    })
}

// -------------------------------------------------------------------------
// Tests (validating commands against test seed)
// -------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[tokio::test]
    async fn test_perform_get_productos() {
        let db = db::create_pools()
            .await
            .expect("Failed to initialize pools");
        let prods = perform_get_productos(&db).await.unwrap();
        assert!(!prods.is_empty());
        assert_eq!(prods[0].pas, "002"); // ALUMNIO should be first since ordered by name
        assert_eq!(prods[0].pasnom, "ALUMINIO");
    }

    #[tokio::test]
    async fn test_perform_compras_acumuladas() {
        let db = db::create_pools()
            .await
            .expect("Failed to initialize pools");

        // Querying current year 2026 range
        let report =
            perform_get_compras_acumuladas(&db, "2026-07-01".to_string(), "2026-07-31".to_string())
                .await
                .unwrap();
        assert!(!report.is_empty());
        
        // Verificar invariante: costo promedio = total / cantidad para cada fila
        for item in &report {
            if item.cantidad > 0.0 {
                let expected_prom = (item.total / item.cantidad * 100.0).round() / 100.0;
                let actual_prom = (item.costo_promedio * 100.0).round() / 100.0;
                assert_eq!(actual_prom, expected_prom);
            }
        }

        // Querying historical range spanning 2024 to 2026
        let report_multi =
            perform_get_compras_acumuladas(&db, "2024-01-01".to_string(), "2026-07-31".to_string())
                .await
                .unwrap();
        assert!(report_multi.len() >= 2); // Debe contener al menos Cobre y Aluminio
    }

    #[tokio::test]
    async fn test_perform_resumen_caja() {
        let db = db::create_pools()
            .await
            .expect("Failed to initialize pools");

        let resumen =
            perform_get_resumen_caja(&db, "2026-07-01".to_string(), "2026-07-31".to_string())
                .await
                .unwrap();

        // Verificar invariante matemático de flujo de caja
        let expected_caja_efectivo = resumen.base_caja 
            + resumen.ingresos 
            + resumen.ventas_contado 
            - resumen.compras 
            - resumen.egresos;
        assert_eq!(resumen.caja_efectivo, expected_caja_efectivo);

        let expected_caja_total = resumen.caja_efectivo + resumen.ventas_credito;
        assert_eq!(resumen.caja_total, expected_caja_total);
    }
}
