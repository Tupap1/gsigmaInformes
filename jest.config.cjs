module.exports = {
  testEnvironment: 'node',
  verbose: true,
  testMatch: ['**/server/tests/**/*.test.js'],
  setupFiles: ['<rootDir>/server/tests/setupEnv.js'],
  clearMocks: true,
  restoreMocks: true
};
