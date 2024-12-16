import type { RouteRecordRaw } from 'vue-router'
import settings from '../pages/settings/index.vue'
const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: () => import('layouts/MainLayout.vue'),
    redirect: '/home',
    children: [
      { path: '/home', component: () => import('pages/home/index.vue') },
      { path: '/settings', component: settings },
      {
        path: '/host/:id',
        component: () => import('pages/host/index.vue'),
      },
    ],
  },
  {
    path: '/host/:id',
    component: () => import('layouts/HostViewLayout.vue'),
    children: [{ path: 'status', component: () => import('pages/host/index.vue') }],
  },
  // Always leave this as last one,
  // but you can also remove it
  {
    path: '/:catchAll(.*)*',
    component: () => import('pages/errors/NotFound.vue'),
  },
]

export default routes
