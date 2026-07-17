const { testConnection } = require('./database');

async function run() {
  console.log('🚀 Iniciando validación de conexiones a MySQL...');
  const success = await testConnection();
  if (success) {
    console.log('🎉 Todas las conexiones se establecieron con éxito.');
    process.exit(0);
  } else {
    console.error('⚠️ Se detectaron fallas de conexión. Verifica tu archivo .env y el estado de tu base de datos.');
    process.exit(1);
  }
}

run();
