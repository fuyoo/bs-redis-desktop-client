import {defineConfig} from 'vite'
import UnoCSS from 'unocss/vite'
import vue from '@vitejs/plugin-vue'
import pkg from './package.json'
import path from 'node:path'
import dayjs from 'dayjs'

export default defineConfig(async () => ({
    plugins: [
        vue(),
        UnoCSS()
    ],
    clearScreen: false,
    server: {
        port: 1420,
        strictPort: true,
        watch: {
            // 3. tell vite to ignore watching `src-tauri`
            ignored: ['**/src-tauri/**', '.idea']
        }
    },
    define: {
        __SYSTEM_INFO__: JSON.stringify({
            pkg: {
                version: pkg.version,
                dependencies: pkg.dependencies,
                devDependencies: pkg.devDependencies
            },
            lastBuildTime: dayjs().format('YYYY-MM-DD HH:mm:ss')
        })
    },
    resolve: {
        alias: {
            '@': path.resolve(__dirname, 'src')
        }
    }
}))
