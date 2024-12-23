import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { Quasar, Dialog } from 'quasar'

// Import icon libraries
import '@quasar/extras/material-icons/material-icons.css'
// Import Quasar css
import 'quasar/src/css/index.sass'
import App from './App.vue'
import router from './router'
import { useI18n } from './i18n'
import 'virtual:uno.css'
const app = createApp(App)
app.use(Quasar, {
  plugins: {
    Dialog,
  }, // import Quasar plugins and add here
})
app.use(useI18n)
app.use(createPinia())
app.use(router)

app.mount('#app')
