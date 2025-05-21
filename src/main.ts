import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { Quasar, Dialog, Notify } from 'quasar'

// Import icon libraries
import '@quasar/extras/material-icons/material-icons.css'
// Import Quasar css
import 'quasar/src/css/index.sass'
import App from './App.vue'
import router from './router'
import { injectI18n } from './i18n'
import 'virtual:uno.css'
const app = createApp(App)
app.use(Quasar, {
  plugins: {
    Dialog,
    Notify,
  }, // import Quasar plugins and add here
})
app.use(injectI18n)
app.use(createPinia())
app.use(router)

app.mount('#app')
