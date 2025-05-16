<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'

const router = useRouter()
const route = useRoute()
const changeTab = async (tab: string) => {
  await router.push({
    path: `/tab/${route.params.id}/main/${tab}`,
  })
  console.log(route.path)
}
const tab = computed(() => {
  if (route.path.includes('database')) {
    return 'database'
  } else {
    return 'info'
  }
})
</script>
<template>
  <div class="flex gap-4 h-40px b-b-1px b-b-solid items-center b-b-#eee">
    <div  class="ml-5 h-full flex items-center"  :class="{ active: tab === 'info' }">
      <q-btn flat round dense @click="changeTab('info')">
        <i class="i-hugeicons:cpu-settings text-5 c"></i>
      </q-btn>
    </div>
    <div class="h-full flex items-center" :class="{ active: tab === 'database' }">
      <q-btn flat round dense @click="changeTab('database')">
        <i class="i-hugeicons:database text-5 c"></i>
      </q-btn>
    </div>
  </div>
  <div class="w-full bg-#eee h-[calc(100%-41px)]">
    <router-view />
  </div>
</template>

<style lang="scss" scoped>
._tab {
  margin: 8px 0;
  height: 50px;
}

.active {
  position: relative;
  &::after {
    content: '';
    position: absolute;
    left: 0;
    bottom: 0;
    height: 2px;
    width: 100%;
    border-radius: 2px;
    background: var(--q-primary);
    color: var(--q-primary);
  }
  .c{
    color: var(--q-primary);
  }
}
</style>
