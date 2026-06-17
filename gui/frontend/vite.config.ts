import babel from '@rolldown/plugin-babel';
import { tanstackRouter } from '@tanstack/router-plugin/vite';
import react, { reactCompilerPreset } from '@vitejs/plugin-react';
import { defineConfig } from 'vite-plus';

// var ref: https://v2.tauri.app/reference/environment-variables/#tauri-cli-hook-commands
const IS_DEBUG = !!process.env.TAURI_ENV_DEBUG;

export default defineConfig({
  resolve: { tsconfigPaths: true },
  plugins: [
    tanstackRouter({
      target: 'react',
      autoCodeSplitting: true,
    }),
    react(),
    // See: https://github.com/vitejs/vite-plugin-react/releases/tag/plugin-react%406.0.0
    babel({
      presets: [reactCompilerPreset()],
    }),
  ],
  build: {
    sourcemap: IS_DEBUG,
  },
  test: {
    alias: [{ find: '@/', replacement: `${__dirname}/src/` }],
    globals: true,
    root: `./src/`,
    environment: 'jsdom',
    setupFiles: [`${__dirname}/vitest.setup.mts`],
    reporters: ['default', 'hanging-process'],
    server: {
      // Avoid `TransitionGroupContext' is not supported resolving ES modules
      // - ref: https://zenn.dev/kanata_o/articles/framework-upgrade-pitfalls
      deps: { inline: [/@mui\//, /react-transition-group/] },
    },
  },
});
