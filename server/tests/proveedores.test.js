// Las variables de entorno se cargan desde .env.test a través del script `npm test`.
// Ver package.json: node -e "require('dotenv').config({path:'.env.test'})"


const request = require('supertest');
const app = require('../index');
const { readPool, writePool } = require('../config/database');

describe('Proveedores API Integration Tests', () => {
  
  // Cerrar pools después de todas las pruebas para liberar manejadores abiertos
  afterAll(async () => {
    await readPool.end();
    await writePool.end();
  });

  describe('GET /api/proveedores', () => {
    test('should return all active suppliers by default', async () => {
      const res = await request(app)
        .get('/api/proveedores')
        .expect(200);

      expect(res.body.success).toBe(true);
      expect(Array.isArray(res.body.data)).toBe(true);
      // El proveedor inactivo (123450000000003) no debería estar en el listado por defecto
      const inactive = res.body.data.find(p => p.id === '123450000000003');
      expect(inactive).toBeUndefined();
    });

    test('should return inactive suppliers if includeInactive=true is passed', async () => {
      const res = await request(app)
        .get('/api/proveedores?includeInactive=true')
        .expect(200);

      expect(res.body.success).toBe(true);
      const inactive = res.body.data.find(p => p.id.trim() === '123450000000003');
      expect(inactive).toBeDefined();
      expect(inactive.status).toBe('I');
    });
  });

  describe('GET /api/proveedores/:id', () => {
    test('should return details of a specific supplier', async () => {
      const res = await request(app)
        .get('/api/proveedores/900120000000001')
        .expect(200);

      expect(res.body.success).toBe(true);
      expect(res.body.data.id).toBe('900120000000001');
      expect(res.body.data.nombre).toBe('PROVEEDOR CON COMPRAS S.A.S.');
    });

    test('should return 404 if supplier does not exist', async () => {
      const res = await request(app)
        .get('/api/proveedores/nonexistentid')
        .expect(404);

      expect(res.body.success).toBe(false);
    });
  });

  describe('POST /api/proveedores', () => {
    test('should create a new supplier and calculate consecutive PROCOD code', async () => {
      const uniqueDoc = '950' + Math.floor(100000 + Math.random() * 900000);
      const newSupplier = {
        numDoc: uniqueDoc,
        tipoDoc: 'N',
        nombre: 'NUEVO PROVEEDOR TEST S.A.S.',
        email: 'test@recicladora.com',
        contacto: 'JEFE COMPRAS TEST',
        direccion1: 'Calle 100 # 50-20',
        ciudad: 'Tunja',
        departamento: 'Boyacá'
      };

      const res = await request(app)
        .post('/api/proveedores')
        .send(newSupplier)
        .expect(201);

      expect(res.body.success).toBe(true);
      expect(res.body.id).toBeDefined();
      // El ID debe comenzar con los primeros 5 dígitos del documento
      expect(res.body.id.startsWith(uniqueDoc.substring(0, 5))).toBe(true);

      // Verificar que fue creado en la base de datos
      const [rows] = await readPool.query(
        "SELECT * FROM pv.proveedo WHERE PROCOD = ?",
        [res.body.id]
      );
      expect(rows.length).toBe(1);
      expect(rows[0].PRONUMDOC).toBe(uniqueDoc);
    });

    test('should rollback if creation fails half-way', async () => {
      const uniqueDoc = '980' + Math.floor(100000 + Math.random() * 900000);
      
      // Intentamos insertar con un tipoDoc inválido de base de datos pero pasando las validaciones de Express 
      // (Para simular una falla en la BD durante la transacción, forzando un error al insertar en pv.proveedo).
      // En este caso, podemos simularlo enviando un body que pase la validación del backend pero cause un error en la BD.
      // Sin embargo, proveedores.js valida: if (!tipoDoc || !isValidTipDoc(tipoDoc)) { throw AppError }
      // ¿Cómo forzamos un error en la BD?
      // Podemos enviar un PROEMA extremadamente largo que exceda el tamaño de la columna (50 chars) en pv.proveedo
      // pero no en adm.trc (trcema1 es de 80 chars).
      // trcema1 recibirá un email de 75 chars (cabe en 80), pero pv.proveedo.PROEMA de 50 chars fallará!
      const longEmail = 'a'.repeat(70) + '@test.com'; // 79 caracteres

      const failSupplier = {
        numDoc: uniqueDoc,
        tipoDoc: 'C',
        nombre: 'PROVEEDOR FALLIDO S.A.S.',
        email: longEmail, // causará Data too long for column 'PROEMA' en proveedo
        direccion1: 'Calle Ficticia',
        ciudad: 'Tunja',
        departamento: 'Boyacá'
      };

      const res = await request(app)
        .post('/api/proveedores')
        .send(failSupplier)
        .expect(500); // Falla del servidor controlada por el handler

      // Verificar que el registro NO quedó guardado en adm.trc (Rollback exitoso)
      const [rows] = await readPool.query(
        "SELECT * FROM adm.trc WHERE TRCNUMDOC = ?",
        [uniqueDoc]
      );
      expect(rows.length).toBe(0);
    });
  });

  describe('PUT /api/proveedores/:id', () => {
    test('should update contact details successfully', async () => {
      const updateData = {
        nombre: 'PROVEEDOR CON COMPRAS MODIFICADO S.A.S.',
        contacto: 'NUEVO CONTACTO CARLOS',
        email: 'nuevoemail@proveedorA.com',
        direccion1: 'Calle Modificada 123',
        ciudad: 'Tunja',
        departamento: 'Boyacá'
      };

      const res = await request(app)
        .put('/api/proveedores/900120000000001')
        .send(updateData)
        .expect(200);

      expect(res.body.success).toBe(true);

      // Validar cambios en adm.trc
      const [trcRows] = await readPool.query(
        "SELECT TRCNOM, TRCDIR1 FROM adm.trc WHERE TRIM(TRCID) = '900120000000001'"
      );
      expect(trcRows[0].TRCNOM).toBe('PROVEEDOR CON COMPRAS MODIFICADO S.A.S.');
      expect(trcRows[0].TRCDIR1).toBe('CALLE MODIFICADA 123');
    });
  });

  describe('DELETE /api/proveedores/:id (Borrado Seguro)', () => {
    test('should perform soft-delete (status = I) if supplier has purchases', async () => {
      const res = await request(app)
        .delete('/api/proveedores/900120000000001')
        .expect(200);

      expect(res.body.success).toBe(true);
      expect(res.body.action).toBe('deactivated');
      expect(res.body.reason).toContain('transacciones históricas');

      // Verificar que sigue existiendo pero su estado es 'I'
      const [rows] = await readPool.query(
        "SELECT status FROM pv.proveedo WHERE TRIM(PROCOD) = '900120000000001'"
      );
      expect(rows[0].status).toBe('I');
    });

    test('should perform hard-delete if supplier has no purchases', async () => {
      const res = await request(app)
        .delete('/api/proveedores/900980000000002')
        .expect(200);

      expect(res.body.success).toBe(true);
      expect(res.body.action).toBe('deleted');

      // Verificar que ya no existe en pv.proveedo
      const [provRows] = await readPool.query(
        "SELECT PROCOD FROM pv.proveedo WHERE TRIM(PROCOD) = '900980000000002'"
      );
      expect(provRows.length).toBe(0);

      // Verificar que tampoco existe en adm.trc
      const [trcRows] = await readPool.query(
        "SELECT TRCID FROM adm.trc WHERE TRIM(TRCID) = '900980000000002'"
      );
      expect(trcRows.length).toBe(0);
    });
  });

  describe('Separación de Privilegios en la Base de Datos', () => {
    test('should fail when attempting to write using the readPool', async () => {
      // El readPool tiene un usuario con permisos SELECT únicamente.
      // Intentar insertar debería generar un error de privilegios en el motor SQL.
      await expect(
        readPool.query(
          "INSERT INTO pv.proveedo (PROCOD, PRONUMDOC, PROTIPDOC) VALUES ('fail_code', '1111', 'C')"
        )
      ).rejects.toThrow(/INSERT command denied|Access denied|SELECT command denied|read-only/i);
    });
  });
});
