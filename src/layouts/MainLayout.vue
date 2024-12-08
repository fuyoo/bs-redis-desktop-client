<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated>
      <q-toolbar>
        <q-btn flat dense round icon="menu" aria-label="Menu" @click="toggleLeftDrawer" />
        <q-toolbar-title data-tauri-drag-region> BS </q-toolbar-title>
        <div>v{{ $q.version }}</div>
      </q-toolbar>
    </q-header>

    <q-drawer v-model="leftDrawerOpen" show-if-above bordered>
      <q-list>
        <q-item-label header> Operation </q-item-label>
        <EssentialLink
          @click="jumpTo(link.link)"
          v-for="link in linksList"
          :key="link.title"
          v-bind="link"
        />
      </q-list>
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import EssentialLink, { type EssentialLinkProps } from 'components/EssentialLink.vue'
import { open } from '@tauri-apps/plugin-shell'
const linksList: EssentialLinkProps[] = [
  {
    title: 'Host',
    caption: 'redis database host',
    icon: 'school',
    link: 'https://redis.io',
  },
  {
    title: 'Github',
    caption: 'github.com/fuyoo/bs',
    icon: 'code',
    link: 'https://github.com/fuyoo/bs-redis-desktop',
  },
  {
    title: 'Settings',
    caption: 'app setting',
    icon: 'settings',
  },
]

const leftDrawerOpen = ref(false)
function toggleLeftDrawer() {
  leftDrawerOpen.value = !leftDrawerOpen.value
}
const jumpTo = async (url?: string) => {
  if (url) {
    console.log
  }
}
</script>
