import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'
import { injectI18n } from './i18n'
import 'virtual:uno.css'
import { buildTray } from './tools/tray'
const app = createApp(App)
app.use(injectI18n)
app.use(createPinia())
app.use(router)
app.mount('#app')
