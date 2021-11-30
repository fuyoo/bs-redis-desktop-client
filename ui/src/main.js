import {createApp} from 'vue'
import router from ':/router'
import Layout from ':/layout'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import zh from 'element-plus/lib/locale/lang/zh-cn'
import Icon from ':/components/Icon'
import store from ':/store'
import "./global_event"
import alert from ':/tools/alert'
const vueApp = createApp(Layout)
vueApp.use(alert)
vueApp.use(store)
vueApp.use(Icon)
vueApp.use(router)
vueApp.use(ElementPlus, {size: 'mini', locale: zh,})
vueApp.mount('#app')

