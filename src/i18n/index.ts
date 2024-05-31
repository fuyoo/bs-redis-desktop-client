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
        },
    });
export const Languages = [
    {
       label: "English",
       value: "en",
    },{
    label:"简体中文",
        "value":'zh-CN'
    }
]

import database from "@/database.ts"
export default {
    install(app:App) {
        database.settings.get(1)
            .then(async (data) =>{
                if (data) {
                 await  i18next.changeLanguage( data.lang)
                }
            })
            .catch(e => console.error(e))
        app.use(I18NextVue, { i18next })
    }
}