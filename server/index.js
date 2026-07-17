const express = require('express');
const cors = require('cors');
const morgan = require('morgan');
const path = require('path');
const { testConnection } = require('./config/database');
const { errorHandler, AppError } = require('./middleware/errorHandler');
const proveedoresRouter = require('./routes/proveedores');

require('dotenv').config();

const app = express();
const PORT = process.env.PORT || 3000;

// Middlewares
app.use(cors());
app.use(morgan('dev'));
app.use(express.json());

// Servir archivos estáticos del frontend
app.use(express.static(path.join(__dirname, '../public')));

// Rutas de la API
app.use('/api/proveedores', proveedoresRouter);

// Ruta básica de salud de la API
app.get('/api/health', (req, res) => {
  res.json({
    status: 'up',
    timestamp: new Date().toISOString(),
    uptime: process.uptime()
  });
});

// Control de rutas no encontradas (404)
app.use((req, res, next) => {
  next(new AppError(`Ruta no encontrada: ${req.originalUrl}`, 404));
});

// Middleware de manejo de errores global (seguro, sin filtrar detalles SQL al cliente)
app.use(errorHandler);

// Iniciar servidor y probar conexión a base de datos
if (process.env.NODE_ENV !== 'test') {
  app.listen(PORT, async () => {
    console.log(`🚀 Servidor corriendo en http://localhost:${PORT}`);
    
    // Probar conexiones a base de datos
    await testConnection();
  });
}

module.exports = app;
