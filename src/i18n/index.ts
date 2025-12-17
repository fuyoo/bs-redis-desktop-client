import { listen } from '@tauri-apps/api/event'
import { createI18n, useI18n } from 'vue-i18n'

import messages from './lang'

import { type App, computed } from 'vue'
import { enUS, ruRU, zhCN } from 'naive-ui'

export type MessageLanguages = keyof typeof messages
// Type-define 'en-US' as the master schema for the resource
export type MessageSchema = (typeof messages)['en-US']

// See https://vue-i18n.intlify.dev/guide/advanced/typescript.html#global-resource-schema-type-definition
/* eslint-disable @typescript-eslint/no-empty-object-type */
declare module 'vue-i18n' {
  // define the locale messages schema
  export interface DefineLocaleMessage extends MessageSchema {}

  // define the datetime format schema
  export interface DefineDateTimeFormat {}

  // define the number format schema
  export interface DefineNumberFormat {}
}
/* eslint-enable @typescript-eslint/no-empty-object-type */
let i18n: any
export const injectI18n = (app: App) => {
  // obtain currently language
  const local = localStorage.getItem('lang') ?? 'en-US'
  i18n = createI18n<{ message: MessageSchema }, MessageLanguages>({
    locale: local,
    legacy: false,
    messages,
  })
  // Set i18n instance on app
  app.use(i18n)
}
export const t = (...args: any[]) => {
  return i18n.global.t(...args)
}
export const getLocate = () => {
  // obtain currently language
  const local = localStorage.getItem('lang') ?? 'en-US'
  console.log(`obtain currently language`, local)
  switch (local) {
    case 'zh-CN':
      return zhCN
    case 'ruRU':
      return ruRU
    default:
      return enUS
  }
}
export const useLocate = () => {
  const $i18n = useI18n()
  const locate = computed(() => {
    console.log($i18n.locale.value)
    switch ($i18n.locale.value) {
      case 'zh-CN':
        return zhCN
      case 'ruRU':
        return ruRU
      default:
        return enUS
    }
  })
  const listenLocateChange = () => {
    listen('emit-event', (e: any) => {
      if (e.evt === 'change_locale') {
        $i18n.locale.value = e.payload.data
      }
    })
  }
  return { locate, listenLocateChange }
}
