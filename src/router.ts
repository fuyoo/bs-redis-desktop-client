import {createMemoryHistory, createRouter} from 'vue-router'
import HomeView from '@/views/HomeView.vue'
import Layout from '@/Layout.vue'

const routes = [
    {
        path: '/',
        component: Layout,
        redirect: '/host',
        children: [
            {
                path: '/host',
                component: HomeView
            }
        ]
    }
]

const router = createRouter({
    history: createMemoryHistory(),
    routes
})

export default router