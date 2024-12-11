<script lang="ts" setup>
import { useQuasar, type QuasarLanguage } from 'quasar'
import { useI18n } from 'vue-i18n'
import { ref, watch } from 'vue'
// import lang pack
const languages = import.meta.glob('../../../../node_modules/quasar/lang/(zh-CN|en-US).js', {
  eager: true,
  import: 'default',
})
// generate lang options
const options = Object.keys(languages).map((k) => (languages[k] as Record<string, any>).nativeName)
// init quasar instance
const $q = useQuasar()
// obtain currently luanguage
const localLang = localStorage.getItem('lang') ?? ($q.lang.getLocale() || 'zh-CN')
// reflect to nativeName
const name = Object.keys(languages)
  .map((k) => languages[k] as Record<string, any>)
  .filter((e) => e.isoName == localLang)[0]?.nativeName
// assign nativeName to ref model
const lang = ref(name)
const $i18n = useI18n()
// watch value change set language
watch(lang, (v: string) => {
  Object.keys(languages).forEach((k) => {
    const val = languages[k] as QuasarLanguage
    if (val.nativeName == v) {
      localStorage.setItem('lang', val.isoName)
      $q.lang.set(val)
      $i18n.locale.value = val.isoName
    }
  })
})
</script>
<template>
  <q-select
    option-value="isoName"
    option-label="nativeName"
    standout
    v-model="lang"
    :options="options"
    :dense="true"
  />
</template>
