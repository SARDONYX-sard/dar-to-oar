/** @type {import('eslint/lib/shared/types').ConfigData} */
module.exports = {
  plugins: ['strict-dependencies', 'unused-imports'],
  extends: ['next/core-web-vitals', 'plugin:import/recommended', 'plugin:import/warnings', 'prettier'],
  settings: {
    // NOTE: eslint cannot resolve aliases by @ without the following two settings
    // See:https://github.com/import-js/eslint-plugin-import/issues/2765#issuecomment-1701641064
    next: {
      rootDir: __dirname,
    },
    'import/resolver': {
      typescript: {
        project: __dirname,
      },
    },
  },
  rules: {
    'strict-dependencies/strict-dependencies': [
      'error',
      [
        { module: 'src/components/pages', allowReferenceFrom: [], allowSameModule: false },
        {
          module: 'src/components',
          allowReferenceFrom: ['src/components/form', 'src/components/pages'],
          allowSameModule: true,
        },
        { module: 'src/hooks', allowSameModule: false },
        { module: 'src/tauri_cmd', allowSameModule: false },
        { module: 'src/utils', allowSameModule: true },
      ],
      { resolveRelativeImport: false },
    ],
    // ref：https://github.com/benmosher/eslint-plugin-import/blob/master/docs/rules/order.md
    'import/order': [
      'error',
      {
        groups: ['builtin', 'external', 'internal', ['parent', 'sibling'], 'object', 'type', 'index'],
        'newlines-between': 'always',
        pathGroupsExcludedImportTypes: ['builtin'],
        alphabetize: { order: 'asc', caseInsensitive: true },
        pathGroups: [
          { pattern: '@/**', group: 'internal', position: 'before' },
          // styles
          // treat group as index because we want it to be last
          { pattern: '@/**.css', group: 'index', position: 'before' },
          { pattern: '@/**.json', group: 'index', position: 'before' },
        ],
      },
    ],
  },
};
