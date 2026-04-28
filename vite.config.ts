import { defineConfig } from 'vite-plus';

export default defineConfig({
  lint: {
    plugins: ['eslint', 'typescript', 'unicorn', 'react', 'react-perf', 'oxc'],
    options: { typeAware: true, typeCheck: true },
    rules: {
      eqeqeq: 'warn',
      'eslint/no-unused-vars': 'error',
    },
  },
  fmt: {
    sortImports: {
      newlinesBetween: false,
      groups: [
        ['value-builtin', 'value-external'],
        ['value-internal', 'value-parent', 'value-sibling', 'value-index'],
        { newlinesBetween: true },
        'type-import',
        'unknown',
      ],
    },
    jsxSingleQuote: true,
    printWidth: 120,
    semi: true,
    singleQuote: true,
    sortPackageJson: true,
    ignorePatterns: ['routeTree.gen.ts', 'cspell.jsonc', 'CHANGELOG.md'],
  },
});
