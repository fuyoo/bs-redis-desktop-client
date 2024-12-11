import { defineBoot } from '#q-app/wrappers'
import { createI18n } from 'vue-i18n'

import messages from 'src/i18n'
import { Lang, type QuasarLanguage } from 'quasar'
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

export default defineBoot(({ app }) => {
  // import lang pack
  const languages = import.meta.glob('../../node_modules/quasar/lang/(zh-CN|en-US).js', {
    eager: true,
    import: 'default',
  })
  // obtain currently luanguage
  const local = localStorage.getItem('lang') ?? (Lang.getLocale() || 'zh-CN')
  // set currently language
  Object.keys(languages).forEach((k) => {
    const val = languages[k] as QuasarLanguage
    if (val.isoName == local) {
      Lang.set(val)
    }
  })
  const i18n = createI18n<{ message: MessageSchema }, MessageLanguages>({
    locale: local,
    legacy: false,
    messages,
  })

  // Set i18n instance on app
  app.use(i18n)
})
