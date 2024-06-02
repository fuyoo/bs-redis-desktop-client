import {createMemoryHistory, createRouter} from 'vue-router'
import HostsView from '@/views/HostView/Host.vue'
import Layout from '@/layout/Layout.vue'

const routes = [
    {
        path: '/',
        component: Layout,
        redirect: '/host',
        children: [
            {
                path: '/host',
                component: HostsView
            },
            {
                path: '/settings',
                component: () => import("@/views/SettingsView/SettingsView.vue")
            },
            {
                path: '/tab/:id',
                component: () => import("@/layout/TabViewLayout.vue")
            }
        ]
    }
]

const router = createRouter({
    history: createMemoryHistory(),
    routes
})

export default router