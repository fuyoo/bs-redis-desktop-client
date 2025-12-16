import { Webview } from '@tauri-apps/api/webview'
import { lightTheme, darkTheme } from 'naive-ui'
import { shallowRef, onMounted, onBeforeUnmount, inject } from 'vue'
import { req } from '@/api'
import { listen } from '@tauri-apps/api/event'
export const useResize = (sub?: number) => {
  const height = shallowRef(100)
  const width = shallowRef(0)
  let resizeTimer = 0 as any
  const onResize = () => {
    clearTimeout(resizeTimer)
    resizeTimer = setTimeout(() => {
      const dom = document.querySelector('#app')
      height.value = (dom?.getBoundingClientRect()?.height || 0) - (sub || 0)
      width.value = dom?.getBoundingClientRect()?.width || 0
    }, 50)
  }
  // dynamic set tree height
  onMounted(() => {
    window.addEventListener('resize', onResize)
    onResize()
  })

  onBeforeUnmount(() => {
    window.removeEventListener('resize', onResize)
    clearTimeout(resizeTimer)
  })

  return {
    height,
    width,
  }
}
type ThemeMode = 'light' | 'dark' | 'auto'
export const useTheme = () => {
  const theme = shallowRef(lightTheme)
  const storageKey = 'theme-mode'
  const setTheme = (
    mode: 'light' | 'dark' | 'auto',
    t?: Ref<typeof lightTheme | typeof darkTheme>,
  ) => {
    localStorage.setItem(storageKey, mode)
    switch (mode) {
      case 'light':
        theme.value = lightTheme
        unlistenTheme()
        break
      case 'dark':
        theme.value = darkTheme
        unlistenTheme()
        break
      default:
        listenTheme()
    }
    if (t) {
      if (mode === 'auto') {
        t.value = window.matchMedia('(prefers-color-scheme: dark)').matches ? darkTheme : lightTheme
      } else {
        t.value = theme.value
      }
      // sync theme.
      req.do('emit_event', {
        evt: 'change_theme',
        data: mode,
      })
    }
  }
  const onMediaChange = (e: MediaQueryListEvent) => {
    setTheme(e.matches ? 'dark' : 'light')
  }
  const listenTheme = () => {
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', onMediaChange)
  }
  const unlistenTheme = () => {
    window.matchMedia('(prefers-color-scheme: dark)').removeEventListener('change', onMediaChange)
  }
  const getMode = () => {
    return (localStorage.getItem(storageKey) || 'auto') as ThemeMode
  }
  const initThemeMode = () => {
    const mode = getMode()
    setTheme(mode)
  }

  // change all Webview theme
  const listenChange = () => {
    listen('emit-event', (e: any) => {
      if (e.payload?.evt === 'change_theme') {
        setTheme(e.payload.data)
      }
    })
  }
  return {
    theme,
    initThemeMode,
    getMode,
    setTheme,
    listenChange,
  }
}
