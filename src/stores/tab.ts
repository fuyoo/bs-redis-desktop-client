import { invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'
import { shallowRef } from 'vue'
import { useRoute, useRouter } from 'vue-router'
export const useTabStore = defineStore('tab', () => {
  // tab list
  const tabList = shallowRef<Tab[]>([])
  // route
  const route = useRoute()
  // router
  const router = useRouter()
  //
  const close = async (id: string) => {
    const t = tab(id)
    // if no data, break
    if (!t) {
      return
    }
    // close webview at rust side.
    await invoke('tab_close', { tab: { ...t } })
    // update tabs list.
    const index = tabList.value.findIndex((e) => e.id === id)
    // update
    await update()
    const nid = tabList.value[index]?.id || tabList.value[index - 1]?.id
    // focus on next tab.
    if (route.params.id) {
      if (nid) {
        await router.push(`/host/${nid}`)
      } else {
        // if tab empty, go to home page.
        await router.push(`/`)
      }
    }
  }
  // obtain tab object
  const tab = (id: string) => {
    return tabList.value.filter((e) => e.id == id)[0]
  }
  // obtain focus tab object
  const focusTab = () => {
    const id = route.params.id
    return tab(id as string)
  }
  // change tab to focus
  const change = async (tab: Tab) => {
    await invoke('tab_change', { tab: tab })
    await update()
    if (tab.id == 'main') {
      router.push('/')
    } else {
      router.push(`/host/${tab.id}`)
    }
  }
  const update = async () => {
    const resp = await invoke<BackendResponse<Tab[]>>('tab_list')
    tabList.value = resp.data
  }

  return { tabList, close, change, update, tab, focusTab }
})
