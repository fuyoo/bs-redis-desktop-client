import { Window } from '@tauri-apps/api/window'
import { Webview } from '@tauri-apps/api/webview'
import { createDiscreteApi, darkTheme, lightTheme, zhCN } from 'naive-ui'
import { getLocate } from '@/i18n'
import { parse } from 'platform'

export const storageKey = 'theme-mode'
type ThemeMode = 'light' | 'dark' | 'auto'
export const getMode = () => {
  return (localStorage.getItem(storageKey) || 'auto') as ThemeMode
}
export const getTheme = () => {
  const mode = getMode()
  switch (mode) {
    case 'light':
      return lightTheme
    case 'dark':
      return darkTheme
    default:
      return window.matchMedia('(prefers-color-scheme: dark)').matches ? darkTheme : lightTheme
  }
}
const configProviderProps = () => {
  const locate = getLocate()
  return {
    theme: getTheme(),
    locate: zhCN,
  }
}

const { message, dialog, notification, loadingBar, modal } = createDiscreteApi(
  ['message', 'dialog', 'notification', 'loadingBar', 'modal'],
  {
    configProviderProps: configProviderProps(),
  },
)
export const createHelper = () => {
  const locate = getLocate()
  const opt = {
    theme: getTheme(),
    locate,
  }
  return createDiscreteApi(['message', 'dialog', 'notification', 'loadingBar', 'modal'], {
    configProviderProps: opt,
  })
}

export { message, dialog, notification, loadingBar, modal }

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

export const dataToHuman = (data: string): string => {
  const bytes = parseInt(data, 10)
  console.log(bytes)
  if (isNaN(bytes) || bytes < 0) return '0 B'

  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB']
  const index = Math.floor(Math.log(bytes) / Math.log(1024))
  const size = bytes / Math.pow(1024, index)

  return `${size.toFixed(0)} ${sizes[index]}`
}

export const Platform = (() => {
  const os = parse()
  return {
    macos: os.os?.family?.includes('OS X'),
    windows: os.os?.family?.includes('Windows'),
    linux: os.os?.family?.includes('Linux'),
  }
})()
