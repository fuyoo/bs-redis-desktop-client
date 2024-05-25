import {createApp} from 'vue'
import router from './router'
import 'virtual:uno.css'
import App from './App.vue'
import i18n from './i18n'

createApp(App).use(router).use(i18n).mount('#app')