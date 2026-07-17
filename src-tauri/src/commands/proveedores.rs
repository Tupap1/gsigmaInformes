use tauri::State;
use crate::db::AppDb;
use crate::models::proveedor::{Proveedor, CreateProveedorInput, UpdateProveedorInput, DeleteResult};

fn map_upper_or_empty(val: &Option<String>) -> String {
    val.as_ref()
        .map(|s| s.trim().to_uppercase())
        .unwrap_or_default()
}

fn map_or_empty(val: &Option<String>) -> String {
    val.as_ref()
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

// core implementation functions to facilitate testing
pub async fn perform_list_proveedores(
    db: &AppDb,
    include_inactive: Option<bool>,
) -> Result<Vec<Proveedor>, String> {
    let include_inactive = include_inactive.unwrap_or(false);
    let mut query = String::from(
        r#"
      SELECT 
        TRIM(p.PROCOD) AS id,
        TRIM(p.PRONUMDOC) AS num_doc,
        p.PROTIPDOC AS tipo_doc,
        TRIM(p.PROEMA) AS email,
        TRIM(p.PROCON) AS contacto,
        p.status AS status,
        p.pais AS pais,
        TRIM(t.TRCNOM) AS nombre,
        TRIM(t.TRCAPE) AS apellido,
        TRIM(t.TRCTEL1) AS telefono1,
        TRIM(t.TRCTEL2) AS telefono2,
        TRIM(t.TRCDIR1) AS direccion1,
        TRIM(t.TRCCIU) AS ciudad,
        TRIM(t.TRCDEPA) AS departamento
      FROM pv.proveedo p
      INNER JOIN adm.trc t ON p.PROCOD = t.TRCID
    "#,
    );

    if !include_inactive {
        query.push_str(" WHERE p.status = 'A'");
    }

    query.push_str(" ORDER BY t.TRCNOM ASC");

    let result = sqlx::query_as::<_, Proveedor>(&query)
        .fetch_all(&db.read_pool)
        .await
        .map_err(|e| {
            log::error!("Error listing suppliers: {:?}", e);
            "Ocurrió un error interno en el servidor.".to_string()
        })?;

    Ok(result)
}

pub async fn perform_get_proveedor(
    db: &AppDb,
    id: String,
) -> Result<Proveedor, String> {
    let trimmed_id = id.trim();
    let query = r#"
      SELECT 
        TRIM(p.PROCOD) AS id,
        TRIM(p.PRONUMDOC) AS num_doc,
        p.PROTIPDOC AS tipo_doc,
        TRIM(p.PROEMA) AS email,
        TRIM(p.PROCON) AS contacto,
        p.status AS status,
        p.pais AS pais,
        TRIM(t.TRCNOM) AS nombre,
        TRIM(t.TRCAPE) AS apellido,
        TRIM(t.TRCTEL1) AS telefono1,
        TRIM(t.TRCTEL2) AS telefono2,
        TRIM(t.TRCDIR1) AS direccion1,
        TRIM(t.TRCCIU) AS ciudad,
        TRIM(t.TRCDEPA) AS departamento
      FROM pv.proveedo p
      INNER JOIN adm.trc t ON p.PROCOD = t.TRCID
      WHERE TRIM(p.PROCOD) = ?
    "#;

    let result = sqlx::query_as::<_, Proveedor>(query)
        .bind(trimmed_id)
        .fetch_optional(&db.read_pool)
        .await
        .map_err(|e| {
            log::error!("Error getting supplier {}: {:?}", trimmed_id, e);
            "Ocurrió un error interno en el servidor.".to_string()
        })?;

    match result {
        Some(prov) => Ok(prov),
        None => Err("Proveedor no encontrado".to_string()),
    }
}

pub async fn perform_create_proveedor(
    db: &AppDb,
    input: CreateProveedorInput,
) -> Result<String, String> {
    let trimmed_doc = input.num_doc.trim();
    if trimmed_doc.is_empty() {
        return Err("El número de documento (numDoc) es requerido.".to_string());
    }
    let trimmed_name = input.nombre.trim();
    if trimmed_name.is_empty() {
        return Err("El nombre o razón social (nombre) es requerido.".to_string());
    }
    let tipo_doc = input.tipo_doc.trim();
    if tipo_doc != "C" && tipo_doc != "N" && tipo_doc != "E" {
        return Err("El tipo de documento (tipoDoc) debe ser C (Cédula), N (NIT) o E (Extranjería).".to_string());
    }

    let empid = std::env::var("DB_EMPID").unwrap_or_else(|_| "000000000000001".to_string());

    let mut tx = db.write_pool.begin().await.map_err(|e| {
        log::error!("Failed to begin transaction: {:?}", e);
        "Ocurrió un error interno en el servidor.".to_string()
    })?;

    sqlx::query("SET SESSION sql_mode = 'STRICT_TRANS_TABLES'")
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            log::error!("Failed to set STRICT_TRANS_TABLES: {:?}", e);
            "Ocurrió un error interno en el servidor.".to_string()
        })?;

    let existing_prov: Option<String> = sqlx::query_scalar(
        "SELECT PROCOD FROM pv.proveedo WHERE TRIM(PRONUMDOC) = ?"
    )
    .bind(trimmed_doc)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| {
        log::error!("Error checking duplicate supplier: {:?}", e);
        "Ocurrió un error interno en el servidor.".to_string()
    })?;

    if existing_prov.is_some() {
        return Err("Ya existe un proveedor con este número de documento.".to_string());
    }

    let existing_trc: Option<String> = sqlx::query_scalar(
        "SELECT TRCID FROM adm.trc WHERE TRIM(TRCNUMDOC) = ?"
    )
    .bind(trimmed_doc)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| {
        log::error!("Error checking duplicate third party: {:?}", e);
        "Ocurrió un error interno en el servidor.".to_string()
    })?;

    let trcid: String;

    let nombre_upper = trimmed_name.to_uppercase();
    let apellido_upper = map_upper_or_empty(&input.apellido);
    let tel1 = map_or_empty(&input.telefono1);
    let tel2 = map_or_empty(&input.telefono2);
    let email_val = map_or_empty(&input.email);
    let dir1_upper = map_upper_or_empty(&input.direccion1);
    let ciu_upper = map_upper_or_empty(&input.ciudad);
    let dep_upper = map_upper_or_empty(&input.departamento);
    let contacto_upper = map_upper_or_empty(&input.contacto);

    if let Some(trc_id) = existing_trc {
        trcid = trc_id.trim().to_string();
        
        sqlx::query(
            r#"
            UPDATE adm.trc SET 
              TRCNOM = ?, TRCAPE = ?, TRCTEL1 = ?, TRCTEL2 = ?, trcema1 = ?, 
              TRCDIR1 = ?, TRCCIU = ?, TRCDEPA = ?, TRCULTMOD = CURDATE()
            WHERE TRCID = ?
            "#
        )
        .bind(&nombre_upper)
        .bind(&apellido_upper)
        .bind(&tel1)
        .bind(&tel2)
        .bind(&email_val)
        .bind(&dir1_upper)
        .bind(&ciu_upper)
        .bind(&dep_upper)
        .bind(&trcid)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            log::error!("Failed to update existing trc {}: {:?}", trcid, e);
            "Ocurrió un error interno en el servidor.".to_string()
        })?;
    } else {
        let prefix = if trimmed_doc.len() >= 5 {
            &trimmed_doc[0..5]
        } else {
            trimmed_doc
        };

        let rows: Vec<String> = sqlx::query_scalar(
            "SELECT PROCOD FROM pv.proveedo WHERE PROCOD LIKE ?"
        )
        .bind(format!("{}%", prefix))
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| {
            log::error!("Failed to fetch sequence for prefix {}: {:?}", prefix, e);
            "Ocurrió un error interno en el servidor.".to_string()
        })?;

        let mut max_seq: u64 = 0;
        for r in rows {
            let code_str = r.trim();
            if code_str.len() >= 10 {
                let seq_part = &code_str[code_str.len() - 10..];
                if let Ok(seq_val) = seq_part.parse::<u64>() {
                    if seq_val > max_seq {
                        max_seq = seq_val;
                    }
                }
            }
        }
        let next_seq = max_seq + 1;
        let seq_str = format!("{:010}", next_seq);
        trcid = format!("{}{}", prefix, seq_str);

        let trc_tip = "PROVEEDOR";
        let trc_nat = if tipo_doc == "N" { "J" } else { "N" };

        sqlx::query(
            r#"
            INSERT INTO adm.trc (
              EMPID, TRCID, TRCNOM, TRCAPE, TRCTEL1, TRCTEL2, trcema1, 
              TRCTIPDOC, TRCNUMDOC, TRCDIR1, TRCCIU, TRCPAI, TRCNAT, 
              TRCDEPA, TRCTIP, TRCULTMOD
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'CO', ?, ?, ?, CURDATE())
            "#
        )
        .bind(&empid)
        .bind(&trcid)
        .bind(&nombre_upper)
        .bind(&apellido_upper)
        .bind(&tel1)
        .bind(&tel2)
        .bind(&email_val)
        .bind(tipo_doc)
        .bind(trimmed_doc)
        .bind(&dir1_upper)
        .bind(&ciu_upper)
        .bind(trc_nat)
        .bind(&dep_upper)
        .bind(trc_tip)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            log::error!("Failed to insert new trc {}: {:?}", trcid, e);
            "Ocurrió un error interno en el servidor.".to_string()
        })?;
    }

    sqlx::query(
        r#"
        INSERT INTO pv.proveedo (
          PROCOD, PROCON, PRONUMDOC, PROTIPDOC, PROEMA, EMPID, status, pais, PROFECMOD
        ) VALUES (?, ?, ?, ?, ?, ?, 'A', 'CO', CURDATE())
        "#
    )
    .bind(&trcid)
    .bind(&contacto_upper)
    .bind(trimmed_doc)
    .bind(tipo_doc)
    .bind(&email_val)
    .bind(&empid)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        log::error!("Failed to insert into proveedo {}: {:?}", trcid, e);
        "Ocurrió un error interno en el servidor.".to_string()
    })?;

    tx.commit().await.map_err(|e| {
        log::error!("Failed to commit transaction: {:?}", e);
        "Ocurrió un error interno en el servidor.".to_string()
    })?;

    Ok(trcid)
}

pub async fn perform_update_proveedor(
    db: &AppDb,
    id: String,
    input: UpdateProveedorInput,
) -> Result<(), String> {
    let trimmed_id = id.trim();
    let trimmed_name = input.nombre.trim();
    if trimmed_name.is_empty() {
        return Err("El nombre o razón social (nombre) es requerido.".to_string());
    }

    let mut tx = db.write_pool.begin().await.map_err(|e| {
        log::error!("Failed to begin transaction: {:?}", e);
        "Ocurrió un error interno en el servidor.".to_string()
    })?;

    sqlx::query("SET SESSION sql_mode = 'STRICT_TRANS_TABLES'")
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            log::error!("Failed to set STRICT_TRANS_TABLES: {:?}", e);
            "Ocurrió un error interno en el servidor.".to_string()
        })?;

    let existing_prov: Option<String> = sqlx::query_scalar(
        "SELECT PROCOD FROM pv.proveedo WHERE TRIM(PROCOD) = ?"
    )
    .bind(trimmed_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| {
        log::error!("Error checking supplier existence: {:?}", e);
        "Ocurrió un error interno en el servidor.".to_string()
    })?;

    if existing_prov.is_none() {
        return Err("Proveedor no encontrado.".to_string());
    }

    let nombre_upper = trimmed_name.to_uppercase();
    let apellido_upper = map_upper_or_empty(&input.apellido);
    let tel1 = map_or_empty(&input.telefono1);
    let tel2 = map_or_empty(&input.telefono2);
    let email_val = map_or_empty(&input.email);
    let dir1_upper = map_upper_or_empty(&input.direccion1);
    let ciu_upper = map_upper_or_empty(&input.ciudad);
    let dep_upper = map_upper_or_empty(&input.departamento);
    let contacto_upper = map_upper_or_empty(&input.contacto);
    let status_val = input.status.as_deref().unwrap_or("A");

    sqlx::query(
        r#"
        UPDATE adm.trc SET 
          TRCNOM = ?, TRCAPE = ?, TRCTEL1 = ?, TRCTEL2 = ?, trcema1 = ?, 
          TRCDIR1 = ?, TRCCIU = ?, TRCDEPA = ?, TRCULTMOD = CURDATE()
        WHERE TRIM(TRCID) = ?
        "#
    )
    .bind(&nombre_upper)
    .bind(&apellido_upper)
    .bind(&tel1)
    .bind(&tel2)
    .bind(&email_val)
    .bind(&dir1_upper)
    .bind(&ciu_upper)
    .bind(&dep_upper)
    .bind(trimmed_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        log::error!("Failed to update trc {}: {:?}", trimmed_id, e);
        "Ocurrió un error interno en el servidor.".to_string()
    })?;

    sqlx::query(
        r#"
        UPDATE pv.proveedo SET 
          PROCON = ?, PROEMA = ?, status = ?, PROFECMOD = CURDATE()
        WHERE TRIM(PROCOD) = ?
        "#
    )
    .bind(&contacto_upper)
    .bind(&email_val)
    .bind(status_val)
    .bind(trimmed_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        log::error!("Failed to update proveedo {}: {:?}", trimmed_id, e);
        "Ocurrió un error interno en el servidor.".to_string()
    })?;

    tx.commit().await.map_err(|e| {
        log::error!("Failed to commit transaction: {:?}", e);
        "Ocurrió un error interno en el servidor.".to_string()
    })?;

    Ok(())
}

pub async fn perform_delete_proveedor(
    db: &AppDb,
    id: String,
) -> Result<DeleteResult, String> {
    let trimmed_id = id.trim();

    let existing: Option<String> = sqlx::query_scalar(
        "SELECT PROCOD FROM pv.proveedo WHERE TRIM(PROCOD) = ?"
    )
    .bind(trimmed_id)
    .fetch_optional(&db.read_pool)
    .await
    .map_err(|e| {
        log::error!("Failed to verify supplier {}: {:?}", trimmed_id, e);
        "Ocurrió un error interno en el servidor.".to_string()
    })?;

    if existing.is_none() {
        return Err("Proveedor no encontrado.".to_string());
    }

    let table_rows: Vec<String> = sqlx::query_scalar(
        r#"
        SELECT TABLE_NAME 
        FROM information_schema.TABLES 
        WHERE TABLE_SCHEMA = 'pv' AND TABLE_NAME LIKE 'compra%'
        "#
    )
    .fetch_all(&db.read_pool)
    .await
    .map_err(|e| {
        log::error!("Failed to fetch table list: {:?}", e);
        "Ocurrió un error interno en el servidor.".to_string()
    })?;

    let mut has_purchases = false;
    if !table_rows.is_empty() {
        let union_queries: Vec<String> = table_rows
            .iter()
            .map(|t| format!("(SELECT COMNUM FROM pv.{} WHERE TRIM(COMPRO) = ? LIMIT 1)", t))
            .collect();
        let union_query = union_queries.join(" UNION ALL ");
        let final_query = format!("SELECT 1 AS has_purchases FROM ({}) AS tmp LIMIT 1", union_query);

        let mut query = sqlx::query(&final_query);
        for _ in 0..table_rows.len() {
            query = query.bind(trimmed_id);
        }

        let result_rows = query.fetch_all(&db.read_pool).await.map_err(|e| {
            log::error!("Failed to check purchase history: {:?}", e);
            "Ocurrió un error interno en el servidor.".to_string()
        })?;

        has_purchases = !result_rows.is_empty();
    }

    let mut tx = db.write_pool.begin().await.map_err(|e| {
        log::error!("Failed to begin transaction for delete: {:?}", e);
        "Ocurrió un error interno en el servidor.".to_string()
    })?;

    let action: String;
    let reason: String;
    let message: String;

    if has_purchases {
        sqlx::query("UPDATE pv.proveedo SET status = 'I', PROFECMOD = CURDATE() WHERE TRIM(PROCOD) = ?")
            .bind(trimmed_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                log::error!("Failed to deactivate supplier {}: {:?}", trimmed_id, e);
                "Ocurrió un error interno en el servidor.".to_string()
            })?;
        action = "deactivated".to_string();
        reason = "El proveedor tiene transacciones históricas de compras. Se desactivó el registro para conservar integridad.".to_string();
        message = "Proveedor desactivado (soft-delete) por historial de transacciones.".to_string();
    } else {
        sqlx::query("DELETE FROM pv.proveedo WHERE TRIM(PROCOD) = ?")
            .bind(trimmed_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                log::error!("Failed to physically delete supplier {}: {:?}", trimmed_id, e);
                "Ocurrió un error interno en el servidor.".to_string()
            })?;

        sqlx::query("DELETE FROM adm.trc WHERE TRIM(TRCID) = ? AND TRCTIP = 'PROVEEDOR'")
            .bind(trimmed_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                log::error!("Failed to delete third party record {}: {:?}", trimmed_id, e);
                "Ocurrió un error interno en el servidor.".to_string()
            })?;
        action = "deleted".to_string();
        reason = "El proveedor no tiene transacciones registradas. Se eliminó físicamente de la base de datos.".to_string();
        message = "Proveedor eliminado físicamente.".to_string();
    }

    tx.commit().await.map_err(|e| {
        log::error!("Failed to commit delete transaction: {:?}", e);
        "Ocurrió un error interno en el servidor.".to_string()
    })?;

    Ok(DeleteResult {
        success: true,
        action,
        reason,
        message,
    })
}

// Tauri Command Wrappers
#[tauri::command]
pub async fn list_proveedores(
    db: State<'_, AppDb>,
    include_inactive: Option<bool>,
) -> Result<Vec<Proveedor>, String> {
    perform_list_proveedores(&db, include_inactive).await
}

#[tauri::command]
pub async fn get_proveedor(
    db: State<'_, AppDb>,
    id: String,
) -> Result<Proveedor, String> {
    perform_get_proveedor(&db, id).await
}

#[tauri::command]
pub async fn create_proveedor(
    db: State<'_, AppDb>,
    input: CreateProveedorInput,
) -> Result<String, String> {
    perform_create_proveedor(&db, input).await
}

#[tauri::command]
pub async fn update_proveedor(
    db: State<'_, AppDb>,
    id: String,
    input: UpdateProveedorInput,
) -> Result<(), String> {
    perform_update_proveedor(&db, id, input).await
}

#[tauri::command]
pub async fn delete_proveedor(
    db: State<'_, AppDb>,
    id: String,
) -> Result<DeleteResult, String> {
    perform_delete_proveedor(&db, id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[tokio::test]
    async fn test_list_and_get_proveedores() {
        let db_state = db::create_pools().await.expect("Failed to initialize pools");

        // List active suppliers
        let list = perform_list_proveedores(&db_state, Some(false)).await.unwrap();
        assert!(!list.is_empty(), "Supplier list should not be empty");

        // The active list should only contain status 'A'
        for p in &list {
            assert_eq!(p.status, "A");
        }

        // Get existing supplier details
        let prov_id = "900120000000001".to_string();
        let prov = perform_get_proveedor(&db_state, prov_id).await.unwrap();
        assert_eq!(prov.nombre, "PROVEEDOR CON COMPRAS S.A.S.");
        assert_eq!(prov.num_doc, "900123456");

        // Attempt to get non-existent supplier
        let err = perform_get_proveedor(&db_state, "NON_EXISTENT".to_string()).await.unwrap_err();
        assert_eq!(err, "Proveedor no encontrado");
    }

    #[tokio::test]
    async fn test_crud_proveedor_lifecycle() {
        let db_state = db::create_pools().await.expect("Failed to initialize pools");

        // Generate unique doc number to avoid collision
        let rand_num: u64 = chrono::Utc::now().timestamp_millis() as u64 % 100000;
        let test_doc = format!("9999{}", rand_num);

        let input = CreateProveedorInput {
            num_doc: test_doc.clone(),
            tipo_doc: "N".to_string(),
            nombre: "PROVEEDOR TEST DE INTEGRACION".to_string(),
            apellido: Some("LTDA".to_string()),
            telefono1: Some("333-1111".to_string()),
            telefono2: None,
            email: Some("test_integration@recicladora.com".to_string()),
            contacto: Some("TEST CONTACTO".to_string()),
            direccion1: Some("Calle Falsa 123".to_string()),
            ciudad: Some("Tunja".to_string()),
            departamento: Some("Boyacá".to_string()),
        };

        // 1. Create supplier
        let created_id = perform_create_proveedor(&db_state, input).await.unwrap();
        assert!(!created_id.is_empty(), "Returned ID should not be empty");
        let expected_prefix = &test_doc[0..5];
        assert!(created_id.starts_with(expected_prefix), "ID does not start with expected prefix");

        // 2. Fetch and verify
        let fetched = perform_get_proveedor(&db_state, created_id.clone()).await.unwrap();
        assert_eq!(fetched.nombre, "PROVEEDOR TEST DE INTEGRACION");
        assert_eq!(fetched.apellido.as_deref(), Some("LTDA"));
        assert_eq!(fetched.email.as_deref(), Some("test_integration@recicladora.com"));
        assert_eq!(fetched.contacto.as_deref(), Some("TEST CONTACTO"));

        // 3. Duplicate check validation
        let duplicate_input = CreateProveedorInput {
            num_doc: test_doc.clone(),
            tipo_doc: "N".to_string(),
            nombre: "OTRO NOMBRE".to_string(),
            apellido: None,
            telefono1: None,
            telefono2: None,
            email: None,
            contacto: None,
            direccion1: None,
            ciudad: None,
            departamento: None,
        };
        let dup_err = perform_create_proveedor(&db_state, duplicate_input).await.unwrap_err();
        assert_eq!(dup_err, "Ya existe un proveedor con este número de documento.");

        // 4. Update fields
        let update_input = UpdateProveedorInput {
            nombre: "PROVEEDOR TEST MODIFICADO".to_string(),
            apellido: Some("S.A.".to_string()),
            telefono1: Some("444-2222".to_string()),
            telefono2: Some("555-3333".to_string()),
            email: Some("mod@recicladora.com".to_string()),
            contacto: Some("CONTACTO MOD".to_string()),
            direccion1: Some("Diagonal 45 # 10-20".to_string()),
            ciudad: Some("Duitama".to_string()),
            departamento: Some("Boyacá".to_string()),
            status: Some("A".to_string()),
        };
        perform_update_proveedor(&db_state, created_id.clone(), update_input).await.unwrap();

        // Verify update
        let fetched_updated = perform_get_proveedor(&db_state, created_id.clone()).await.unwrap();
        assert_eq!(fetched_updated.nombre, "PROVEEDOR TEST MODIFICADO");
        assert_eq!(fetched_updated.apellido.as_deref(), Some("S.A."));
        assert_eq!(fetched_updated.telefono1.as_deref(), Some("444-2222"));
        assert_eq!(fetched_updated.telefono2.as_deref(), Some("555-3333"));
        assert_eq!(fetched_updated.email.as_deref(), Some("mod@recicladora.com"));
        assert_eq!(fetched_updated.contacto.as_deref(), Some("CONTACTO MOD"));
        assert_eq!(fetched_updated.direccion1.as_deref(), Some("DIAGONAL 45 # 10-20"));
        assert_eq!(fetched_updated.ciudad.as_deref(), Some("DUITAMA"));

        // 5. Delete (hard delete because no purchases)
        let del_res = perform_delete_proveedor(&db_state, created_id.clone()).await.unwrap();
        assert!(del_res.success);
        assert_eq!(del_res.action, "deleted");
        assert!(del_res.reason.contains("Se eliminó físicamente"));

        // Verify it no longer exists
        let fetch_err = perform_get_proveedor(&db_state, created_id.clone()).await.unwrap_err();
        assert_eq!(fetch_err, "Proveedor no encontrado");
    }

    #[tokio::test]
    async fn test_secure_delete_soft_delete() {
        let db_state = db::create_pools().await.expect("Failed to initialize pools");

        let prov_id = "900120000000001".to_string();

        let before_del = perform_get_proveedor(&db_state, prov_id.clone()).await.unwrap();
        assert_eq!(before_del.status, "A");

        let del_res = perform_delete_proveedor(&db_state, prov_id.clone()).await.unwrap();
        assert!(del_res.success);
        assert_eq!(del_res.action, "deactivated");
        assert!(del_res.reason.contains("desactivó el registro"));

        let after_del = perform_get_proveedor(&db_state, prov_id.clone()).await.unwrap();
        assert_eq!(after_del.status, "I");

        let restore_input = UpdateProveedorInput {
            nombre: after_del.nombre,
            apellido: after_del.apellido,
            telefono1: after_del.telefono1,
            telefono2: after_del.telefono2,
            email: after_del.email,
            contacto: after_del.contacto,
            direccion1: after_del.direccion1,
            ciudad: after_del.ciudad,
            departamento: after_del.departamento,
            status: Some("A".to_string()),
        };
        perform_update_proveedor(&db_state, prov_id.clone(), restore_input).await.unwrap();
    }
}
