/**
 * Middleware global para el manejo de errores en Express.
 * Evita la fuga de información sensible (como queries SQL y credenciales) hacia el frontend.
 */
function errorHandler(err, req, res, next) {
  // Log detallado del error en consola (lado servidor)
  console.error('🔥 Error interno detectado:', {
    message: err.message,
    stack: err.stack,
    sql: err.sql,
    sqlState: err.sqlState,
    code: err.code
  });

  const statusCode = err.statusCode || 500;
  const clientMessage = err.isPublic ? err.message : 'Ocurrió un error interno en el servidor.';

  res.status(statusCode).json({
    success: false,
    message: clientMessage
  });
}

/**
 * Clase personalizada de error para respuestas amigables al cliente.
 */
class AppError extends Error {
  constructor(message, statusCode = 400) {
    super(message);
    this.statusCode = statusCode;
    this.isPublic = true;
    Error.captureStackTrace(this, this.constructor);
  }
}

module.exports = {
  errorHandler,
  AppError
};
