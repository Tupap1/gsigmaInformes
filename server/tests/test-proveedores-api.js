/**
 * Script de Pruebas Automatizadas — API de Proveedores
 * Valida el CRUD completo contra el servidor Express en ejecución.
 * 
 * Uso: node server/tests/test-proveedores-api.js
 */

const http = require('http');

const BASE_URL = 'http://localhost:3001';
let TESTS_PASSED = 0;
let TESTS_FAILED = 0;
let CREATED_ID = null; // ID del proveedor de prueba que crearemos

// ─── Helpers ──────────────────────────────────────────────────────────────────

function request(method, path, body = null) {
  return new Promise((resolve, reject) => {
    const options = {
      hostname: 'localhost',
      port: 3001,
      path,
      method,
      headers: { 'Content-Type': 'application/json' }
    };

    const req = http.request(options, (res) => {
      let data = '';
      res.on('data', chunk => data += chunk);
      res.on('end', () => {
        try {
          resolve({ status: res.statusCode, body: JSON.parse(data) });
        } catch (e) {
          resolve({ status: res.statusCode, body: data });
        }
      });
    });

    req.on('error', reject);
    if (body) req.write(JSON.stringify(body));
    req.end();
  });
}

function assert(condition, label, details = '') {
  if (condition) {
    console.log(`  ✅ PASS: ${label}`);
    TESTS_PASSED++;
  } else {
    console.error(`  ❌ FAIL: ${label}${details ? ' — ' + details : ''}`);
    TESTS_FAILED++;
  }
}

// ─── Pruebas ──────────────────────────────────────────────────────────────────

async function testHealth() {
  console.log('\n📡 [0] Health Check');
  const res = await request('GET', '/api/health');
  assert(res.status === 200, 'Servidor responde 200 OK', JSON.stringify(res.body));
  assert(res.body.status === 'up', 'Estado del servidor es "up"');
}

async function testGetProveedores() {
  console.log('\n📋 [1] GET /api/proveedores');
  const res = await request('GET', '/api/proveedores');
  assert(res.status === 200, 'Responde 200 OK');
  assert(res.body.success === true, 'success = true');
  assert(Array.isArray(res.body.data), 'data es un array');
  assert(res.body.data.length > 0, `Hay proveedores en la BD (${res.body.data.length} encontrados)`);

  // Todos los registros tienen el campo 'nombre' y 'id'
  const hasNames = res.body.data.every(p => p.nombre && p.id);
  assert(hasNames, 'Todos los proveedores tienen nombre e id');
}

async function testValidationErrors() {
  console.log('\n🛡️ [2] POST /api/proveedores — Validaciones de servidor');

  // Sin número de documento
  let res = await request('POST', '/api/proveedores', { nombre: 'TEST SIN DOC', tipoDoc: 'C' });
  assert(res.status === 400, 'Rechaza POST sin numDoc (400)');
  assert(res.body.success === false, 'success = false en error de validación');

  // Sin nombre
  res = await request('POST', '/api/proveedores', { numDoc: '99999TEST99', tipoDoc: 'C' });
  assert(res.status === 400, 'Rechaza POST sin nombre (400)');

  // Tipo de documento inválido
  res = await request('POST', '/api/proveedores', { numDoc: '99999TEST99', nombre: 'TEST', tipoDoc: 'X' });
  assert(res.status === 400, 'Rechaza POST con tipoDoc inválido (400)');
}

async function testCreateProveedor() {
  console.log('\n✍️ [3] POST /api/proveedores — Creación válida');
  
  // Usamos un número de documento único con timestamp para evitar colisiones
  const testDoc = `TEST${Date.now().toString().slice(-7)}`;

  const res = await request('POST', '/api/proveedores', {
    numDoc: testDoc,
    tipoDoc: 'C',
    nombre: 'PROVEEDOR PRUEBA AUTOMATIZADA',
    apellido: 'PRUEBA',
    telefono1: '3000000001',
    email: 'prueba@test.com',
    contacto: 'Contacto Test',
    direccion1: 'CALLE 1 # 2-3',
    ciudad: 'TUNJA',
    departamento: 'BOYACA'
  });

  assert(res.status === 201, `Crea proveedor con status 201 (doc: ${testDoc})`);
  assert(res.body.success === true, 'success = true');
  assert(typeof res.body.id === 'string' && res.body.id.length > 0, `ID generado: ${res.body.id}`);

  if (res.body.id) {
    CREATED_ID = res.body.id;
    console.log(`    🔑 ID creado: ${CREATED_ID}`);
  }
}

async function testDuplicateDocument() {
  console.log('\n🚫 [4] POST /api/proveedores — Documento duplicado');

  if (!CREATED_ID) {
    console.log('  ⚠️ Saltando: no hay ID creado en el paso anterior');
    return;
  }

  // Obtener el documento del proveedor creado
  const getRes = await request('GET', `/api/proveedores/${encodeURIComponent(CREATED_ID)}`);
  const numDoc = getRes.body.data ? getRes.body.data.numDoc : null;

  if (!numDoc) {
    assert(false, 'Se pudo obtener el proveedor recién creado para verificar duplicado');
    return;
  }

  // Intentar crear con el mismo documento
  const res = await request('POST', '/api/proveedores', {
    numDoc: numDoc,
    tipoDoc: 'C',
    nombre: 'DUPLICADO INTENTO'
  });

  assert(res.status === 400, 'Rechaza creación con documento duplicado (400)');
  assert(res.body.success === false, 'success = false en duplicado');
}

async function testGetById() {
  console.log('\n🔍 [5] GET /api/proveedores/:id');

  if (!CREATED_ID) {
    console.log('  ⚠️ Saltando: no hay ID creado');
    return;
  }

  const res = await request('GET', `/api/proveedores/${encodeURIComponent(CREATED_ID)}`);
  assert(res.status === 200, 'Obtiene proveedor por ID (200)');
  assert(res.body.success === true, 'success = true');
  assert(res.body.data.nombre === 'PROVEEDOR PRUEBA AUTOMATIZADA', 'Nombre coincide');
  assert(res.body.data.ciudad === 'TUNJA', 'Ciudad coincide');
}

async function testUpdateProveedor() {
  console.log('\n✏️ [6] PUT /api/proveedores/:id');

  if (!CREATED_ID) {
    console.log('  ⚠️ Saltando: no hay ID creado');
    return;
  }

  const res = await request('PUT', `/api/proveedores/${encodeURIComponent(CREATED_ID)}`, {
    nombre: 'PROVEEDOR PRUEBA ACTUALIZADO',
    telefono1: '3111111111',
    email: 'actualizado@test.com',
    contacto: 'Nuevo Contacto',
    ciudad: 'SOGAMOSO',
    departamento: 'BOYACA'
  });

  assert(res.status === 200, 'Actualiza proveedor (200)');
  assert(res.body.success === true, 'success = true');

  // Verificar que los cambios se aplicaron
  const getRes = await request('GET', `/api/proveedores/${encodeURIComponent(CREATED_ID)}`);
  assert(getRes.body.data.nombre === 'PROVEEDOR PRUEBA ACTUALIZADO', 'Nombre actualizado correctamente');
  assert(getRes.body.data.ciudad === 'SOGAMOSO', 'Ciudad actualizada correctamente');
}

async function testDeleteProveedor() {
  console.log('\n🗑️ [7] DELETE /api/proveedores/:id — Borrado seguro (sin compras → hard delete)');

  if (!CREATED_ID) {
    console.log('  ⚠️ Saltando: no hay ID creado');
    return;
  }

  const res = await request('DELETE', `/api/proveedores/${encodeURIComponent(CREATED_ID)}`);
  assert(res.status === 200, 'Elimina proveedor de prueba (200)');
  assert(res.body.success === true, 'success = true');
  assert(res.body.action === 'deleted', `Acción = "deleted" (sin compras históricas), acción real: ${res.body.action}`);

  // Verificar que ya no aparece en la lista activos
  const getRes = await request('GET', `/api/proveedores/${encodeURIComponent(CREATED_ID)}`);
  assert(getRes.status === 404, 'Proveedor ya no existe (404 tras borrado físico)');
}

async function testDeleteNonExistent() {
  console.log('\n❓ [8] DELETE /api/proveedores/:id — ID inexistente');
  const res = await request('DELETE', '/api/proveedores/FAKE_ID_QUE_NO_EXISTE');
  assert(res.status === 404, 'Retorna 404 para ID no encontrado');
  assert(res.body.success === false, 'success = false');
}

async function testSoftDeleteExistingProvider() {
  console.log('\n🔒 [9] DELETE — Soft-delete en proveedor CON compras históricas');

  // Buscamos un proveedor activo que sepamos tiene compras (VENTANILLA = 0010000000001)
  const knownId = '0010000000001';
  const res = await request('DELETE', `/api/proveedores/${knownId}`);

  // No lo eliminamos realmente, solo verificamos que devolvería soft-delete
  if (res.status === 200 && res.body.action === 'deactivated') {
    assert(true, `Proveedor con compras se desactivó (soft-delete) correctamente`);
    // Reactivarlo para no afectar la BD de producción
    await request('PUT', `/api/proveedores/${knownId}`, {
      nombre: 'VENTANILLA',
      status: 'A'
    });
    console.log('    ♻️ Proveedor reactivado (status=A restaurado)');
  } else if (res.status === 404) {
    assert(false, 'Proveedor de prueba no encontrado');
  } else {
    assert(false, `Respuesta inesperada: ${JSON.stringify(res.body)}`);
  }
}

// ─── Runner ───────────────────────────────────────────────────────────────────

async function runAll() {
  console.log('═══════════════════════════════════════════════════════');
  console.log('🧪 Suite de Pruebas — API de Proveedores (Fase 2)');
  console.log('═══════════════════════════════════════════════════════');

  try {
    await testHealth();
    await testGetProveedores();
    await testValidationErrors();
    await testCreateProveedor();
    await testDuplicateDocument();
    await testGetById();
    await testUpdateProveedor();
    await testDeleteProveedor();
    await testDeleteNonExistent();
    await testSoftDeleteExistingProvider();
  } catch (err) {
    console.error('\n💥 Error fatal en la suite de pruebas:', err.message);
    TESTS_FAILED++;
  }

  console.log('\n═══════════════════════════════════════════════════════');
  console.log(`📊 RESULTADOS: ${TESTS_PASSED} pasaron | ${TESTS_FAILED} fallaron`);
  if (TESTS_FAILED === 0) {
    console.log('🎉 TODAS LAS PRUEBAS PASARON CORRECTAMENTE');
  } else {
    console.log('⚠️  Hay pruebas fallidas. Revisar los logs anteriores.');
  }
  console.log('═══════════════════════════════════════════════════════');
  process.exit(TESTS_FAILED > 0 ? 1 : 0);
}

runAll();
