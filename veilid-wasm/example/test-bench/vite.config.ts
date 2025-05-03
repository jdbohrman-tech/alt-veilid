import { defineConfig, searchForWorkspaceRoot } from 'vite';
import react from '@vitejs/plugin-react-swc'
import wasm from 'vite-plugin-wasm';
import path from 'path';

// search up for workspace root
const workspaceRoot = searchForWorkspaceRoot(process.cwd());

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), wasm()],
  server: {
    port: 5001,
    fs: {
      allow: [
        workspaceRoot,
        path.resolve(
          workspaceRoot,
          '../../pkg/'
        ),
      ],
    },

  },
})
