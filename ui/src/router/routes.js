import Layout from ':/layout/index'

export default [
    {
        path: '/',
        component: Layout,
        redirect: '/home',
        children: [
            {
                path: '/home',
                component: () => import('::/home')
            }
        ]
    },
    {
        path: '/detail',
        component: () => import('::/detail')
    },
    {
        path: '/set',
        component: () => import('::/set')
    },
    {
        path: '/look-more',
        component: () => import('::/lookMore')
    }
]
