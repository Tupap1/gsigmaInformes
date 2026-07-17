const express = require('express');
const router = express.Router();
const { readPool, writePool } = require('../config/database');
const { AppError } = require('../middleware/errorHandler');

/**
 * Helper para validar el tipo de documento.
 */
function isValidTipDoc(tip) {
  return ['C', 'N', 'E'].includes(tip);
}

/**
 * GET /api/proveedores
 * Obtiene el listado de proveedores.
 * Por defecto solo activos, a menos que se pase includeInactive=true.
 */
router.get('/', async (req, res, next) => {
  try {
    const includeInactive = req.query.includeInactive === 'true';
    let query = `
      SELECT 
        TRIM(p.PROCOD) AS id,
        TRIM(p.PRONUMDOC) AS numDoc,
        p.PROTIPDOC AS tipoDoc,
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
    `;

    if (!includeInactive) {
      query += " WHERE p.status = 'A'";
    }

    query += " ORDER BY t.TRCNOM ASC";

    const [rows] = await readPool.query(query);
    res.json({
      success: true,
      data: rows
    });
  } catch (error) {
    next(error);
  }
});

/**
 * GET /api/proveedores/:id
 * Obtiene el detalle de un proveedor por su PROCOD (id).
 */
router.get('/:id', async (req, res, next) => {
  try {
    const { id } = req.params;
    const query = `
      SELECT 
        TRIM(p.PROCOD) AS id,
        TRIM(p.PRONUMDOC) AS numDoc,
        p.PROTIPDOC AS tipoDoc,
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
    `;

    const [rows] = await readPool.query(query, [id]);

    if (rows.length === 0) {
      throw new AppError('Proveedor no encontrado', 404);
    }

    res.json({
      success: true,
      data: rows[0]
    });
  } catch (error) {
    next(error);
  }
});

/**
 * POST /api/proveedores
 * Crea un nuevo proveedor.
 * Lógica transaccional que asegura inserciones consistentes en adm.trc y pv.proveedo.
 */
router.post('/', async (req, res, next) => {
  let connection;
  try {
    const {
      numDoc,
      tipoDoc,
      nombre,
      apellido = '',
      telefono1 = '',
      telefono2 = '',
      email = '',
      contacto = '',
      direccion1 = '',
      ciudad = '',
      departamento = ''
    } = req.body;

    // 1. Validaciones de servidor
    if (!numDoc || !numDoc.trim()) {
      throw new AppError('El número de documento (numDoc) es requerido.');
    }
    if (!nombre || !nombre.trim()) {
      throw new AppError('El nombre o razón social (nombre) es requerido.');
    }
    if (!tipoDoc || !isValidTipDoc(tipoDoc)) {
      throw new AppError('El tipo de documento (tipoDoc) debe ser C (Cédula), N (NIT) o E (Extranjería).');
    }

    const trimmedDoc = numDoc.trim();
    const empid = process.env.DB_EMPID || '000000000000001';

    // Obtener conexión para la transacción
    connection = await writePool.getConnection();
    await connection.query("SET SESSION sql_mode = 'STRICT_TRANS_TABLES'");
    await connection.beginTransaction();

    // 2. Verificar duplicado en pv.proveedo
    const [existingProv] = await connection.query(
      "SELECT PROCOD FROM pv.proveedo WHERE TRIM(PRONUMDOC) = ?",
      [trimmedDoc]
    );
    if (existingProv.length > 0) {
      throw new AppError('Ya existe un proveedor con este número de documento.', 400);
    }

    // 3. Verificar si el tercero ya existe en adm.trc
    const [existingTrc] = await connection.query(
      "SELECT TRCID FROM adm.trc WHERE TRIM(TRCNUMDOC) = ?",
      [trimmedDoc]
    );

    let trcid;

    if (existingTrc.length > 0) {
      // Si ya existe en trc, reutilizamos el TRCID y actualizamos sus datos de contacto
      trcid = existingTrc[0].TRCID.trim();
      await connection.query(`
        UPDATE adm.trc SET 
          TRCNOM = ?, TRCAPE = ?, TRCTEL1 = ?, TRCTEL2 = ?, trcema1 = ?, 
          TRCDIR1 = ?, TRCCIU = ?, TRCDEPA = ?, TRCULTMOD = CURDATE()
        WHERE TRCID = ?
      `, [
        nombre.toUpperCase(),
        apellido.toUpperCase(),
        telefono1,
        telefono2,
        email,
        direccion1.toUpperCase(),
        ciudad.toUpperCase(),
        departamento.toUpperCase(),
        trcid
      ]);
    } else {
      // Si no existe, generamos un código de proveedor nuevo en base al prefijo
      const prefix = trimmedDoc.substring(0, 5);
      
      // Consultamos todos los códigos de proveedor con el mismo prefijo para calcular el consecutivo
      const [rows] = await connection.query(
        "SELECT PROCOD FROM pv.proveedo WHERE PROCOD LIKE ?",
        [prefix + '%']
      );

      let maxSeq = 0;
      for (const r of rows) {
        const codeStr = r.PROCOD.trim();
        if (codeStr.length >= 10) {
          const seqPart = codeStr.slice(-10);
          const seqVal = parseInt(seqPart, 10);
          if (!isNaN(seqVal) && seqVal > maxSeq) {
            maxSeq = seqVal;
          }
        }
      }
      const nextSeq = maxSeq + 1;
      const seqStr = String(nextSeq).padStart(10, '0');
      trcid = prefix + seqStr;

      // Inserción en adm.trc (Maestro de terceros)
      const trcTip = 'PROVEEDOR';
      const trcNat = tipoDoc === 'N' ? 'J' : 'N'; // J=Jurídica, N=Natural
      await connection.query(`
        INSERT INTO adm.trc (
          EMPID, TRCID, TRCNOM, TRCAPE, TRCTEL1, TRCTEL2, trcema1, 
          TRCTIPDOC, TRCNUMDOC, TRCDIR1, TRCCIU, TRCPAI, TRCNAT, 
          TRCDEPA, TRCTIP, TRCULTMOD
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'CO', ?, ?, ?, CURDATE())
      `, [
        empid,
        trcid,
        nombre.toUpperCase(),
        apellido.toUpperCase(),
        telefono1,
        telefono2,
        email,
        tipoDoc,
        trimmedDoc,
        direccion1.toUpperCase(),
        ciudad.toUpperCase(),
        trcNat,
        departamento.toUpperCase(),
        trcTip
      ]);
    }

    // Inserción en pv.proveedo
    await connection.query(`
      INSERT INTO pv.proveedo (
        PROCOD, PROCON, PRONUMDOC, PROTIPDOC, PROEMA, EMPID, status, pais, PROFECMOD
      ) VALUES (?, ?, ?, ?, ?, ?, 'A', 'CO', CURDATE())
    `, [
      trcid,
      contacto.toUpperCase(),
      trimmedDoc,
      tipoDoc,
      email,
      empid
    ]);

    await connection.commit();
    res.status(201).json({
      success: true,
      message: 'Proveedor creado correctamente.',
      id: trcid
    });

  } catch (error) {
    if (connection) {
      await connection.rollback();
    }
    next(error);
  } finally {
    if (connection) {
      connection.release();
    }
  }
});

/**
 * PUT /api/proveedores/:id
 * Actualiza los datos de un proveedor existente.
 * Actualiza la información en adm.trc y pv.proveedo de forma transaccional.
 */
router.put('/:id', async (req, res, next) => {
  let connection;
  try {
    const { id } = req.params;
    const {
      nombre,
      apellido = '',
      telefono1 = '',
      telefono2 = '',
      email = '',
      contacto = '',
      direccion1 = '',
      ciudad = '',
      departamento = '',
      status = 'A'
    } = req.body;

    // Validar nombre
    if (!nombre || !nombre.trim()) {
      throw new AppError('El nombre o razón social (nombre) es requerido.');
    }

    connection = await writePool.getConnection();
    await connection.beginTransaction();

    // Verificar si el proveedor existe en pv.proveedo
    const [existing] = await connection.query(
      "SELECT PROCOD FROM pv.proveedo WHERE TRIM(PROCOD) = ?",
      [id.trim()]
    );
    if (existing.length === 0) {
      throw new AppError('Proveedor no encontrado.', 404);
    }

    const trcid = id.trim();

    // Actualizar adm.trc
    await connection.query(`
      UPDATE adm.trc SET 
        TRCNOM = ?, TRCAPE = ?, TRCTEL1 = ?, TRCTEL2 = ?, trcema1 = ?, 
        TRCDIR1 = ?, TRCCIU = ?, TRCDEPA = ?, TRCULTMOD = CURDATE()
      WHERE TRIM(TRCID) = ?
    `, [
      nombre.toUpperCase(),
      apellido.toUpperCase(),
      telefono1,
      telefono2,
      email,
      direccion1.toUpperCase(),
      ciudad.toUpperCase(),
      departamento.toUpperCase(),
      trcid
    ]);

    // Actualizar pv.proveedo
    await connection.query(`
      UPDATE pv.proveedo SET 
        PROCON = ?, PROEMA = ?, status = ?, PROFECMOD = CURDATE()
      WHERE TRIM(PROCOD) = ?
    `, [
      contacto.toUpperCase(),
      email,
      status,
      trcid
    ]);

    await connection.commit();
    res.json({
      success: true,
      message: 'Proveedor actualizado correctamente.'
    });

  } catch (error) {
    if (connection) {
      await connection.rollback();
    }
    next(error);
  } finally {
    if (connection) {
      connection.release();
    }
  }
});

/**
 * DELETE /api/proveedores/:id
 * Lógica de Borrado Seguro:
 * 1. Consultar tablas de compras históricas para ver si tiene compras.
 * 2. Si no tiene compras: Hard-delete en pv.proveedo y adm.trc.
 * 3. Si tiene compras: Soft-delete actualizando status = 'I' en pv.proveedo.
 */
router.delete('/:id', async (req, res, next) => {
  let connection;
  try {
    const { id } = req.params;
    const trcid = id.trim();

    // 1. Verificar si existe en pv.proveedo
    const [existing] = await readPool.query(
      "SELECT PROCOD, PRONUMDOC FROM pv.proveedo WHERE TRIM(PROCOD) = ?",
      [trcid]
    );
    if (existing.length === 0) {
      throw new AppError('Proveedor no encontrado.', 404);
    }

    // 2. Obtener tablas de compras en el esquema pv
    const [tableRows] = await readPool.query(`
      SELECT TABLE_NAME 
      FROM information_schema.TABLES 
      WHERE TABLE_SCHEMA = 'pv' AND TABLE_NAME LIKE 'compra%'
    `);
    const existingTables = tableRows.map(r => r.TABLE_NAME);

    let hasPurchases = false;
    if (existingTables.length > 0) {
      const queries = existingTables.map(t => `(SELECT COMNUM FROM pv.${t} WHERE TRIM(COMPRO) = ? LIMIT 1)`);
      const unionQuery = queries.join(' UNION ALL ');
      const finalQuery = `SELECT 1 AS has_purchases FROM (${unionQuery}) AS tmp LIMIT 1`;
      
      const [result] = await readPool.query(finalQuery, Array(existingTables.length).fill(trcid));
      hasPurchases = result.length > 0;
    }

    connection = await writePool.getConnection();
    await connection.beginTransaction();

    let action;
    let reason;

    if (hasPurchases) {
      // Soft delete: actualizar status = 'I'
      await connection.query(
        "UPDATE pv.proveedo SET status = 'I', PROFECMOD = CURDATE() WHERE TRIM(PROCOD) = ?",
        [trcid]
      );
      action = 'deactivated';
      reason = 'El proveedor tiene transacciones históricas de compras. Se desactivó el registro para conservar integridad.';
    } else {
      // Hard delete: eliminar de pv.proveedo
      await connection.query(
        "DELETE FROM pv.proveedo WHERE TRIM(PROCOD) = ?",
        [trcid]
      );
      // Eliminar de adm.trc si no está referenciado como cliente u otro rol (por seguridad, solo lo eliminamos de trc si su TRCTIP es 'PROVEEDOR')
      await connection.query(
        "DELETE FROM adm.trc WHERE TRIM(TRCID) = ? AND TRCTIP = 'PROVEEDOR'",
        [trcid]
      );
      action = 'deleted';
      reason = 'El proveedor no tiene transacciones registradas. Se eliminó físicamente de la base de datos.';
    }

    await connection.commit();
    res.json({
      success: true,
      action,
      reason,
      message: action === 'deleted' 
        ? 'Proveedor eliminado físicamente.' 
        : 'Proveedor desactivado (soft-delete) por historial de transacciones.'
    });

  } catch (error) {
    if (connection) {
      await connection.rollback();
    }
    next(error);
  } finally {
    if (connection) {
      connection.release();
    }
  }
});

module.exports = router;
