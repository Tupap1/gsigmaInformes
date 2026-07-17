/**
 * Módulo para resolver nombres de tablas anuales y particionadas en el POS.
 */

const CURRENT_YEAR = new Date().getFullYear(); // e.g. 2026

/**
 * Retorna las tablas que corresponden a un rango de fechas.
 * @param {string} baseName Nombre base de la tabla (e.g. 'compra', 'venta')
 * @param {string|Date} startDate Fecha de inicio (YYYY-MM-DD)
 * @param {string|Date} endDate Fecha de fin (YYYY-MM-DD)
 * @returns {string[]} Array de nombres de tablas correspondientes
 */
function getYearTables(baseName, startDate, endDate) {
  let startYear, endYear;

  if (typeof startDate === 'string') {
    startYear = parseInt(startDate.split('-')[0], 10);
  } else if (startDate instanceof Date) {
    // Si es un objeto Date, obtenemos el año en hora local
    startYear = startDate.getFullYear();
  } else {
    startYear = new Date(startDate).getFullYear();
  }

  if (typeof endDate === 'string') {
    endYear = parseInt(endDate.split('-')[0], 10);
  } else if (endDate instanceof Date) {
    endYear = endDate.getFullYear();
  } else {
    endYear = new Date(endDate).getFullYear();
  }

  if (isNaN(startYear) || isNaN(endYear)) {
    throw new Error('Fechas inválidas proporcionadas a getYearTables');
  }

  if (startYear > endYear) {
    throw new Error('La fecha de inicio no puede ser posterior a la fecha de fin');
  }

  const tables = [];

  for (let year = startYear; year <= endYear; year++) {
    // Validar rango permitido (2011 a 2035)
    if (year < 2011 || year > 2035) {
      throw new Error(`Año ${year} fuera de rango de soporte histórico (2011-2035)`);
    }

    if (year === CURRENT_YEAR) {
      tables.push(baseName);
    } else {
      const suffix = String(year).slice(-2);
      tables.push(`${baseName}${suffix}`);
    }
  }

  // Eliminar duplicados si los hay
  return [...new Set(tables)];
}

/**
 * Retorna todas las tablas desde 2011 hasta el año actual para búsquedas históricas exhaustivas.
 * @param {string} baseName Nombre base de la tabla (e.g. 'compra')
 * @returns {string[]} Array de nombres de tablas desde 2011 hasta el año actual
 */
function getAllYearTables(baseName) {
  const tables = [];
  for (let year = 2011; year <= CURRENT_YEAR; year++) {
    if (year === CURRENT_YEAR) {
      tables.push(baseName);
    } else {
      const suffix = String(year).slice(-2);
      tables.push(`${baseName}${suffix}`);
    }
  }
  return tables;
}

module.exports = {
  getYearTables,
  getAllYearTables,
  CURRENT_YEAR
};
