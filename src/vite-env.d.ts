/// <reference types="vite/client" />

declare module '*.vue' {
    import type {DefineComponent} from 'vue'
    const component: DefineComponent<{}, {}, any>
    export default component
}

declare interface Window {
    __SYSTEM_INFO__: {
        pkg: {
            version: string,
            dependencies: Record<string, string>
            devDependencies: Record<string, string>
        },
        lastBuildTim: string
    }
}