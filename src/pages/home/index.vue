<template>
  <q-page class="p-4">
    <div class="flex gap-4">
      <div v-for="i in 15" :key="i" @click="changeTab(i)"
        class="relative px-4 h-15 shadow-lg b b-solid b-#0001 flex justify-center items-center rounded cursor-pointer active:opacity-50">
        <div class="bg-#000 text-white rounded flex justify-center items-center text-5 w-8 h-8 mr-3">
          <i class="i-mdi:server-network"></i>
        </div>
        <div class="flex-1 flex flex-col mr-2 justify-center items-start select-none">
          <span class="text-4 text-#333">host-{{ i }}</span>
          <span class="text-3 text-#0006">password,cluster</span>
        </div>
      </div>
      <div
        class="w-15 h-15 shadow-lg b b-solid b-#0001 flex justify-center items-center text-10 rounded cursor-pointer active:opacity-50">
        <i class="i-ic:round-plus"></i>
      </div>
    </div>
  </q-page>
</template>

<script setup lang="ts">
import { useTabStore } from '@/stores/tab';


const tab = useTabStore()
const changeTab = (i: number) => {
  tab.change({ id: `host-${i}`, name: `host-${i}` })
}
</script>
