import {createApp} from 'vue'
import router from './router'
import 'virtual:uno.css'
import App from './App.vue'
import i18n from './i18n'
import ArcoVue from '@arco-design/web-vue'
import "@arco-design/web-vue/dist/arco.css"
import { createPinia } from 'pinia'
createApp(App).use(router).use(createPinia()).use(ArcoVue).use(i18n).mount('#app')