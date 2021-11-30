import { createRouter, createWebHashHistory } from 'vue-router'
import routes from './routes.js'
import storage from ':/tools/storage'

const router = createRouter({
  history: createWebHashHistory(),
  routes
})
const whiteNameList = ['/login']
router.beforeEach(async (to, from, next) => {
  next()
})
export default router
