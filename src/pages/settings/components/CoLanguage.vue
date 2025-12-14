<script lang="ts" setup>
import { useI18n } from 'vue-i18n'
import { ref } from 'vue'
// import lang pack
const languages = import.meta.glob('../../../../node_modules/quasar/lang/(zh-CN|en-US).js', {
  eager: true,
  import: 'default',
})
// generate lang options
const options = Object.keys(languages).map((k) => ({
  label: (languages[k] as Record<string, any>).nativeName,
  value: (languages[k] as Record<string, any>).isoName,
}))
// obtain currently language
const localLang = localStorage.getItem('lang') ?? 'en-US'
// reflect to nativeName
const name = Object.keys(languages)
  .map((k) => languages[k] as Record<string, any>)
  .filter((e) => e.isoName == localLang)[0]?.nativeName
// assign nativeName to ref model
const lang = ref(name)
const $i18n = useI18n()
function handleSelect(v: string) {
  lang.value = options.find((e) => e.value == v)!.label
  localStorage.setItem('lang', v)
  $i18n.locale.value = v
  location.reload()
}
</script>
<template>
  <n-dropdown
    key-field="value"
    label-field="label"
    trigger="hover"
    :options="options"
    @select="handleSelect"
  >
    <n-button>{{ lang }}</n-button>
  </n-dropdown>
</template>
