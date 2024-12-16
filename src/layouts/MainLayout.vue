<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated>
      <q-toolbar data-tauri-drag-region>
        <span class="mt-4 ml--0.5 flex-shrink-0 text-#fff9" :class="{ 'hide': !$q.platform.is.mac }">BS RDC</span>
        <q-btn class="ml-5" flat dense round icon="home" to="/"></q-btn>
        <q-btn class="ml-5" flat dense round icon="info"
          @click="open(`https://github.com/fuyoo/bs-redis-desktop-client`)"></q-btn>
        <CoHostTab />
        <q-space> </q-space>
        <q-btn class="mr-5" :class="{ 'mr0': $q.platform.is.mac }" flat dense round to="/settings" icon="tune" />
        <WindowOperationButtonGroup v-if="!$q.platform.is.mac"></WindowOperationButtonGroup>
      </q-toolbar>
    </q-header>

    <q-page-container>
      <q-scroll-area class="w-full h-[calc(100vh-50px)]">
        <router-view />
      </q-scroll-area>
    </q-page-container>
  </q-layout>
</template>

<script setup lang="ts">
import WindowOperationButtonGroup from 'src/components/WindowOperationButtonGroup.vue'
import CoHostTab from 'src/components/CoHostTab.vue'
import { open } from '@tauri-apps/plugin-shell'
import { useQuasar } from 'quasar';
const $q = useQuasar()
</script>

<style scoped lang="scss">
.mr0 {
  margin-right: 0;
}

.hide {
  display: none;
}
</style>
