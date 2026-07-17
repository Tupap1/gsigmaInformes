const { getYearTables, getAllYearTables } = require('../utils/yearTable');

describe('Year Table Resolver Tests', () => {
  test('should return correct table for a single historical year', () => {
    const result = getYearTables('compra', '2024-06-15', '2024-06-15');
    expect(result).toEqual(['compra24']);
  });

  test('should return correct tables for a range crossing years', () => {
    const result = getYearTables('compra', '2023-12-15', '2024-01-15');
    expect(result).toEqual(['compra23', 'compra24']);
  });

  test('should handle Date objects as input', () => {
    const start = new Date('2023-12-15T00:00:00Z');
    const end = new Date('2024-01-15T00:00:00Z');
    const result = getYearTables('compra', start, end);
    expect(result).toEqual(['compra23', 'compra24']);
  });

  test('should throw error if start year is after end year', () => {
    expect(() => {
      getYearTables('compra', '2024-01-01', '2023-01-01');
    }).toThrow('La fecha de inicio no puede ser posterior a la fecha de fin');
  });

  test('should throw error for unsupported year (before 2011)', () => {
    expect(() => {
      getYearTables('compra', '2010-12-31', '2011-01-01');
    }).toThrow('Año 2010 fuera de rango de soporte histórico');
  });

  test('should throw error for unsupported year (after 2035)', () => {
    expect(() => {
      getYearTables('compra', '2035-01-01', '2036-01-01');
    }).toThrow('Año 2036 fuera de rango de soporte histórico');
  });

  test('should generate all tables from 2011 to current year', () => {
    const result = getAllYearTables('compra');
    expect(result.length).toBeGreaterThan(10);
    expect(result[0]).toBe('compra11');
    expect(result[result.length - 1]).toBe('compra');
  });
});
