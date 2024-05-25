import i18next from 'i18next'
import I18NextVue from 'i18next-vue'
import LanguageDetector from 'i18next-browser-languagedetector'
import {App} from 'vue'
import en from './en.ts'
import zh_cn from './zh_cn.ts'
i18next
    // detect user language
    // learn more: https://github.com/i18next/i18next-browser-languageDetector
    .use(LanguageDetector)
    // init i18next
    // for all options read: https://www.i18next.com/overview/configuration-options
    .init({
        debug: true,
        fallbackLng: 'en',
        resources: {
            en: {
                translation: en
            },
            "zh-CN":{
                translation: zh_cn
            }
        }
    });

export default {
    install(app:App) {
        app.use(I18NextVue, { i18next })
    }
}