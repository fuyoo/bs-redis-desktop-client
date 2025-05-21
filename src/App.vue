<script setup lang="ts">
import { RouterView } from 'vue-router'

import { useLocate } from '@/i18n'

const { locate } = useLocate()
</script>

<template>
  <n-config-provider :locale="locate">
    <n-message-provider>
      <RouterView />
    </n-message-provider>
  </n-config-provider>
</template>

<style scoped></style>
