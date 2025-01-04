import { fileURLToPath, URL } from 'node:url'

import { defineConfig, loadEnv } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import vueDevTools from 'vite-plugin-vue-devtools'
import UnoCSS from 'unocss/vite'
import { quasar, transformAssetUrls } from '@quasar/vite-plugin'
import { readFileSync } from 'node:fs'
import path from 'node:path'

// https://vite.dev/config/
export default defineConfig(({ mode }) => {
  const pkg = readFileSync(path.join(__dirname, 'package.json')).toString()
  return {
    plugins: [
      vue({
        template: { transformAssetUrls },
      }),
      vueJsx(),
      vueDevTools(),
      UnoCSS(),
      quasar({
        sassVariables: fileURLToPath(new URL('./src/css/quasar.variables.scss', import.meta.url)),
      }),
    ],
    resolve: {
      alias: {
        '@': fileURLToPath(new URL('./src', import.meta.url)),
      },
    },
    server: {
      port: 9000,
    },
    define: {
      __APP_PKG: pkg,
    },
  }
})
