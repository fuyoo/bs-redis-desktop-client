import { Window } from '@tauri-apps/api/window'
import { Webview } from '@tauri-apps/api/webview'

export const showHostConfigureDetail = async (id: string) => {
  const appWindow = new Window('host-config-detail', {
    title: 'BS',
  })

  // loading embedded asset:
  const webview = new Webview(appWindow, 'host-config-detail', {
    url: '/#/detail/host/' + id,
  } as any)
  setTimeout(() => {
    webview?.setFocus()
  }, 10)
}
