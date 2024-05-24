//! NOTE: If this file is not in the root directory, this setting will not be reflected at test time.
import nextJest from 'next/jest.js';

const PROJECT_ROOT = 'dar2oar_gui/frontend';

const createJestConfig = nextJest({
  // Provide the path to your Next.js app to load next.config.js and .env files in your test environment
  dir: PROJECT_ROOT,
});

// Add any custom config to be passed to Jest
/** @type {import('jest').Config} */
const config = {
  preset: 'ts-jest',
  moduleNameMapper: {
    // We can use `<rootDir>`.
    '^@/(.+)': `${PROJECT_ROOT}/src/$1`,
  },
  moduleDirectories: ['node_modules', '<rootDir>'],
  testEnvironment: 'jest-environment-jsdom',
};

// createJestConfig is exported this way to ensure that next/jest can load the Next.js config which is async
export default createJestConfig(config);
