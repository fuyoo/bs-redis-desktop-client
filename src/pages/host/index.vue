<script setup lang="ts">
import CoHostInfo from './components/CoHostInfo.vue'
import { ref } from 'vue'
import { useReqStore } from '@/stores/req'
import CoDatabase from './components/CoDatabase/index.vue'
const navTab = ref('status')
const dbsize = ref(16)
const tabs = ref(new Set<number>())
</script>
<template>
  <div>
    <q-toolbar class="text-primary b-b b-b-solid b-b-#eee">
      <q-btn flat round dense @click="navTab = 'status'" class="mr-2">
        <i class="i-hugeicons:cpu-settings text-5"></i>
      </q-btn>
      <q-btn flat round dense @click="navTab = 'data'" class="mr-2">
        <i class="i-material-symbols:database text-5"></i>
      </q-btn>
    </q-toolbar>
    <q-tab-panels v-model="navTab" keep-alive>
      <q-tab-panel name="status" class="q-pa-none">
        <q-scroll-area class="h-[calc(100vh-50px)]">
          <co-host-info />
        </q-scroll-area>
      </q-tab-panel>
      <q-tab-panel name="data" class="q-pa-none">
        <q-scroll-area class="h-[calc(100vh-50px)]">
          <co-database></co-database>
        </q-scroll-area>
      </q-tab-panel>
    </q-tab-panels>
  </div>
</template>

<style lang="scss" scoped>
._tab {
  margin: 8px 0;
  height: 50px;
}

._db-active {
  background: #ebebeb;
  width: 70px;
  border-radius: 8px;
  height: 50px;
  padding: 0 8px;
  color: #1976d2;
}
</style>
