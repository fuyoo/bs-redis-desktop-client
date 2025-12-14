import type { RouteRecordRaw } from 'vue-router'
import settings from '@/pages/settings/index.vue'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: () => import('@/layouts/layout.vue'),
    redirect: '/home',
    children: [
      { path: '/home', component: () => import('@/pages/home/home.vue') },
      { path: '/settings', component: settings },
    ]
  },
  /*{
    path: '/',
    component: () => import('@/layouts/MainLayout.vue'),
    redirect: '/home',
    children: [
      { path: '/home', component: () => import('@/pages/home/index.vue') },
      { path: '/settings', component: settings },
      {
        path: '/host/:id',
        component: () => import('@/pages/host/empty.vue'),
      },
      {
        path: '/detail/host/:id',
        component: () => import('@/pages/host/detail.vue'),
      },
    ],
  },*/
  {
    path: '/tab/:id',
    component: () => import('@/layouts/HostViewLayout.vue'),
    children: [
      {
        path: 'main',
        component: () => import('@/pages/host/layout.vue'),
        children: [
          {
            path: 'info',
            component: () => import('@/pages/host/host-info.vue'),
          },
          {
            path: 'database',
            component: () => import('@/pages/host/database.vue'),
            children: [
              {
                path: 'string/:key',
                component: () =>
                  import('@/pages/host/key-types/co-string/index.vue'),
              },
              {
                path: 'set/:key',
                component: () => import('@/pages/host/key-types/co-set/index.vue'),
              },
              {
                path: 'none/:key',
                component: () => import('@/pages/host/key-types/co-none/index.vue'),
              },
              {
                path: 'list/:key',
                component: () => import('@/pages/host/key-types/co-list/index.vue'),
              },
              {
                path: 'hash/:key',
                component: () => import('@/pages/host/key-types/co-hash/index.vue'),
              },
              {
                path: 'zset/:key',
                component: () => import('@/pages/host/key-types/co-zset/index.vue'),
              },
              {
                path: 'unsupported/:key',
                component: () => import('@/pages/host/key-types/co-unsupported/index.vue'),
              },
            ]
          },
        ],
      },
    ],
  },
  // Always leave this as last one,
  // but you can also remove it
  {
    path: '/:catchAll(.*)*',
    component: () => import('@/pages/errors/NotFound.vue'),
  },
]

export default routes
