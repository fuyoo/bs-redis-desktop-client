import {defineStore} from 'pinia'
import {computed, ref} from 'vue'
import router from '../router'

export interface TabItem {
    id: number,
    name: string
}

export default defineStore('tabs', () => {
    const tabs = ref<Map<number, TabItem>>(new Map())
    const activeId = ref(0)
    const list = computed<TabItem[]>(() => {
        let s = [] as TabItem[]
        tabs.value.forEach((v, k) => {
            s.push(v)
        })
        return [...tabs.value].map((k) => k[1])
    })
    const changeTab = async (tabId: number) => {
        activeId.value = tabId
        await router.push({
            path: '/tab',
            query: {...activeTab.value}
        })
    }
    const activeTab = computed<TabItem>(() => tabs.value.get(activeId.value))

    const addTab = async (tab: TabItem) => {
        tabs.value.set(tab.id, tab)
        activeId.value = tab.id
        await router.push({
            path: '/tab',
            query: {...tab}
        })
    }

    const delTab = (id: number) => {
        tabs.value.delete(id)
        if (activeId.value == id) {
            // 返回上一个
            router.back()
        }
    }
    return {list, changeTab, activeTab, addTab, delTab}
})