<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated>
      <q-toolbar data-tauri-drag-region>
        <q-btn class="ml-5" flat dense round icon="home" to="/"></q-btn>
        <q-btn
          class="ml-5"
          flat
          dense
          round
          icon="info"
          @click="open(`https://github.com/fuyoo/bs-redis-desktop-client`)"
        ></q-btn>
        <CoHostTab />
        <q-space> </q-space>
        <q-btn flat dense round to="/settings" icon="tune" class="mr-5" />
        <WindowOperationButtonGroup></WindowOperationButtonGroup>
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
</script>
