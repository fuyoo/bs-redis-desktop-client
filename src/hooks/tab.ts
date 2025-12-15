import type { ComposerTranslation } from 'vue-i18n'
import { req } from '@/api'
import { getCurrentWebview } from '@tauri-apps/api/webview'
import { useRoute } from 'vue-router'
export const useTab = ($t: ComposerTranslation) => {
  // tab list
  const tabList = shallowRef<Tab[]>([])
  const route = useRoute()
  //
  const close = async (id: string) => {
    const t = tab(id)
    // if no data, break
    if (!t) {
      return
    }
    // close webview at rust side.
    await req.do('tab_close', { tab: { ...t } })
    // update
    await update()
  }
  // obtain tab object
  const tab = (id: string) => {
    return tabList.value.filter((e) => e.id == id)[0]
  }
  // change tab to focus
  const change = async (tab: Tab) => {
    tab.id = tab.id.toString()
    await req.do('tab_change', { tab: tab })
    await update()
  }
  const update = async () => {
    const resp = await req.do<Tab[]>('tab_list')
    tabList.value = [
      {
        id: 'main',
        name: $t('home.name'),
      },
      ...resp.data,
    ]
  }
  const current = () => {
    const resp = getCurrentWebview()
    return tab(resp.label)
  }
  return { tabList, close, change, update, tab, current }
}
