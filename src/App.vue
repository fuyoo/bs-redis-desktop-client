<script setup lang="ts">
import { RouterView } from 'vue-router'
import { useLocate } from '@/i18n'
import { useTheme } from './hooks/life';
import { buildTray } from './tools/tray';
const { theme, initThemeMode, listenChange } = useTheme()
provide('theme', theme)
const { locate, listenLocateChange } = useLocate()
initThemeMode()
listenChange()
listenLocateChange()
buildTray()
</script>

<template>
  <n-config-provider :locale="locate" :theme="theme">
    <n-dialog-provider>
      <RouterView />
    </n-dialog-provider>
  </n-config-provider>
</template>

<style scoped></style>
