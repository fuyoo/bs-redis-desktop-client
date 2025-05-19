<script lang="ts" setup>
import { useRoute } from 'vue-router'

const route = useRoute()
import CoKeys from '@/pages/host/components/CoKeys/index.vue'


</script>
<template>
  <n-split direction="horizontal"  :resize-trigger-size="1" max="600px"
           min="260px"
           default-size="260px">
    <template #1>
      <CoKeys></CoKeys>
    </template>
    <template #2>
      <router-view :key="route.path"></router-view>
    </template>
  </n-split>
</template>
<style scoped lang="scss">
</style>
