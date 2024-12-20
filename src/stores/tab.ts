import { invoke } from '@tauri-apps/api/core'
import { defineStore } from 'pinia'
import { shallowRef } from 'vue'
import { useRoute, useRouter } from 'vue-router'
export const useTabStore = defineStore('tab', () => {
  const tabList = shallowRef<{ id: string; name: string }[]>([])
  const route = useRoute()
  const router = useRouter()
  const close = async (id: string) => {
    // close webview at rust side.
    await invoke('tab_close', { tab: { id } })
    // update tabs list.
    const index = tabList.value.findIndex((e) => e.id === id)
    tabList.value.splice(
      tabList.value.findIndex((e) => e.id === id),
      1,
    )
    const nid = tabList[index]?.id || tabList[index - 1]?.id
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

  const change = async (tab: Tab) => {
    await invoke('tab_change', { tab: tab })
    await update()
  }
  const update = async () => {
    const resp = await invoke<BackendResponse<Tab[]>>('tab_list')
    console.log(resp)
    tabList.value = resp.data
  }

  return { tabList, close, change, update }
})
