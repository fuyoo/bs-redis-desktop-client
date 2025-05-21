<script lang="ts" setup>
import { useRoute } from 'vue-router'
import CoKeys from '@/pages/host/components/CoKeys/index.vue'
const route = useRoute()
</script>
<template>
  <n-split direction="horizontal"  :resize-trigger-size="1" max="600px"
           min="260px"
           default-size="260px">
    <template #1>
      <CoKeys></CoKeys>
    </template>
    <template #2>
      <n-modal-provider>
      <router-view :key="route.path"></router-view>
      </n-modal-provider>
    </template>
  </n-split>
</template>
<style scoped lang="scss">
</style>
