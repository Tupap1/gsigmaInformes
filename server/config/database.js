const mysql = require('mysql2/promise');
require('dotenv').config();

// Configuración común y específica para los pools.
// Para máxima compatibilidad con MySQL 5.5, el driver mysql2 negocia
// automáticamente el protocolo mysql_native_password durante el handshake.
const baseConfig = {
  host: process.env.DB_HOST || 'localhost',
  port: parseInt(process.env.DB_PORT || '3306', 10),
  connectionLimit: 5,
  charset: 'utf8',
  // Opciones de compatibilidad para servidores antiguos (MySQL 5.5):
  connectTimeout: 10000 // 10 segundos de timeout
};

const dbConfigRead = {
  ...baseConfig,
  user: process.env.DB_READ_USER || 'root',
  password: process.env.DB_READ_PASSWORD || ''
};

const dbConfigWrite = {
  ...baseConfig,
  user: process.env.DB_WRITE_USER || 'root',
  password: process.env.DB_WRITE_PASSWORD || ''
};

// Crear pools de conexión separados según requisitos de seguridad
console.log('🔄 Inicializando pools de conexión a base de datos...');
const readPool = mysql.createPool(dbConfigRead);
const writePool = mysql.createPool(dbConfigWrite);

/**
 * Función para probar la conectividad de ambos pools
 */
async function testConnection() {
  let allOk = true;

  try {
    // Probamos con un query simple que no requiere esquema seleccionado
    const [rows] = await readPool.query('SELECT 1 AS test');
    console.log('✅ ReadPool (Lectura) conectado correctamente.');
  } catch (error) {
    console.error('❌ Error de conexión en ReadPool (Lectura):', error.message);
    allOk = false;
  }

  try {
    const [rows] = await writePool.query('SELECT 1 AS test');
    console.log('✅ WritePool (Escritura) conectado correctamente.');
  } catch (error) {
    console.error('❌ Error de conexión en WritePool (Escritura):', error.message);
    allOk = false;
  }

  return allOk;
}

module.exports = {
  readPool,
  writePool,
  testConnection
};
