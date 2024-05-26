import {defineStore} from 'pinia'
import {computed, ref} from 'vue'
import router from '../router'
import i18next from 'i18next'

export interface TabItem {
    id: number,
    name: string,
    path: string
}

const initTab = (): Map<number, TabItem> => {
    const map = new Map()
    map.set(-1, {
        id: -1,
        name: i18next.t('layout.host'),
        path: '/host'
    })
    map.set(-2, {
        id: -2,
        name: i18next.t('layout.settings'),
        path: '/settings'
    })
    return map
}
const defaultMenuIds = [-1, -2]
export default defineStore('tabs', () => {

    const tabs = ref<Map<number, TabItem>>(initTab())
    const activeId = ref(-1)
    const list = computed(() => {
        let s = [] as TabItem[]
        tabs.value.forEach((v) => {
            s.push(v)
        })
        // hide the default menu items
        return [...tabs.value].map((k) => k[1]).filter(e => defaultMenuIds.indexOf(e.id) == -1)
    })
    let scroll = () => {
    }
    const changeTab = async (tabId: number) => {
        activeId.value = tabId
        console.log(activeTab.value)
        await router.push({
            path: activeTab.value?.path,
            query: {...activeTab.value}
        })
    }

    const activeTab = computed(() => tabs.value.get(activeId.value))

    const addTab = async (tab: TabItem) => {
        tabs.value.set(tab.id, tab)
        activeId.value = tab.id
        await router.push({
            path: tab.path,
            query: {...tab}
        })
        scroll()
    }


    const delTab = async (id: number) => {
        // first get the tab position.
        const data = list.value
        const len = data.length
        let index = 0
        // first get the tab index
        for (let i = 0; i < len; i++) {
            const item = data[i]
            if (item.id == id) {
                index = i
                break
            }
        }
        let nextId = -1
        // if the index eq the tab length, so this is the last element. get a tab data ahead.
        if (index == len - 1) {
            const item = data[index - 1]
            if (item) {
                nextId = item.id
            }
        } else {
            const item = data[index + 1]
            if (item) {
                nextId = item.id
            }
        }
        // delete tab
        tabs.value.delete(id)
        // setActiveTab
        await changeTab(nextId)
    }

    const setScroller = (cb: () => void) => {
        scroll = cb
    }
    return {list, changeTab, activeTab, addTab, delTab, setScroller}
})