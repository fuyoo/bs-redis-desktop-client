<script lang="ts" setup>
import { useI18n } from 'vue-i18n'
import { ref } from 'vue'

// generate lang options
const options = [{
  label: "简体中文(zh-CN)",
  value: "zh-CN",
}, {
  label: "English(en-US)",
  value: "en-US",
},]
// obtain currently language
const localLang = localStorage.getItem('lang') ?? 'en-US'
// reflect to nativeName
const name = options.filter((e) => e.value == localLang)[0]?.label
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
  <n-dropdown key-field="value" label-field="label" trigger="hover" :options="options" @select="handleSelect">
    <n-button>{{ lang }}</n-button>
  </n-dropdown>
</template>
